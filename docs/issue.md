# Rust 运算符到 Dart 的映射

下表描述当前 codegen 支持的自动映射。Rust 方法会通过完整 trait 路径调用，
例如 `std::ops::Add::add` 和 `std::cmp::PartialEq::eq`。

| Dart 运算符 | Rust trait / 方法 | 结果 |
| --- | --- | --- |
| `a + b` | `std::ops::Add::add` | 支持，生成 `operator +` |
| `a - b` | `std::ops::Sub::sub` | 支持，生成 `operator -` |
| `a * b` | `std::ops::Mul::mul` | 支持，生成 `operator *` |
| `a / b` | `std::ops::Div::div` | 支持，生成 `operator /` |
| `a % b` | `std::ops::Rem::rem` | 支持，生成 `operator %` |
| `-a` | `std::ops::Neg::neg` | 支持，生成一元 `operator -` |
| `~a` | `std::ops::Not::not` | 支持，生成 `operator ~` |
| `a & b` | `std::ops::BitAnd::bitand` | 支持，生成 `operator &` |
| `a \| b` | `std::ops::BitOr::bitor` | 支持，生成 `operator \|` |
| `a ^ b` | `std::ops::BitXor::bitxor` | 支持，生成 `operator ^` |
| `a << b` | `std::ops::Shl::shl` | 支持，生成 `operator <<` |
| `a >> b` | `std::ops::Shr::shr` | 支持，生成 `operator >>` |
| `a == b` | `std::cmp::PartialEq::eq` | 支持，生成 Rust 调用的 `operator ==` |
| `a != b` | `PartialEq::eq` | Dart 对 `==` 取反，不单独生成 `ne` |
| `a < b` | `std::cmp::PartialOrd::lt` | 支持 |
| `a <= b` | `std::cmp::PartialOrd::le` | 支持 |
| `a > b` | `std::cmp::PartialOrd::gt` | 支持 |
| `a >= b` | `std::cmp::PartialOrd::ge` | 支持 |

`PartialOrd::partial_cmp` 没有直接的 Dart 运算符返回类型，因此 codegen 会为它
生成四个布尔函数，分别调用 `lt`、`le`、`gt`、`ge`。

## 不支持或有条件的映射

- Dart 的 `~/` 和 `>>>` 没有唯一对应的 Rust 标准 trait，因此不会自动映射。
- `Index` / `IndexMut` 不自动映射为 `[]` / `[]=`。它们通常返回借用的
  `&Output`，而 FRB 不能在不知道 `Output` 所有权约束的情况下安全地返回该值。
- `AddAssign` 等 `*Assign` trait 不能单独声明 Dart 运算符。Dart 的 `+=` 等语法
  会基于已有的 `+` 等运算符完成赋值；如需 Rust 的原地语义，应显式暴露普通方法。
- Dart 不能在简单 `enum` 上声明具体的 `==` 或 `hashCode`，所以这类枚举保留 Dart
  原有相等行为。Freezed 类型会自动使用 `equal: false`，使 Rust 的 `PartialEq`
  实现在生成的基类中生效。
- 如果 Rust 没有实现 `PartialEq`，结构体继续使用原有的 Dart 字段比较；实现后则
  使用 `identical(this, other) || other is T && runtimeType == other.runtimeType`
  再调用 Rust `PartialEq::eq`。为满足 Dart 相等对象的哈希契约，该实现的
  `hashCode` 固定为 `0`。
