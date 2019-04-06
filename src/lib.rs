extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn increment(_args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);

    let ItemFn {
        vis,
        constness,
        unsafety,
        asyncness,
        abi,
        ident,
        decl,
        block,
        ..
    } = func;

    let inputs = &decl.inputs;
    let output = &decl.output;
    let params = &decl.generics.params;
    let where_clause = &decl.generics.where_clause;

    let quoted_decl = quote!(<#params>(#inputs) #output #where_clause);

    let result = quote! {
        #vis #constness #unsafety #asyncness #abi fn #ident #quoted_decl {
            let result = #block;
            result + 1
        }
    };

    result.into()
}
