use proc_macro2::{Ident, Span};
use quote::quote;

#[proc_macro]
pub fn derive_crud_requests(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ident = syn::parse::<Ident>(input).expect("failed to parse derive input");

    let get = Ident::new(&format!("Get{}", ident), Span::call_site());
    let list = Ident::new(&format!("List{}s", ident), Span::call_site());
    let create = Ident::new(&format!("Create{}", ident), Span::call_site());
    let delete = Ident::new(&format!("Delete{}", ident), Span::call_site());
    let update = Ident::new(&format!("Update{}", ident), Span::call_site());
    let partial_config = Ident::new(&format!("Partial{}Config", ident), Span::call_site());

    quote! {
        #[typeshare::typeshare]
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, resolver_api::derive::Request)]
        #[response(#ident)]
        pub struct #get {
            pub id: String,
        }

        #[typeshare::typeshare]
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, resolver_api::derive::Request)]
        #[response(Vec<#ident>)]
        pub struct #list {
            pub query: Option<MongoDocument>,
        }

        //

        #[typeshare::typeshare]
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, resolver_api::derive::Request)]
        #[response(#ident)]
        pub struct #create {
            pub name: String,
            pub config: #partial_config,
        }

        //

        #[typeshare::typeshare]
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, resolver_api::derive::Request)]
        #[response(#ident)]
        pub struct #delete {
            pub id: String,
        }

        //

        #[typeshare::typeshare]
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, resolver_api::derive::Request)]
        #[response(#ident)]
        pub struct #update {
            pub id: String,
            pub config: #partial_config,
        }
    }
    .into()
}

