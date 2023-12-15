extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro_derive(ExampleMacro)]
pub fn example_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(fields),
        ..
    }) = input.data
    {
        fields.named
    } else {
        return TokenStream::new();
    };

    let field_names = fields.iter().map(|f| &f.ident);
    let expanded = quote! {
        impl #name {
            pub fn example_method(&self) {
                #(println!("Field {}: {:?}", stringfy!(#field_names), &self.#field_names);)*
            }
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn log_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;

    let expanded = quote! {
        fn #fn_name() {
            println!("Function {} is called", stringify!(#fn_name));
            println!("Block {} is called", stringify!(#fn_block));
            #fn_block
        }
    };

    expanded.into()
}
