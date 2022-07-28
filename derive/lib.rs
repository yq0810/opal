use proc_macro::Span;
use proc_macro::TokenStream;
use syn::DataEnum;
use syn::Path;
use syn::Variant;
use syn::{parse_macro_input, DeriveInput};

use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, Error, Fields, Ident};

#[proc_macro_derive(Sqlgogo, attributes(result))]
pub fn derive_sqlgogo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ref name = input.ident;
    let ref data = input.data;

    let EnumName @ _ = &input.ident;

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
                 ident: VariantName @ _,
                 ..
             }| VariantName,
        );

    let each_type = variants
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
        impl Entrys for #EnumName { // <- Assumes there being no generics.
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
    let ref name = input.ident;
    let ref data = input.data;

    let EnumName @ _ = &input.ident;

    let variants = match &input.data {
        Data::Enum(DataEnum { variants: it, .. }) => it,
        _ => unreachable!(),
    };
    let each_physical_variant = variants.iter().map(
        |Variant {
             ident: VariantName @ _,
             ..
         }| VariantName,
    );

    let each_physical_variant_copy = each_physical_variant.clone();
    let expanded = quote! {
        impl ValueOP for #EnumName { // <- Assumes there being no generics.
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
    };

    TokenStream::from(expanded)
}





#[proc_macro_derive(AsTotalMsgMacro, attributes(totalMsgName))]
pub fn derive_as_total_msg(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ref name = input.ident;
    let ref data = input.data;

    let EnumName @ _ = &input.ident;

    let msg_name = EnumName.to_string().replace("Msg", "");
    let msg_name_i = Ident::new(&msg_name, EnumName.span());
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
        impl AsTotalMsg for #EnumName { // <- Assumes there being no generics.
            fn to_total_msg (self: &'_ Self)
              -> TotalMsg
            {
                TotalMsg::#total_msg_name(Msgs::#msg_name_i(self.clone()))
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