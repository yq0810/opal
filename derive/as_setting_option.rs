use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DataStruct, DeriveInput, Field};

pub fn expanded(input: DeriveInput) -> TokenStream {
    let struct_name @ _ = &input.ident;

    let variants = match &input.data {
        Data::Struct(DataStruct { fields: it, .. }) => it,
        _ => unreachable!(),
    };
    let each_physical_variant = variants.iter().map(
        |Field {
             ident: variant_name,
             ..
         }| variant_name,
    );
    // debug!("{:#?}", input);

    let page_word = &input
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("page"))
        .map(|x| {
            let f = format!("{}", x.tokens)
                .replace("\"", "")
                .replace("(", "")
                .replace(")", "");
            syn::Ident::new(&f, x.span())
        })
        .collect::<Vec<_>>()
        .first()
        .map(|x| x.clone())
        .unwrap();
    let page = quote! { components::#page_word };

    let config = struct_name;
    let expanded = quote! {
        use crate::TotalMsgScope;
        use crate::SettingOption;
        use crate::AsSettingOption;
        use crate::traits::as_input_type::AsInputType;

        impl AsSettingOption for Msgs {
            type O = #page::Msg;
            type Config = #config;

            fn get_options<M, T, C>(config: &Self::Config, link: TotalMsgScope) -> Vec<SettingOption>
            where
                C: Component + 'static,
                M: Into<C::Message>,
                T: SettingCallbackFn<M> + 'static,
                <C as yew::Component>::Message: From<Self::O>,
            {
                let l = vec![
                #(
                    config.#each_physical_variant.input_type(),
                )*
                ];
                l.iter()
                    .map(|input_type| -> SettingOption {
                        Self::option_input_data::<M, T, C>(input_type, &link)
                    })
                    .collect()
            }
        }
    };

    TokenStream::from(expanded)
}
