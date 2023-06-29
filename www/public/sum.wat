(module
  (import "console" "log" (func $log))
  (import "console" "error" (func $error))
  (memory (import "js" "mem") 1)
  (data (i32.const 0) "Hi")
  (func $sum (param i32 i32) (result i32)
    call $log
    call $error
    local.get 0
    local.get 1
    i32.add)
  (export "sum" (func $sum))
)