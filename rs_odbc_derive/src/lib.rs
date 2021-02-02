use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn;

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

#[allow(non_snake_case)]
#[proc_macro_derive(EqSQLUINTEGER)]
pub fn EqSQLUINTEGER_derive(input: TokenStream) -> TokenStream {
    type_eq(input, Ident::new("SQLUINTEGER", Span::call_site()))
}

#[allow(non_snake_case)]
#[proc_macro_derive(EqSQLSMALLINT)]
pub fn EqSQLSMALLINT_derive(input: TokenStream) -> TokenStream {
    type_eq(input, Ident::new("SQLSMALLINT", Span::call_site()))
}

#[allow(non_snake_case)]
#[proc_macro_derive(EqSQLULEN)]
pub fn EqSQLULEN_derive(input: TokenStream) -> TokenStream {
    type_eq(input, Ident::new("SQLULEN", Span::call_site()))
}

#[proc_macro_derive(SqlType)]
pub fn sql_type_derive(input: TokenStream) -> TokenStream {
    empty_trait_derive(input, Ident::new("SqlType", Span::call_site()))
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
            if let syn::Meta::List(attr_list) = attr.parse_meta().expect("KURAC") {
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
            type AttrType = crate::OdbcAttr;
        }
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
