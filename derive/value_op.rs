use proc_macro::TokenStream;
use quote::quote;
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
    });
    let each_physical_variant_copy = each_physical_variant.clone();
    let expanded = if is_top_msg.is_some() {
        quote! {
            use crate::ValueOP;
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
            use crate::ValueOP;
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
