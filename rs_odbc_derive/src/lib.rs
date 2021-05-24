use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{self, parse::Parse, parse::Parser};

#[proc_macro_derive(Ident, attributes(identifier))]
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
        impl #impl_generics crate::Ident for #type_name #ty_generics #where_clause {
            type Type = crate::#identifier_type;
            const IDENTIFIER: Self::Type = #identifier;
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn odbc_type(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast: syn::DeriveInput = syn::parse(input).unwrap();
    let mut args = args.into_iter();
    // TODO:
    let inner_type: Ident = syn::parse(args.next().unwrap().into()).expect("KAR");

    match inner_type.to_string().as_str() {
        // REQUIREMENT1: supported types must have a valid zero-byte representation because of AttrZeroFill
        // REQUIREMENT2: supported types must have the same representation as SQLPOINTER because of Ident
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
                        .parse2(quote! { (crate::#inner_type) })
                        .expect(&format!("{}: unknown ODBC type", inner_type)),
                );
            } else {
                panic!("`odbc_type` can only be implemente for ZST");
            }

            quote! {
                #ast

                unsafe impl crate::IntoSQLPOINTER for #type_name {
                    fn into_SQLPOINTER(self) -> crate::SQLPOINTER {
                        self.0 as _
                    }
                }
                unsafe impl crate::AsMutSQLPOINTER for #type_name {
                    fn as_mut_SQLPOINTER(&mut self) -> crate::SQLPOINTER {
                        (self as *mut Self).cast()
                    }
                }
                unsafe impl crate::AsMutSQLPOINTER for std::mem::MaybeUninit<#type_name> {
                    fn as_mut_SQLPOINTER(&mut self) -> crate::SQLPOINTER {
                        self.as_mut_ptr().cast()
                    }
                }

                impl PartialEq<crate::#inner_type> for #type_name {
                    fn eq(&self, other: &crate::#inner_type) -> bool {
                        self.0 == *other
                    }
                }

                impl #type_name {
                    #[inline]
                    pub(crate) const fn identifier(&self) -> crate::#inner_type {
                        self.0
                    }
                }
            }
        }
        syn::Data::Enum(ref data) => {
            let variants = data.variants.iter().map(|v| &v.ident);

            quote! {
                #ast

                unsafe impl crate::IntoSQLPOINTER for #type_name {
                    fn into_SQLPOINTER(self) -> crate::SQLPOINTER {
                        self as crate::#inner_type as _
                    }
                }
                impl PartialEq<crate::#inner_type> for #type_name {
                    fn eq(&self, other: &crate::#inner_type) -> bool {
                        *self as crate::#inner_type == *other
                    }
                }

                impl std::convert::TryFrom<crate::#inner_type> for #type_name {
                    type Error = crate::#inner_type;

                    fn try_from(source: crate::#inner_type) -> Result<Self, Self::Error> {
                        match source {
                            #(x if x == #type_name::#variants as crate::#inner_type => Ok(#type_name::#variants)),*,
                            unknown => Err(unknown),
                        }
                    }
                }

                impl #type_name {
                    pub(crate) const fn identifier(&self) -> crate::#inner_type {
                        *self as crate::#inner_type
                    }
                }
            }
        }
        _ => panic!("`odbc_type` can only be implemented for ZST structs or enums"),
    };

    ret.extend(quote! {
        impl crate::Ident for #type_name {
            type Type = <crate::#inner_type as crate::Ident>::Type;
            const IDENTIFIER: Self::Type = <crate::#inner_type as crate::Ident>::IDENTIFIER;
        }

        impl crate::AttrZeroAssert for #type_name {
            #[inline]
            fn assert_zeroed(&self) {
                // TODO: Check implementation on types in lib.rs
                assert_eq!(0, *self);
            }
        }

        impl PartialEq<#type_name> for crate::#inner_type {
            fn eq(&self, other: &#type_name) -> bool {
                other == self
            }
        }
    });

    ret.into()
}
