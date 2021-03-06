use proc_macro::Span;
use proc_macro::TokenStream;
use syn::DataEnum;
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
