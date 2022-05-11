use hello_macro::HelloMacro;
use hello_macro_derive::api;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
    println!("Hello, world!");
    // test();
    get();
}

// #[my_proc_macro_attribute(blog::(::ideawand::com))]
// fn test() {
//     println!("hello, blog.ideawand.com, hello, 极客幼稚园");
// }

// #[api("/index")]
#[api(blog::article)]
fn get() {
    println!("get /index");
}
