use hello_macro_derive::api;

fn main() {
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
