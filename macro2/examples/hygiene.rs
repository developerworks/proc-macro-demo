#![feature(decl_macro)]
// try uncommenting the following line, and commenting out the line right after

// macro_rules! foso {
macro foo($name: ident) {
    pub struct $name;
    impl Default for $name {
        fn default() -> $name {
            $name
        }
    }
}

foo!(Foo);

fn main() {
    // this fails with a `macro`, but succeeds with a `macro_rules`
    let foo = Foo::default();
}
