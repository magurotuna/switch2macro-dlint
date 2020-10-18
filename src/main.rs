#![allow(unused)]
use quote::{quote, ToTokens};
use std::io::{self, Read};
use syn::{
  Expr, ExprCall, GenericArgument, Ident, ItemFn, Lit, LitStr, PathArguments,
  Result as SynResult, Stmt, Type,
};

fn main() {
  let input = read_from_stdin();

  //let input = r##"
  //fn foo() -> i32 {
  //assert_lint_ok::<PreferConst>(r#"var x = 0;"#);
  //assert_lint_ok::<PreferConst>(r#"var y = 0;"#);
  //}
  //"##;
  let item_fn = parse_as_fn(input).unwrap();
  convert_valid_cases(item_fn);
}

fn convert_valid_cases(item_fn: ItemFn) {
  let mut rule = None;
  let mut codes = Vec::new();

  for stmt in &item_fn.block.stmts {
    if let Stmt::Semi(expr, _) = stmt {
      if let Expr::Call(expr_call) = expr {
        let args = &expr_call.args;
        let valid_src = extract_arg_as_lit(args, 0);

        if rule.is_none() {
          rule = Some(extract_rule_from_turbofish(&*expr_call.func));
        }

        codes.push(valid_src);
      }
    }
  }

  let output_codes = quote! {
    [
      #(#codes),*
    ]
  };
  let rule = rule.unwrap();
  let output = quote! {
    assert_lint_ok_macro! {
      #rule,
      #output_codes
    };
  };
  eprintln!("{}", output);
}

fn extract_arg_as_lit<'a>(
  args: impl IntoIterator<Item = &'a Expr>,
  index: usize,
) -> &'a Lit {
  args
    .into_iter()
    .enumerate()
    .find_map(|(i, elem)| {
      if i == index {
        if let Expr::Lit(expr_lit) = elem {
          Some(&expr_lit.lit)
        } else {
          None
        }
      } else {
        None
      }
    })
    .unwrap()
}

fn extract_rule_from_turbofish(func: &Expr) -> &Ident {
  if let Expr::Path(expr_path) = func {
    let first_path_segment = expr_path.path.segments.iter().next().unwrap();
    if let PathArguments::AngleBracketed(angle_generic) =
      &first_path_segment.arguments
    {
      let first_generic = angle_generic.args.iter().next().unwrap();
      if let GenericArgument::Type(ty) = first_generic {
        if let Type::Path(type_path) = ty {
          let first_path_segment =
            type_path.path.segments.iter().next().unwrap();
          return &first_path_segment.ident;
        }
      }
    }
  }
  unreachable!()
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
