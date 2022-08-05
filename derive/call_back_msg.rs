use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DataEnum, DeriveInput, Variant};

pub fn expanded(input: DeriveInput) -> TokenStream {
    let enum_name @ _ = &input.ident;

    let variants = match &input.data {
        Data::Enum(DataEnum { variants: it, .. }) => it,
        _ => unreachable!(),
    };
    let each_physical_variant = variants.iter().map(
        |Variant {
             ident: variant_name @ _,
             ..
         }| variant_name,
    );

    let is_top_msg = variants.iter().find(|x| {
        let fields = x.fields.iter().collect::<Vec<_>>();

        if fields.len() != 1 || fields[0].ident.is_some() {
            panic!("UnzipConsensus only supports 1-tuple variants");
        } else {
            // let f = format!("{:?}", fields);
            // if f.contains("Msg") {
            //     panic!("{:?}", fields);
            // }
        }

        let f = format!("{:?}", fields);
        f.contains("Msg")
        // syn::Ident::new(&f, x.span())
    });
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
    // .find(|x| x.contains("Msg"))
    // .is_some();
    let each_physical_variant_copy = each_physical_variant.clone();

    let expanded = if is_top_msg.is_some() {
        quote! {
                use yew::html::Scope;
                use yew::Callback;
                use yew::Component;
                use crate::CallbackMsg;
                impl CallbackMsg for #enum_name { // <- Assumes there being no generics.
                    type O = #page::Msg;

                    fn as_callback<M, T, C>(&self, link: &Scope<C>) -> Box<Callback<String>>
                    where
                        C: Component + 'static,
                        M: Into<C::Message>,
                        T: SettingCallbackFn<M> + 'static,
                        <C as yew::Component>::Message: From<Self::O>,
                    {
                        match self {
                        #(
                            | Self::#each_physical_variant_copy ( x ) => {
                                x.as_callback::<M,T,C>(link)
                            },
                        )*
                        }
                    }
                }
        }
    } else {
        quote! {
            use yew::html::Scope;
            use yew::Callback;
            use yew::Component;
            use crate::CallbackMsg;
            impl CallbackMsg for #enum_name { // <- Assumes there being no generics.
                type O = #page::Msg;

                fn as_callback<M, T, C>(&self, link: &Scope<C>) -> Box<Callback<String>>
                where
                    C: Component + 'static,
                    M: Into<C::Message>,
                    T: SettingCallbackFn<M> + 'static,
                    <C as yew::Component>::Message: From<Self::O>,
                {
                    match self {
                    #(
                        | Self::#each_physical_variant_copy ( _ ) => { Self::to_callback_fn(
                                |x| Self::#each_physical_variant_copy (x.parse().ok()),
                                link
                            )
                        },
                    )*
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}
