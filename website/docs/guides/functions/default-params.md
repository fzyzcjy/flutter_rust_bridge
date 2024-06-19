# Default parameters

Dart allows default values for function and constructor parameters, and you can achieve the same effect using `#[frb(default)]`. The syntax is as follows:

- If the parameter is a `String` or any other primitive, `#[frb(default = ".." | 0 | true | ..)]` annotates its default value.
- If the parameter is a class or an enum, `#[frb(default = "..")]` annotates the *Dart code* to initialize the parameter.
  Note that this is run in the *constant context*, so classes can only be constructed if they are preceded with `const`.

This will be translated to either a default value annotation, or Freezed's `@Default` in the case of enum constructor parameters.

## Example

```rust
pub enum Answer { Yes, No }
pub struct Point(pub f64, pub f64);

#[frb]
pub fn defaults(
    #[frb(default = "Answer.Yes")]
    answer: Answer,
    #[frb(default = "const Point(field0: 2, field1: 3)")]
    point: Point,
);
```