extern crate proc_macro;
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub fn hex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros_lib::hex(input.into()).into()
}
