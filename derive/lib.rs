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
    let EachPhysicalVariant = variants
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

    let EachType =
        variants
            .iter()
            // Only keep the `#[physical]`-annotated variants
            // .filter(|variant| {
            //     variant
            //         .attrs
            //         .iter()
            //         .any(|attr| attr.path.is_ident("result"))
            // })
            // From `Foo { … }` get `Foo`
            .map(|Variant { ident: VariantName @ _, .. }| {
                let new_type_name = format!("{}Result",VariantName.to_string().split("By").collect::<Vec<_>>().first().unwrap());
                Ident::new(&new_type_name,VariantName.span()) })
    // let EachType =
    //     variants
    //         .iter()
    //         .map(|variant| {
    //             variant
    //                 .attrs
    //                 .iter()
    //                 .filter(|attr| attr.path.is_ident("result"))
    //                 .collect::<Vec<_>>().first().unwrap().clone().path.get_ident()})
    ;

    let expanded = quote! {
        impl Entrys for #EnumName { // <- Assumes there being no generics.
            fn entrys (self: &'_ Self, js: JsValue)
              -> Vec<std::string::String>
            {
                match *self {
                #(
                    | Self::#EachPhysicalVariant { .. } => {
                        #EachType::from_entrys(js)
                    },
                )*
                // | _ => false,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
