use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(EnvAttribute, attributes(identifier))]
pub fn conn_attribute_derive(input: TokenStream) -> TokenStream {
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
            type IdentifierType = SQLINTEGER;

            fn identifier() -> Self::IdentifierType {
                #identifier
            }
        }

        impl #impl_generics EnvAttribute for #type_name #ty_generics #where_clause {}
    };

    gen.into()
}

#[proc_macro_derive(AnsiType)]
pub fn ansi_type_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let type_name = &ast.ident;

    let gen = quote! {
        impl #impl_generics AnsiType for #type_name #ty_generics #where_clause {}
    };

    gen.into()
}

