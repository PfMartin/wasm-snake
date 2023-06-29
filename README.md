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

### WebAssembly as bytearray

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

### Import JavaScript to WASM

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

### Sharing memory

- Create memory in WebAssembly and export it to JS

    ```bash
    (memory $mem 1) # create some memory
    (data (i32.const 0) "Hi") # Store the string hi to the create memory
    (func $sum (param i32 i32) (result i32)
      call $log
      call $error
      local.get 0
      local.get 1
      i32.add)
    (export "mem" (memory $mem)) # export the memory with a name mem
    (export "sum" (func $sum))
    ```

    ```js
    //...
    const wasmMemory = wasm.instance.exports.mem;
    const uint8Array = new Uint8Array(wasmMemory.buffer, 0, 2); // Only fist two bytes
    const hiText = new TextDecoder().decode(uint8Array);
    console.log(hiText);
    //...
    ```

- Create memory in JS and import it to WebAssembly

    ```bash
    (memory (import "js" "mem") 1) # import memory
    (data (i32.const 0) "Hi")
    (func $sum (param i32 i32) (result i32)
      call $log
      call $error
      local.get 0
      local.get 1
      i32.add)
    (export "sum" (func $sum))
    ```

    ```js
    const memory = new WebAssembly.Memory({
      initial: 1,
    });

    const importObject = {
      js: {
        mem: memory,
      },
      console: {
        log: () => {
          console.log('Just logging something');
        },
        error: () => {
          console.log("I'm just an error");
        },
      },
    };

    const res = await fetch('sum.wasm');
    const buffer = await res.arrayBuffer();
    const wasm = await WebAssembly.instantiate(buffer, importObject);
    const sumFunction = wasm.instance.exports.sum;

    const uint8Array = new Uint8Array(memory.buffer, 0, 2);
    const hiText = new TextDecoder().decode(uint8Array);

    const result = sumFunction(70, 80);
    console.log(result);
    console.log(hiText);
    ```

### Pack Webassembly

- Rust dependencies:
  - `wasm-bindgen`
  - Add `#[wasm_bindgen]` trait to every function
  - `cargo install wasm-pack`
  - Add lib to `Cargo.toml`

      ```toml
      [lib]
      crate-type = ["cdylib"]
      ```

- `wasm-pack build --target web`
- This will create a `pkg` folder, which contains a javascript module
- Install this module to where you want to use it by adding it to your `package.json`

    ```json
    "snake_game": "file:../pkg"
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
