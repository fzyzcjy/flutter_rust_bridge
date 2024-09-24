# Extra Dart code

Arbitrary extra Dart code can be inserted into auto-generated Dart classes,
by using the attribute `#[frb(dart_code = ...)]` syntax.

## Example

```rust
#[frb(dart_code = "
    int extraMethod() => a * 2;
"
)]
pub struct MyStruct {
    ...
}
```

Then the generated Dart class will look like:

```dart
class MyStruct {
  ... // other auto-generated code
  
  int extraMethod() => a * 2; // The extra code
}
```

## Where to annotate the attribute?
You can annotate the attribute only at rust `struct`s and `enum`s.
It doesn't work on `function`s, `mod`s or other elements, which are not translated into dart `classes`.

The dart code is _copied_ into the generated dart class, inside the scope of the rust element, which was annotated.

For example, if you annotate a `struct MyStruct` in the rust file `minimal.rs`, then the dart code will be inserted into the generated dart class `class MyStruct` in the generated file `minimal.dart`, inside this class definition.

## Impossible Dart Code
Because the dart_code is inserted into the generated dart code from the rust element you annotated, the result must be valid dart code.

Thus a pure statement like `int a = 10;` will not work as expected if inserted into a `struct`, it needs to be a member field, like a variable or method/function.

For the same reason `extend` doesn't work neither.

## Possible Dart Code
Any valid Dart Code for a class body is allowed in the `#[frb(dart_code =...)]`. 
  
In order to `import` things, simply write down the import statements besides normal code, and it will be automatically recognized and pasted.

## Works only if rust code is callable
Your annotated rust struct needs to be callable by Dart - otherwise no code will be generated.

You can achieve this in two ways: Having an additional rust function inside your struct:

```rust
#[frb(dart_code = "static void dartSay() => print('Dart_code at struct');
")]
// #[frb]
pub struct DartCodeStruct {}
impl DartCodeStruct {
    pub fn noop() {}
}
```

or having another rust function using your rust struct as a parameter:

```rust
#[frb(dart_code = "static void dartSay() => print('Dart_code at struct');
")]
// #[frb]
pub struct DartCodeStruct {}

pub fn noop(dcs : DartCodeStruct){}
```

## toubleshooting
You can see if the dart_code was inserted into the generated code (e.g. `myRustCode.dart`) by looking into this file.
If you see the comment 
```rust
// These types are ignored because they are not used by any `pub` functions: `MyStruct`
```
then your rust code is not generated, thus no dart_code was inserted.

Another error could be that your dart_code is invalid.
If so you will get the error message: `stderr=Could not format because the source could not be parsed:` when running the code generation.
