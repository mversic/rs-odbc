use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{self, parse::Parse, parse::Parser};

#[proc_macro_derive(EnvAttr)]
pub fn env_attr_derive(input: TokenStream) -> TokenStream {
    empty_trait_derive(input, Ident::new("EnvAttr", Span::call_site()))
}

#[proc_macro_derive(ConnAttr)]
pub fn conn_attr_derive(input: TokenStream) -> TokenStream {
    attr_derive(input, Ident::new("ConnAttr", Span::call_site()))
}

#[proc_macro_derive(StmtAttr)]
pub fn stmt_attr_derive(input: TokenStream) -> TokenStream {
    attr_derive(input, Ident::new("StmtAttr", Span::call_site()))
}

#[proc_macro_derive(ColAttr)]
pub fn col_attr_derive(input: TokenStream) -> TokenStream {
    attr_derive(input, Ident::new("ColAttr", Span::call_site()))
}

#[proc_macro_derive(InfoType)]
pub fn info_type_derive(input: TokenStream) -> TokenStream {
    attr_derive(input, Ident::new("InfoType", Span::call_site()))
}

#[proc_macro_derive(DiagField)]
pub fn diag_field_derive(input: TokenStream) -> TokenStream {
    attr_derive(input, Ident::new("DiagField", Span::call_site()))
}

#[proc_macro_derive(CType)]
pub fn c_type_derive(input: TokenStream) -> TokenStream {
    empty_trait_derive(input, Ident::new("CType", Span::call_site()))
}

#[proc_macro_derive(Identifier, attributes(identifier))]
pub fn into_identifier(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let type_name = &ast.ident;

    let mut identifier = None;
    let mut identifier_type = None;
    for attr in ast.attrs.into_iter() {
        if attr.path.is_ident("identifier") {
            if let syn::Meta::List(attr_list) = attr.parse_meta().unwrap() {
                let mut attr_list = attr_list.nested.into_iter();

                if let syn::NestedMeta::Meta(ref meta) = attr_list.next().unwrap() {
                    identifier_type = meta.path().get_ident().map(|x| x.to_owned());
                } else {
                    panic!("1st item not type");
                }
                if let syn::NestedMeta::Lit(lit) = attr_list.next().unwrap() {
                    identifier = Some(lit);
                } else {
                    panic!("2nd item not literal");
                }
            }
        }
    }

    let gen = quote! {
        impl #impl_generics crate::Identifier for #type_name #ty_generics #where_clause {
            type IdentType = crate::#identifier_type;
            const IDENTIFIER: Self::IdentType = #identifier;
        }
    };

    gen.into()
}

fn empty_trait_derive(input: TokenStream, attr_name: Ident) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let type_name = &ast.ident;

    let gen = quote! {
        impl #impl_generics #attr_name for #type_name #ty_generics #where_clause {}
    };

    gen.into()
}

