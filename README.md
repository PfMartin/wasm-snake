# Snake Game Notes

## Rust compilation

- Can create binary executable or library
- LLVM is used to create binary

## WebAssembly

### Introduction



## Learnings

- You don't have to create a project every time you want to use rust. You can just create a `main.rs` and use `rustc main.rs`
- Using `impl` and `dyn` both work but the compiled file with `dyn` will be smaller
  - `impl` Will create a separate function for every variant, which implements the trait at compile time
  - `dyn` Dynamic dispatch: Decision of exactly which function to call at run time

    ```rust
    fn log_info(val: impl Log) {
      val.log();
    }

    fn log_info(val: dyn Log) {
      val.log()
    }
    ```
