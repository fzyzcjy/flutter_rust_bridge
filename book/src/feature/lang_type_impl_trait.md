# Type Impl Trait

In Rust, we can use signature inputs or signature output as func args.
For example, `pub fn tt(t: impl Serialize) {}` is work.
And `Enum` would transform between dart and rust. So we can translate `TypeImplTrait` to `Enum` make something very imaginative.

## some code that writing by hand can work

  + part 1
  ```rs
  pub enum SerializeEnum {
      Record(Record),
      SerEnum(SerEnum),
  }

  impl Serialize for SerializeEnum {
      fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          match *self {
              SerializeEnum::Record(ref __field0) => __field0.serialize(__serializer),
              SerializeEnum::SerEnum(ref __field0) => __field0.serialize(__serializer),
          }
      }
  }
  ```
  > Some inspiration comes from enum_dispatch, Thanks to the library
and translate signature to 
  + part 2
  ```rs
  pub fn tt(t: SerializeEnum) {}
  ```


## automatically generated results with build.rs
```rs
fn wire_tt_impl(port_: MessagePort, t: impl Wire2Api<SerializeEnum> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "tt",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_t = t.wire2api();
            move |task_callback| Ok(tt(api_t))
        },
    )
}
```
The `Enum of part1 above` is generated automatically, and you don't need to modify your original function in `part 2`, because wire has already implemented translation from `Wire2Api<Impl Serialize>` to `Wire2Api<SerializeEnum>`.

[If you look at the details of the generated code (rust and dart)](https://github.com/huang12zheng/flutter_rust_bridge/blob/51d2d7bedc6a99493bb8b77a84d0d4a82488e650/frb_codegen/src/ir/file.rs),
it translates the IrImplTrait into EnumRef when check dependent function parameters. And then directly calls the TypeEnumRefGenerator to generate the code.


## Some information for reference:
* a more doc about IrImplTrait
```
'impl Aa+Cc' make results to the 'AaCcEnum'
```

- [discuss 866](https://github.com/fzyzcjy/flutter_rust_bridge/discussions/866)

* If you have a function argument as the Type Impl Trait, do not manually implement enum with the same name as the translation. This could conflict.

* some problem (like large_enum_variant and private) maybe need to handle by yourself.