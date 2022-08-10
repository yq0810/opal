mod as_setting_option;
mod as_total_msg;
mod call_back_msg;
mod setting_callback_fn;
mod sqlgogo;
mod value_op;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Sqlgogo, attributes(result))]
pub fn derive_sqlgogo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    sqlgogo::expanded(input)
}

#[proc_macro_derive(ValueOPMacro)]
pub fn derive_value_op(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    value_op::expanded(input)
}

#[proc_macro_derive(AsTotalMsgMacro, attributes(totalMsgName))]
pub fn derive_as_total_msg(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    as_total_msg::expanded(input)
}

#[proc_macro_derive(AsSettingOptionMacro, attributes(page))]
pub fn derive_as_setting_option(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    as_setting_option::expanded(input)
}

#[proc_macro_derive(SettingCallbackFnMacro, attributes(page))]
pub fn derive_setting_callback_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    setting_callback_fn::expanded(input)
}

#[proc_macro_derive(CallbackMsgMacro, attributes(page))]
pub fn derive_call_back_msg(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    call_back_msg::expanded(input)
}

// total

#[proc_macro_derive(WidgetMsg, attributes(totalMsgName, page))]
pub fn derive_widget_msg(input: TokenStream) -> TokenStream {
    let import = quote! {
        use crate::components;
        use crate::SettingCallbackFn;
        use crate::AsTotalMsg;
    };
    let import = TokenStream::from(import);
    let input = parse_macro_input!(input as DeriveInput);
    let a = value_op::expanded(input.clone());
    let b = as_total_msg::expanded(input.clone());
    let c = call_back_msg::expanded(input.clone());
    let d = setting_callback_fn::expanded(input.clone());

    let tokens = vec![import, a, b, c, d];
    TokenStream::from_iter(tokens)
}

#[proc_macro_derive(OptionMsg, attributes(page))]
pub fn derive_option_msg(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let a = value_op::expanded(input.clone());
    let c = call_back_msg::expanded(input.clone());
    let d = setting_callback_fn::expanded(input.clone());

    let tokens = vec![a, c, d];
    TokenStream::from_iter(tokens)
}
