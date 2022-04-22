#[macro_use]
extern crate quote;
extern crate pest_generator;
extern crate proc_macro;

mod build;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
#[ignore]
fn gen() {
    build::gen_parser();
}
