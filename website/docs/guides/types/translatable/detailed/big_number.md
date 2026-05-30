# Big number

Codegen optionally supports big integer and decimal types from [`num_bigint`](https://docs.rs/num-bigint), [`rust_decimal`](https://docs.rs/rust_decimal), and [`bigdecimal`](https://docs.rs/bigdecimal).

| :crab: Rust                                  | :dart: Dart |
|----------------------------------------------|-------------|
| `num_bigint::BigInt`                         | `BigInt`    |
| `num_bigint::BigUint`                        | `BigInt`    |
| `bigdecimal::num_bigint::BigInt`             | `BigInt`    |
| `bigdecimal::num_bigint::BigUint`            | `BigInt`    |
| `rust_decimal::Decimal`                      | `Decimal`   |
| `bigdecimal::BigDecimal`                     | `Decimal`   |

## Setup

Enable the Rust feature for each crate you use:

```toml
[dependencies]
flutter_rust_bridge = { features = ["num-bigint", "rust_decimal", "bigdecimal"] }
num-bigint = "0.4"
rust_decimal = "1"
bigdecimal = "0.4"
```

When using decimal types, add the Dart package:

```yaml
dependencies:
  decimal: ^3.2.4
```

## Example

```rust
pub struct BigNumberExample {
    pub signed: num_bigint::BigInt,
    pub unsigned: num_bigint::BigUint,
    pub decimal: rust_decimal::Decimal,
    pub big_decimal: bigdecimal::BigDecimal,
}

pub fn echo_big_number(input: BigNumberExample) -> BigNumberExample {
    input
}
```

```dart
final result = await echoBigNumber(
  input: BigNumberExample(
    signed: BigInt.parse('-170141183460469231731687303715884105728'),
    unsigned: BigInt.parse('340282366920938463463374607431768211455'),
    decimal: Decimal.parse('123456789.123456789'),
    bigDecimal: Decimal.parse('-987654321.987654321'),
  ),
);
```

:bulb: Implementation detail: these types are serialized as decimal strings across the FFI boundary, then parsed on the receiving side.
