use proc_macro::{token_stream, TokenStream};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{self, parse::Parse, parse::Parser};

// TODO: Better message
const ZST_MSG: &str = "`odbc_type` must be implemented on a zero-sized struct or an enum";

#[proc_macro_derive(Ident, attributes(identifier))]
pub fn into_identifier(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let type_name = &ast.ident;

    let mut identifier = None;
    let mut identifier_type = None;
    for attr in ast.attrs.into_iter() {
        if attr.path.is_ident("identifier") {
            if let syn::Meta::List(attr_list) = attr.parse_meta().expect("Missing arguments") {
                let mut attr_list = attr_list.nested.into_iter();

                if let syn::NestedMeta::Meta(meta) = &attr_list.next().expect("Missing arguments") {
                    identifier_type = meta.path().get_ident().map(|x| x.to_owned());
                } else {
                    panic!("1st argument is not a valid ODBC type");
                }
                if let syn::NestedMeta::Lit(lit) = attr_list.next().expect("Missing 2nd argument") {
                    identifier = Some(lit);
                } else {
                    panic!("2nd argument is not a valid literal");
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

fn parse_inner_type(mut args: token_stream::IntoIter) -> Ident {
    let inner_type: Ident = syn::parse(args.next().unwrap().into()).unwrap();

    if args.next().is_some() {
        // TODO: Better message
        panic!("Only one ODBC type can be declared");
    }

    match inner_type.to_string().as_str() {
        // TODO: Expand with other types. Maybe use Copy trait. Then implement Ident only if inner_type implements inner
        // REQUIREMENT1: supported types must have a valid zero-byte representation because of AttrZeroFill
        // REQUIREMENT2: supported types must have the same representation as SQLPOINTER because of Ident
        "SQLINTEGER" | "SQLUINTEGER" | "SQLSMALLINT" | "SQLUSMALLINT" | "SQLLEN" | "SQLULEN" => {}
        unsupported => panic!("{}: unsupported ODBC type", unsupported),
    }

    inner_type
}

fn odbc_derive(ast: &mut syn::DeriveInput, inner_type: &Ident) -> TokenStream2 {
    ast.attrs.extend(
        syn::Attribute::parse_outer
            .parse2(quote! { #[derive(Debug, Clone, Copy)] })
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
                panic!("{}", ZST_MSG);
            }

            quote! {
                unsafe impl crate::convert::AsMutSQLPOINTER for #type_name {
                    fn as_mut_SQLPOINTER(&mut self) -> crate::SQLPOINTER {
                        (self as *mut Self).cast()
                    }
                }
                unsafe impl crate::convert::AsMutSQLPOINTER for core::mem::MaybeUninit<#type_name> {
                    fn as_mut_SQLPOINTER(&mut self) -> crate::SQLPOINTER {
                        self.as_mut_ptr().cast()
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
                impl TryFrom<crate::#inner_type> for #type_name {
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
        _ => panic!("{}", ZST_MSG),
    };

    ret.extend(quote! {
        impl crate::Ident for #type_name where crate::#inner_type: crate::Ident {
            type Type = <crate::#inner_type as crate::Ident>::Type;
            const IDENTIFIER: Self::Type = <crate::#inner_type as crate::Ident>::IDENTIFIER;
        }

        impl crate::Scalar for #type_name where crate::#inner_type: crate::Scalar {}

        unsafe impl crate::convert::IntoSQLPOINTER for #type_name {
            fn into_SQLPOINTER(self) -> crate::SQLPOINTER {
                Self::identifier(&self) as _
            }
        }

        impl crate::attr::AttrZeroAssert for #type_name {
            #[inline]
            fn assert_zeroed(&self) {
                // TODO: Check implementation on types in lib.rs
                assert_eq!(0, Self::identifier(&self));
            }
        }

        #ast
    });

    ret
}

#[proc_macro_attribute]
pub fn odbc_bitmask(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast: syn::DeriveInput = syn::parse(input).unwrap();

    let inner_type = parse_inner_type(args.into_iter());
    let mut odbc_bitmask = odbc_derive(&mut ast, &inner_type);

    let type_name = &ast.ident;
    odbc_bitmask.extend(quote! {
        impl core::ops::BitAnd<#type_name> for #type_name {
            type Output = crate::#inner_type;

            fn bitand(self, other: #type_name) -> Self::Output {
                Self::identifier(&self) &Self::identifier(&other)
            }
        }
        impl core::ops::BitAnd<crate::#inner_type> for #type_name {
            type Output = crate::#inner_type;

            fn bitand(self, other: crate::#inner_type) -> Self::Output {
                Self::identifier(&self) & other
            }
        }
        impl core::ops::BitAnd<#type_name> for crate::#inner_type {
            type Output = crate::#inner_type;

            fn bitand(self, other: #type_name) -> Self::Output {
                other & self
            }
        }
    });

    odbc_bitmask.into()
}

#[proc_macro_attribute]
pub fn odbc_type(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast: syn::DeriveInput = syn::parse(input).unwrap();

    ast.attrs.extend(
        syn::Attribute::parse_outer
            .parse2(quote! { #[derive(PartialEq, Eq)] })
            .unwrap(),
    );

    let inner_type = parse_inner_type(args.into_iter());
    let mut odbc_type = odbc_derive(&mut ast, &inner_type);

    let type_name = &ast.ident;
    odbc_type.extend(quote! {
        impl PartialEq<crate::#inner_type> for #type_name {
            fn eq(&self, other: &crate::#inner_type) -> bool {
                self.identifier() == *other
            }
        }

        impl PartialEq<#type_name> for crate::#inner_type {
            fn eq(&self, other: &#type_name) -> bool {
                other == self
            }
        }
    });

    odbc_type.into()
}
