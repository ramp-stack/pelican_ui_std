use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AppPage)]
pub fn derive_app_page(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl AppPage for #name {}

        impl #name {
            pub fn navigate(ctx: &mut Context) {
                let page = #name::new(ctx);
                let has_nav = page.2;
                ctx.trigger_event(NavigateEvent(Some(page.into_boxed()), has_nav));
            }
        }        
    };

    TokenStream::from(expanded)
}
