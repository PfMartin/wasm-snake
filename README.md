# Snake Game Notes

## Rust compilation

- Can create binary executable or library
- LLVM is used to create binary

## WebAssembly

### Introduction

- We need to compile Rust to WebAssembly code
- WebAssembly can be executed in the browser
- It runs at native speed
- Suitable for applications that use a lot of resources
  - 3D apps
  - CAD
  - Virtual Reality
  - System applications
  - Games
- WebAssembly consists of nodes
- Only knows the following types `i32`, `i64`, `f32`, `f64`
- WebAssembly function for adding two numbers
- WAT (WebAssembly Text)
- [wat to wasm Demo](https://webassembly.github.io/wabt/demo/wat2wasm/)
  
    ```wat
      (module
        (func $sum (param $a i32) (param $b i32) (result i32)
          local.get $a
          local.get $b
          i32.add
        )
        (export "sum" (func $sum))
      )
    ```

- Look at webassembly instructions inside a `.wasm` file `xxd -g1 sum.wasm`
- You can just load these instructions in javscript

    ```js
    async function init() {
      const byteArray = new Int8Array([
          0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01,
          0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07,
          0x07, 0x01, 0x03, 0x73, 0x75, 0x6d, 0x00, 0x00, 0x0a, 0x09, 0x01,
          0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b, 0x00, 0x18, 0x04,
          0x6e, 0x61, 0x6d, 0x65, 0x01, 0x06, 0x01, 0x00, 0x03, 0x73, 0x75,
          0x6d, 0x02, 0x09, 0x01, 0x00, 0x02, 0x00, 0x01, 0x61, 0x01, 0x01,
          0x62,
        ]);

        const wasm = await WebAssembly.instantiate(byteArray.buffer);
        const sumFunction = wasm.instance.exports.sum;
        const result = sumFunction(100, 1000);
        console.log(result);
    }
    ```

- Import to wasm
  - You can also import functions to wasm
  - Just define an importObject and import it

      ```wat
      (module
        (import "console" "log" (func $log))
        (import "console" "error" (func $error))
        (func $sum (param $a i32) (param $b i32) (result i32)
          local.get $a
          local.get $b
          i32.add
        )
        (export "sum" (func $sum))
      )
      ```

      ```js
      const importObject = {
        console: {
          log: () => {
            console.log('Simple console.log')
          },
          error: () => {
            console.error('Simple console.error')
          }
        }
      }

      const res = await fetch('sum.wasm');
      const buffer = await res.arrayBuffer();
      const wasm = await WebAssembly.instantiate(buffer, importObject);
      ```


### Resources

- [Rust WebAssembly](https://rustwasm.github.io/book/)
- [Mozilla WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly)

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
