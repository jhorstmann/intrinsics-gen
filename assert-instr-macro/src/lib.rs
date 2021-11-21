use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn assert_instr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
