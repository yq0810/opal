use proc_macro::TokenStream;
use syn::DataEnum;
use syn::DataStruct;
use syn::Field;
use syn::Variant;
use syn::{parse_macro_input, DeriveInput};

use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, Ident};

#[proc_macro_derive(Sqlgogo, attributes(result))]
pub fn derive_sqlgogo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name @ _ = &input.ident;

    let variants = match &input.data {
        Data::Enum(DataEnum { variants: it, .. }) => it,
        _ => unreachable!(),
    };
    let each_physical_variant = variants
        .iter()
        // Only keep the `#[physical]`-annotated variants
        // .filter(|variant| {
        //     variant
        //         .attrs
        //         .iter()
        //         .any(|attr| attr.path.is_ident("result"))
        // })
        // From `Foo { … }` get `Foo`
        .map(
            |Variant {
                 ident: variant_name @ _,
                 ..
             }| variant_name,
        );

    let each_type = variants.iter().map(
        |Variant {
             ident: variant_name @ _,
             ..
         }| {
            let new_type_name = format!(
                "{}Result",
                variant_name
                    .to_string()
                    .split("By")
                    .collect::<Vec<_>>()
                    .first()
                    .unwrap()
            );
            Ident::new(&new_type_name, variant_name.span())
        },
    );

    let expanded = quote! {
        impl Entrys for #enum_name { // <- Assumes there being no generics.
            fn entrys<T> (self: &'_ Self, js: JsValue)
              -> Vec<T> where for<'a> T: Deserialize<'a>
            {
                match *self {
                #(
                    | Self::#each_physical_variant { .. } => {
                        #each_type::from_entrys(js)
                    },
                )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ValueOPMacro)]
pub fn derive_value_op(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
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
    });
    let each_physical_variant_copy = each_physical_variant.clone();
    let expanded = if is_top_msg.is_some() {
        quote! {
            impl ValueOP for #enum_name { // <- Assumes there being no generics.
                fn get_value (self: &'_ Self)
                -> String
                {
                    match self {
                    #(
                        | Self::#each_physical_variant ( x ) => {
                            x.get_value()
                        },
                    )*
                    }
                }

                fn set_value (self: &'_ Self, new_value: String)
                -> Self
                {
                    match self {
                    #(
                        | Self::#each_physical_variant_copy ( x ) => {
                            Self::#each_physical_variant_copy (x.set_value(new_value))
                        },
                    )*
                    }
                }
            }
        }
    } else {
        quote! {
            impl ValueOP for #enum_name { // <- Assumes there being no generics.
                fn get_value (self: &'_ Self)
                  -> String
                {
                    match self {
                    #(
                        | Self::#each_physical_variant ( x ) => {
                            x.clone().unwrap_or_default().to_string()
                        },
                    )*
                    }
                }

                fn set_value (self: &'_ Self, new_value: String)
                  -> Self
                {
                    match self {
                    #(
                        | Self::#each_physical_variant_copy ( _ ) => {
                            Self::#each_physical_variant_copy (new_value.parse().ok())
                        },
                    )*
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(AsTotalMsgMacro, attributes(totalMsgName))]
pub fn derive_as_total_msg(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

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

#[proc_macro_derive(AsSettingOptionMacro, attributes(page))]
pub fn derive_as_setting_option(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
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

    let page = &input
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

    let config = struct_name;
    let expanded = quote! {
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
// impl AsTotalMsg for T1Msg {
//     fn to_total_msg(&self) -> TotalMsg {
//         TotalMsg::TriggerMsg(Msgs::T1(self.clone()))
//     }
// }

#[proc_macro_derive(SettingCallbackFnMacro)]
pub fn derive_setting_callback_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name @ _ = &input.ident;

    let msg_name = enum_name.to_string().replace("Msg", "OptionUpdate");
    let msg_name_i = Ident::new(&msg_name, enum_name.span());

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
    let each_type = variants.iter().map(
        |Variant {
             ident: variant_name @ _,
             ..
         }| {
            let new_type_name = if is_top_msg.is_some() {
                format!("{}Msg", variant_name.to_string())
            } else {
                format!("{}", msg_name_i)
            };
            Ident::new(&new_type_name, variant_name.span())
        },
    );

    let expanded = if is_top_msg.is_some() {
        quote! {
            impl SettingCallbackFn<PMsg> for #enum_name { // <- Assumes there being no generics.
                fn msg_fn() -> Box<dyn Fn(Self) -> PMsg> {
                    let f = |x| -> PMsg {
                        match x {
                            #(
                                | Self::#each_physical_variant ( v ) => {
                                    <#each_type as SettingCallbackFn<PMsg>>::msg_fn()(v)
                                },
                            )*
                        }
                    };
                    Box::new(f)
                }
            }
        }
    } else {
        quote! {
            impl SettingCallbackFn<PMsg> for #enum_name { // <- Assumes there being no generics.
                fn msg_fn() -> Box<dyn Fn(Self) -> PMsg> {
                    let f = |x| -> PMsg {
                        match x {
                            #(
                                | Self::#each_physical_variant ( x ) => {
                                    PMsg::#each_type(Self::#each_physical_variant(x))
                                },
                            )*
                        }
                    };
                    Box::new(f)
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(CallbackMsgMacro, attributes(page))]
pub fn derive_call_back_msg(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
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
    let page = &input
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
    // .find(|x| x.contains("Msg"))
    // .is_some();
    let each_physical_variant_copy = each_physical_variant.clone();

    let expanded = if is_top_msg.is_some() {
        quote! {
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
                type O = crate::components::#page::Msg;

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
