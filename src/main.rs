mod convert;
mod extract;

use std::collections::VecDeque;
use std::env;
use std::io::{self, Read};
use syn::{ItemFn, Result as SynResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let input = read_from_stdin()?;
  let (msg, hint) = parse_args(env::args().skip(1))?;
  let item_fn = parse_as_fn(input)?;

  //  convert::valid_cases(&item_fn);
  convert::invalid_cases(&item_fn, msg, hint);

  Ok(())
}

fn read_from_stdin() -> Result<String, &'static str> {
  let mut buffer = String::new();
  io::stdin()
    .read_to_string(&mut buffer)
    .map_err(|_| "Failed to read source code from stdin")?;
  Ok(buffer)
}

fn parse_args(
  args: impl Iterator<Item = String>,
) -> Result<(String, Option<String>), &'static str> {
  let mut args: VecDeque<String> = args.collect();
  if args.is_empty() || args.len() > 2 {
    return Err(
      "USAGE: cat input_invalid | cargo run -- <DIAGNOSTIC MESSAGE> <HINT (optional)>",
    );
  }

  let message = args.pop_front().unwrap();
  let hint = args.pop_front();

  Ok((message, hint))
}

fn parse_as_fn(src: impl AsRef<str>) -> SynResult<ItemFn> {
  let src = src.as_ref();
  let func = syn::parse_str::<ItemFn>(src)?;
  Ok(func)
}
