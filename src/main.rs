#![allow(unused)]
use quote::ToTokens;
use std::io::{self, Read};
use syn::{ItemFn, Result as SynResult};

fn main() {
    // TODO(magurotuna): remove next line
    //let input = read_from_stdin();

    let input = "fn foo() -> i32 { 42 }";
    let result = parse_as_fn(input).unwrap();
    eprintln!("{}", result.to_token_stream());
}

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn parse_as_fn(src: &str) -> SynResult<ItemFn> {
    let func = syn::parse_str::<ItemFn>(src)?;
    Ok(func)
}
