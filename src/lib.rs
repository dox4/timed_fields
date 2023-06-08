use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{self, Parse, Parser},
    parse_macro_input, ItemStruct, Path,
};

#[derive(Default)]
struct Args {
    no_updated_at: bool,
    no_created_at: bool,
    no_deleted_at: bool,
}

impl Parse for Args {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let mut args = Args::default();
        let vars = syn::punctuated::Punctuated::<Path, syn::Token![,]>::parse_terminated(input)?;
        for var in vars.iter() {
            if var.is_ident("no_created_at") {
                args.no_created_at = true;
            } else if var.is_ident("no_updated_at") {
                args.no_updated_at = true;
            } else if var.is_ident("no_deleted_at") {
                args.no_deleted_at = true;
            }
        }
        Ok(args)
    }
}

#[proc_macro_attribute]
pub fn add_timed_fields(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut is = parse_macro_input!(input as ItemStruct);
    let args = parse_macro_input!(attr as Args);
    if let syn::Fields::Named(ref mut fields) = is.fields {
        let mut timed_fields = vec![];
        if !args.no_created_at {
            timed_fields.push(
                quote! { #[ignore_when(insert, update)] pub created_at: Option<DateTime<Local>> },
            );
        }
        if !args.no_updated_at {
            timed_fields.push(
                quote! { #[ignore_when(insert, update)] pub updated_at: Option<DateTime<Local>> },
            );
        }
        if !args.no_deleted_at {
            timed_fields.push(quote! {
                #[serde(skip_serializing_if = "Option::is_none")]
                #[ignore_when(insert, update)]
                pub deleted_at: Option<DateTime<Local>>
            });
        }
        for field in timed_fields {
            fields
                .named
                .push(syn::Field::parse_named.parse2(field).unwrap());
        }
    }
    quote! {
        #is
    }
    .into()
}
