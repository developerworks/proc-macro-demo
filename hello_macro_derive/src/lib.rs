extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Item};

// 定义派生过程宏
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

/// 属性宏
#[proc_macro_attribute]
pub fn my_proc_macro_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("attr: {:#?}", attr);
    eprintln!("item: {:#?}", item);
    item
}

#[proc_macro_attribute]
pub fn api(attr: TokenStream, item: TokenStream) -> TokenStream {
    let macro_attr = parse_macro_input!(attr as AttributeArgs);
    eprintln!("attr: {:#?}", macro_attr);
    let code = parse_macro_input!(item as Item);
    eprintln!("item: {:#?}", code);
    quote!(#code).into()
}


// 函数宏
#[proc_macro]
pub fn func_macro(item: TokenStream) -> TokenStream {
    item
}
