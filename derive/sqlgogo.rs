use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Ident, Variant};

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
