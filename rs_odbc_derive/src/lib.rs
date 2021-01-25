use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn;

#[proc_macro_derive(EnvAttr, attributes(identifier))]
pub fn env_attr_derive(input: TokenStream) -> TokenStream {
    into_attr(
        input,
        Ident::new("EnvAttr", Span::call_site()),
        Ident::new("SQLINTEGER", Span::call_site()),
    )
}

#[proc_macro_derive(ConnAttr, attributes(identifier))]
pub fn conn_attr_derive(input: TokenStream) -> TokenStream {
    into_attr(
        input,
        Ident::new("ConnAttr", Span::call_site()),
        Ident::new("SQLINTEGER", Span::call_site()),
    )
}

#[proc_macro_derive(StmtAttr, attributes(identifier))]
pub fn stmt_attr_derive(input: TokenStream) -> TokenStream {
    into_attr(
        input,
        Ident::new("StmtAttr", Span::call_site()),
        Ident::new("SQLINTEGER", Span::call_site()),
    )
}

#[proc_macro_derive(ColAttr, attributes(identifier))]
pub fn col_attr_derive(input: TokenStream) -> TokenStream {
    into_attr(
        input,
        Ident::new("ColAttr", Span::call_site()),
        Ident::new("SQLUSMALLINT", Span::call_site()),
    )
}

#[allow(non_snake_case)]
#[proc_macro_derive(EqSQLUINTEGER)]
pub fn EqSQLUINTEGER_derive(input: TokenStream) -> TokenStream {
    type_eq(input, Ident::new("SQLUINTEGER", Span::call_site()))
}

#[allow(non_snake_case)]
#[proc_macro_derive(EqSQLULEN)]
pub fn EqSQLULEN(input: TokenStream) -> TokenStream {
    type_eq(input, Ident::new("SQLULEN", Span::call_site()))
}

#[proc_macro_derive(AnsiType)]
pub fn ansi_type_derive(input: TokenStream) -> TokenStream {
    char_type(input, Ident::new("AnsiType", Span::call_site()))
}

#[proc_macro_derive(UnicodeType)]
pub fn unicode_type_derive(input: TokenStream) -> TokenStream {
    char_type(input, Ident::new("UnicodeType", Span::call_site()))
}

fn into_attr(input: TokenStream, attr_name: Ident, identifier_type: Ident) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let type_name = &ast.ident;

    let mut identifier = None;
    for attr in ast.attrs.into_iter() {
        if attr.path.is_ident("identifier") {
            identifier = Some(
                attr.parse_args::<syn::Lit>()
                    .expect("Attribute #[identifier(<num>)] missing"),
            );
        }
    }

    let gen = quote! {
        impl #impl_generics crate::Attribute for #type_name #ty_generics #where_clause {
            type AttrType = crate::OdbcAttr;
            type IdentType = crate::#identifier_type;
            const IDENTIFIER: Self::IdentType = #identifier;
        }

        impl #impl_generics #attr_name for #type_name #ty_generics #where_clause {}
    };

    gen.into()
}

fn char_type(input: TokenStream, trait_name: Ident) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let type_name = &ast.ident;

    let gen = quote! {
        impl #impl_generics crate::#trait_name for #type_name #ty_generics #where_clause {}
    };

    gen.into()
}

fn type_eq(input: TokenStream, into_type_name: Ident) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    match &ast.data {
        syn::Data::Enum(data) => {
            let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
            let variants = data.variants.iter().map(|v| &v.ident);
            let type_name = &ast.ident;

            let gen = quote! {
                impl #impl_generics std::convert::TryFrom<#into_type_name> for #type_name #ty_generics #where_clause {
                    type Error = #into_type_name;

                    fn try_from(source: #into_type_name) -> Result<Self, Self::Error> {
                        match source {
                            #(x if x == #type_name::#variants as #into_type_name => Ok(#type_name::#variants)),*,
                            unknown => Err(unknown),
                        }
                    }
                }
                impl PartialEq<#into_type_name> for #type_name {
                    fn eq(&self, other: &#into_type_name) -> bool {
                        *self as #into_type_name == *other
                    }
                }
                impl PartialEq<#type_name> for #into_type_name {
                    fn eq(&self, other: &#type_name) -> bool {
                        other == self
                    }
                }
            };

            gen.into()
        }
        _ => panic!("Only enums are supported for this derive currently"),
    }
}
