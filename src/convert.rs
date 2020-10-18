#![allow(unused)]
use crate::extract::{
  extract_arg_as_lit, extract_arg_as_macro, extract_function_name, extract_int,
  extract_ints_from_vec_macro, extract_rule_from_turbofish,
  extract_tuples_from_vec_macro, FunctionName,
};
use quote::ToTokens;
use std::fmt;
use syn::{Expr, ItemFn, Lit, Stmt};

pub fn valid_cases(item_fn: &ItemFn) {
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

  let codes: Vec<_> = codes
    .into_iter()
    .map(|c| c.to_token_stream().to_string())
    .collect();

  let output = format!(
    r#"
    assert_lint_ok_macro! {{
      {rule},
      [
        {codes},
      ],
    }};
"#,
    rule = rule.unwrap().to_string(),
    codes = codes.join(",\n      ")
  );
  println!("{}", output);
}

#[derive(Debug)]
pub struct Error {
  pub line: Option<usize>,
  pub col: usize,
  pub message: &'static str,
  pub hint: Option<&'static str>,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut s = "            {\n".to_string();
    if let Some(line) = self.line {
      s += &format!("              line: {},\n", line);
    }
    s += &format!("              col: {},\n", self.col);
    s += &format!("              message: \"{}\",\n", self.message);
    if let Some(hint) = self.hint {
      s += &format!("              hint: {},\n", hint);
    }
    s += "            }";

    write!(f, "{}", s)
  }
}

pub fn invalid_cases(item_fn: &ItemFn, default_message: &'static str) {
  let mut rule = None;
  let mut errors: Vec<(&Lit, Vec<Error>)> = Vec::new();

  for stmt in &item_fn.block.stmts {
    if let Stmt::Semi(expr, _) = stmt {
      if let Expr::Call(expr_call) = expr {
        if rule.is_none() {
          rule = Some(extract_rule_from_turbofish(&*expr_call.func));
        }

        let args = &expr_call.args;
        let invalid_src = extract_arg_as_lit(args, 0);

        let mut inner_errors: Vec<Error> = Vec::new();

        match extract_function_name(&*expr_call.func) {
          FunctionName::Err => {
            let col = extract_int(extract_arg_as_lit(args, 1)).unwrap();
            let e = Error {
              line: None,
              col,
              message: default_message,
              hint: None,
            };
            inner_errors.push(e);
          }
          FunctionName::ErrN => {
            let vec_macro = extract_arg_as_macro(args, 1);
            let columns = extract_ints_from_vec_macro(vec_macro);
            for col in columns {
              let e = Error {
                line: None,
                col,
                message: default_message,
                hint: None,
              };
              inner_errors.push(e);
            }
          }
          FunctionName::ErrOnLine => {
            let line = extract_int(extract_arg_as_lit(args, 1)).unwrap();
            let col = extract_int(extract_arg_as_lit(args, 2)).unwrap();
            let e = Error {
              line: Some(line),
              col,
              message: default_message,
              hint: None,
            };
            inner_errors.push(e);
          }
          FunctionName::ErrOnLineN => {
            let vec_macro = extract_arg_as_macro(args, 1);
            let result = extract_tuples_from_vec_macro(vec_macro);
            for (line, col) in result {
              let e = Error {
                line: Some(line),
                col,
                message: default_message,
                hint: None,
              };
              inner_errors.push(e);
            }
          }
          _ => {}
        }

        errors.push((invalid_src, inner_errors));
      }
    }
  }

  let error_output = errors
    .into_iter()
    .map(|(src, errors)| {
      let mut s = format!("{}: [\n", src.to_token_stream());
      s += errors
        .into_iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join(",\n")
        .as_str();
      s += "\n          ]";
      s
    })
    .collect::<Vec<_>>()
    .join(",\n");

  let output = format!(
    r#"
    assert_lint_err_macro! {{
      {rule},
      {error_output}
    }};
"#,
    rule = rule.unwrap().to_string(),
    error_output = error_output,
  );
  println!("{}", output);
}
