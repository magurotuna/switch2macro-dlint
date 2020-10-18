mod convert;
mod extract;

use std::io::{self, Read};
use syn::{ItemFn, Result as SynResult};

fn main() {
  let input = read_from_stdin();
  let item_fn = parse_as_fn(input).unwrap();

  //convert::valid_cases(&item_fn);
  convert::invalid_cases(
    &item_fn,
    "ts directives are not allowed without comment",
    Some(
      "Add an in-line comment explaining the reason for using this directive",
    ),
  );
}

fn read_from_stdin() -> String {
  let mut buffer = String::new();
  io::stdin().read_to_string(&mut buffer).unwrap();
  buffer
}

fn parse_as_fn(src: impl AsRef<str>) -> SynResult<ItemFn> {
  let src = src.as_ref();
  let func = syn::parse_str::<ItemFn>(src)?;
  Ok(func)
}
