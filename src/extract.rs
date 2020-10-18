use proc_macro2::TokenTree;
use syn::{Expr, GenericArgument, Ident, Lit, Macro, PathArguments, Type};

#[derive(Debug)]
pub enum FunctionName {
  Ok,
  OkN,
  Err,
  ErrOnLine,
  ErrN,
  ErrOnLineN,
}

impl<S: AsRef<str>> From<S> for FunctionName {
  fn from(s: S) -> Self {
    let s = s.as_ref();
    match s {
      "assert_lint_ok" => Self::Ok,
      "assert_lint_ok_n" => Self::OkN,
      "assert_lint_err" => Self::Err,
      "assert_lint_err_n" => Self::ErrN,
      "assert_lint_err_on_line" => Self::ErrOnLine,
      "assert_lint_err_on_line_n" => Self::ErrOnLineN,
      _ => unreachable!(),
    }
  }
}

pub fn extract_arg_as_lit<'a>(
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

pub fn extract_arg_as_macro<'a>(
  args: impl IntoIterator<Item = &'a Expr>,
  index: usize,
) -> &'a Macro {
  args
    .into_iter()
    .enumerate()
    .find_map(|(i, elem)| {
      if i == index {
        if let Expr::Macro(expr_macro) = elem {
          Some(&expr_macro.mac)
        } else {
          None
        }
      } else {
        None
      }
    })
    .unwrap()
}

pub fn extract_ints_from_vec_macro(mac: &Macro) -> Vec<usize> {
  let mut ret = Vec::new();
  let tokens = mac.tokens.clone();
  for token in tokens.into_iter() {
    if let TokenTree::Literal(lit) = token {
      ret.push(lit.to_string().parse::<usize>().unwrap());
    }
  }
  ret
}

pub fn extract_tuples_from_vec_macro(mac: &Macro) -> Vec<(usize, usize)> {
  let mut ret = Vec::new();
  let tokens = mac.tokens.clone();
  for token in tokens.into_iter() {
    let mut tmp_tuple = Vec::new();

    if let TokenTree::Group(group) = token {
      for token in group.stream().into_iter() {
        if let TokenTree::Literal(lit) = token {
          tmp_tuple.push(lit.to_string().parse::<usize>().unwrap());
        }
      }
    }

    if tmp_tuple.len() == 2 {
      ret.push((tmp_tuple[0], tmp_tuple[1]));
    }
  }

  ret
}

pub fn extract_str_literals_from_vec_macro<'a>(mac: &'a Macro) -> Vec<String> {
  let mut ret = Vec::new();
  let tokens = mac.tokens.clone();
  for token in tokens {
    if let TokenTree::Literal(lit) = token {
      ret.push(lit.to_string());
    }
  }
  ret
}

pub fn extract_rule_from_turbofish(func: &Expr) -> &Ident {
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

pub fn extract_function_name(func: &Expr) -> FunctionName {
  if let Expr::Path(expr_path) = func {
    let first_path_segment = expr_path.path.segments.iter().next().unwrap();
    return first_path_segment.ident.to_string().into();
  }
  unreachable!()
}

pub fn extract_int(lit: &Lit) -> Option<usize> {
  if let Lit::Int(lit_int) = lit {
    lit_int.base10_parse::<usize>().ok()
  } else {
    None
  }
}
