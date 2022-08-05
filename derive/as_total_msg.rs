use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{DeriveInput, Ident};

pub fn expanded(input: DeriveInput) -> TokenStream {
    let enum_name @ _ = &input.ident;

    let msg_name = enum_name.to_string().replace("Msg", "");
    let msg_name_i = Ident::new(&msg_name, enum_name.span());
    // debug!("{:#?}", input);

    let total_msg_name = &input
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("totalMsgName"))
        .map(|x| {
            let f = format!("{}Msg", x.tokens)
                .replace("\"", "")
                .replace("(", "")
                .replace(")", "");
            syn::Ident::new(&f, x.span())
        })
        .collect::<Vec<_>>()
        .first()
        .map(|x| x.clone())
        .unwrap();

    let expanded = quote! {
        use crate::TotalMsg;
        impl AsTotalMsg for #enum_name { // <- Assumes there being no generics.
            fn to_total_msg (self: &'_ Self)
              -> TotalMsg
            {
                TotalMsg::#total_msg_name(Msgs::#msg_name_i(self.clone()))
            }

        }
    };

    TokenStream::from(expanded)
}
