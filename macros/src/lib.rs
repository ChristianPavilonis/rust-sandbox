use proc_macro::TokenStream;
use rs_ray::{ray, Ray};
extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, LitStr};

// #[proc_macro]
// pub fn it(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     // Parse the input tokens as a string literal.
//     let input = parse_macro_input!(input as LitStr);
//     let test_name = input.value();
//
//     // Transform the test name into a valid Rust function identifier.
//     let function_name = test_name.replace(" ", "_");
//
//     // Generate the output tokens as Rust code using the `quote` crate.
//     let output = quote! {
//         #[test]
//         fn #function_name() {
//             #input
//         }
//     };
//
//     // Return the generated Rust code as a token stream.
//     output.into()
// }

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    ray!("attr", attr.to_string());
    ray!("item", item.to_string());
    item
}
