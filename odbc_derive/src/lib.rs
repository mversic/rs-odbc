use proc_macro2::{Ident, Span};
use proc_macro::{TokenStream};
use quote::quote;
use syn;

#[proc_macro_derive(EnvAttribute, attributes(identifier))]
pub fn env_attribute_derive(input: TokenStream) -> TokenStream {
    into_attr(input, Ident::new("EnvAttribute", Span::call_site()), Ident::new("SQLINTEGER", Span::call_site()))
}

#[proc_macro_derive(ConnAttribute, attributes(identifier))]
pub fn conn_attribute_derive(input: TokenStream) -> TokenStream {
    into_attr(input, Ident::new("ConnAttribute", Span::call_site()), Ident::new("SQLINTEGER", Span::call_site()))
}

#[proc_macro_derive(StmtAttribute, attributes(identifier))]
pub fn stmt_attribute_derive(input: TokenStream) -> TokenStream {
    into_attr(input, Ident::new("StmtAttribute", Span::call_site()), Ident::new("SQLINTEGER", Span::call_site()))
}

#[proc_macro_derive(ColAttribute, attributes(identifier))]
pub fn col_attribute_derive(input: TokenStream) -> TokenStream {
    into_attr(input, Ident::new("ColAttribute", Span::call_site()), Ident::new("SQLUSMALLINT", Span::call_site()))
}

#[proc_macro_derive(AnsiType)]
pub fn ansi_type_derive(input: TokenStream) -> TokenStream {
    into_type(input, Ident::new("AnsiType", Span::call_site()))
}

#[proc_macro_derive(UnicodeType)]
pub fn unicode_type_derive(input: TokenStream) -> TokenStream {
    into_type(input, Ident::new("UnicodeType", Span::call_site()))
}

fn into_attr(input: TokenStream, attr_name: Ident, identifier_type: Ident) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let type_name = &ast.ident;

    let mut identifier = None;
    for attr in ast.attrs.into_iter() {
        if attr.path.is_ident("identifier") {
            identifier = Some(attr.parse_args::<syn::Lit>().expect(
                "Attribute #[identifier(<num>)] missing"
            ));
        }
    }

    let gen = quote! {
        impl #impl_generics Attribute for #type_name #ty_generics #where_clause {
            type AttributeType = OdbcAttribute;
            type IdentifierType = #identifier_type;

            fn identifier() -> Self::IdentifierType {
                #identifier
            }
        }

        impl #impl_generics #attr_name for #type_name #ty_generics #where_clause {}
    };

    gen.into()
}

fn into_type(input: TokenStream, trait_name: Ident) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let type_name = &ast.ident;

    let gen = quote! {
        impl #impl_generics #trait_name for #type_name #ty_generics #where_clause {}
    };

    gen.into()
}