fn attr_derive(input: TokenStream, attr_name: Ident) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let type_name = &ast.ident;

    let gen = quote! {
        impl #impl_generics #attr_name for #type_name #ty_generics #where_clause {
            // TODO: try not to hard code type
            type AttrType = crate::OdbcAttr;
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn odbc_type(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast: syn::DeriveInput = syn::parse(input).unwrap();
    let mut args = args.into_iter();
    let inner_type: Ident = syn::parse(args.next().unwrap().into()).expect("KAR");

    match inner_type.to_string().as_str() {
        "SQLINTEGER" | "SQLUINTEGER" | "SQLSMALLINT" | "SQLUSMALLINT" | "SQLLEN" | "SQLULEN" => {}
        unsupported => panic!("{}: unsupported ODBC type", unsupported),
    }

    if args.next().is_some() {
        println!("`odbc type` can only declare one type");
    }

    ast.attrs.extend(
        syn::Attribute::parse_outer
            .parse2(quote! {
                #[derive(Debug, PartialEq, Eq, Clone, Copy)]
            })
            .unwrap(),
    );

    let type_name = &ast.ident;
    let mut ret = match ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            ast.attrs.extend(
                syn::Attribute::parse_outer
                    .parse2(quote! { #[repr(transparent)] })
                    .unwrap(),
            );

            if struct_data.fields.is_empty() {
                struct_data.fields = syn::Fields::Unnamed(
                    syn::FieldsUnnamed::parse
                        .parse2(quote! { (#inner_type) })
                        .expect(&format!("{}: unknown ODBC type", inner_type)),
                );
            } else {
                panic!("`odbc_type` can only be implemente for ZST");
            }

            quote! {
                #ast

                unsafe impl crate::AsSQLPOINTER for #type_name {
                    fn as_SQLPOINTER(&self) -> crate::SQLPOINTER {
                        self.0 as crate::SQLPOINTER
                    }
                }
                unsafe impl crate::AsMutSQLPOINTER for #type_name {
                    fn as_mut_SQLPOINTER(&mut self) -> crate::SQLPOINTER {
                        self as *mut _ as crate::SQLPOINTER
                    }
                }
                unsafe impl crate::AsMutSQLPOINTER for std::mem::MaybeUninit<#type_name> {
                    fn as_mut_SQLPOINTER(&mut self) -> crate::SQLPOINTER {
                        self.as_mut_ptr().cast()
                    }
                }

                unsafe impl<LEN: Copy> crate::Len<crate::OdbcAttr, LEN> for std::mem::MaybeUninit<#type_name> where LEN: From<crate::SQLSMALLINT> {
                    type StrLen = ();

                    fn len(&self) -> LEN {
                        LEN::from(<#inner_type as crate::Identifier>::IDENTIFIER)
                    }
                }
                unsafe impl<LEN: Copy> crate::Len<crate::DriverAttr, LEN> for std::mem::MaybeUninit<#type_name> where LEN: From<crate::SQLSMALLINT> {
                    type StrLen = ();

                    fn len(&self) -> LEN {
                        LEN::from(<#inner_type as crate::Identifier>::IDENTIFIER)
                    }
                }

                impl PartialEq<#inner_type> for #type_name {
                    fn eq(&self, other: &#inner_type) -> bool {
                        self.0 == *other
                    }
                }

                impl #type_name {
                    #[inline]
                    pub(crate) const fn identifier(&self) -> #inner_type {
                        self.0
                    }
                }
            }
        }
        syn::Data::Enum(ref data) => {
            let variants = data.variants.iter().map(|v| &v.ident);

            quote! {
                #ast

                unsafe impl crate::AsSQLPOINTER for #type_name {
                    fn as_SQLPOINTER(&self) -> crate::SQLPOINTER {
                        *self as #inner_type as crate::SQLPOINTER
                    }
                }
                impl PartialEq<#inner_type> for #type_name {
                    fn eq(&self, other: &#inner_type) -> bool {
                        *self as #inner_type == *other
                    }
                }

                impl std::convert::TryFrom<#inner_type> for #type_name {
                    type Error = #inner_type;

                    fn try_from(source: #inner_type) -> Result<Self, Self::Error> {
                        match source {
                            #(x if x == #type_name::#variants as #inner_type => Ok(#type_name::#variants)),*,
                            unknown => Err(unknown),
                        }
                    }
                }

                impl #type_name {
                    pub(crate) const fn identifier(&self) -> #inner_type {
                        *self as #inner_type
                    }
                }
            }
        }
        _ => panic!("`odbc_type` can only be implemented for ZST structs or enums"),
    };

    ret.extend(quote! {
        unsafe impl<LEN: Copy> crate::Len<crate::OdbcAttr, LEN> for #type_name where LEN: From<crate::SQLSMALLINT> {
            type StrLen = ();

            fn len(&self) -> LEN {
                LEN::from(<#inner_type as crate::Identifier>::IDENTIFIER)
            }
        }
        unsafe impl<LEN: Copy> crate::Len<crate::DriverAttr, LEN> for #type_name where LEN: From<crate::SQLSMALLINT> {
            type StrLen = ();

            fn len(&self) -> LEN {
                LEN::from(<#inner_type as crate::Identifier>::IDENTIFIER)
            }
        }

        impl PartialEq<#type_name> for #inner_type {
            fn eq(&self, other: &#type_name) -> bool {
                other == self
            }
        }
    });

    ret.into()
}
