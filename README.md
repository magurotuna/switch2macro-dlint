# What's this

This tool helps you to migrate to `assert_lint_err!` macro in [`deno_lint`](https://github.com/deno_land/deno_lint).

# Usage

First of all, prepare an original source code. You can find the example input in the `input_invalid` file, where the test function `for_direction_invalid` is placed. As you can see, make sure that your source code is a function, because the tool will try to parse it as a function.

Your source code will be read from stdin, so you may choose whatever options where you can pass the code to stdin.

(Personally I recommend using `pbpaste` command which is available on macOS or any other similar commands.)

If you have the source code prepared, let's execute this tool with one or two arguments, like:

```sh
$ cat input_invalid | cargo run -- 'required arg' 'optional arg'
```

then you'll get the converted result!

```rust
    assert_lint_err! {
      ForDirection,
      "for(let i = 0; i < 2; i--) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 0; i < 2; --i) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 0; i <= 2; i--) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 0; i <= 2; --i) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 2; i > 2; i++) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 2; i > 2; ++i) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 2; i >= 0; i++) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 2; i >= 0; ++i) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 0; i < 2; i -= 1) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 0; i <= 2; i -= 1) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 2; i > 2; i -= -1) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 2; i >= 0; i -= -1) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 2; i > 2; i += 1) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 2; i >= 0; i += 1) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 0; i < 2; i += -1) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      "for(let i = 0; i <= 2; i += -1) {}": [
        {
          col: 0,
          message: "required arg",
          hint: "optional arg",
        }
      ],
      r#"
for (let i = 0; i < 2; i++) {
  for (let j = 0; j < 2; j--) {}
}
      "#: [
        {
          line: 3,
          col: 2,
          message: "required arg",
          hint: "optional arg",
        }
      ]
    };
```



