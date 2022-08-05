use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DataEnum, DeriveInput, Ident, Variant};

pub fn expanded(input: DeriveInput) -> TokenStream {
    let enum_name @ _ = &input.ident;

    let msg_name = enum_name.to_string().replace("Msg", "OptionUpdate");
    let msg_name_i = Ident::new(&msg_name, enum_name.span());
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
            impl SettingCallbackFn<#page::Msg> for #enum_name { // <- Assumes there being no generics.
                fn msg_fn() -> Box<dyn Fn(Self) -> #page::Msg> {
                    let f = |x| -> #page::Msg {
                        match x {
                            #(
                                | Self::#each_physical_variant ( v ) => {
                                    <#each_type as SettingCallbackFn<#page::Msg>>::msg_fn()(v)
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
            impl SettingCallbackFn<#page::Msg> for #enum_name { // <- Assumes there being no generics.
                fn msg_fn() -> Box<dyn Fn(Self) -> #page::Msg> {
                    let f = |x| -> #page::Msg {
                        match x {
                            #(
                                | Self::#each_physical_variant ( x ) => {
                                    #page::Msg::#each_type(Self::#each_physical_variant(x))
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
