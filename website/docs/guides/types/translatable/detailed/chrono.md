# DateTime (Chrono)

Codegen optionally support [chrono crate](https://docs.rs/chrono) with feature `chrono`.

| :crab: Rust       | :dart: Dart                   |
| -----------       | -----------                   |
| `DateTime<Utc>`   | `DateTime` *utc*              |
| `DateTime<Local>` | `DateTime` *local timezone*   |
| `NaiveDateTime`   | `DateTime` *utc assumed*      |
| `Duration`        | `Duration`                    |

You can also use nullable values through `Option`, for example: `Option<NaiveDateTime>`.

:warning: Please note that:

- on native platforms, *microseconds* unit is used.
- on web platform, *milliseconds* unit is used (due to JS limitation with dates).

:bulb: Also a `DateTime<Local>` will always be translated into local time of the device, which might not be what you want if you expect them to be sent *as-is*.

> In that case, you could implement it in your codebase by sending a `u32` (timezone offset) alongside the `i64` (timestamp) over the wire, or open a issue / PR here to further discuss it. The reason why this choice was originally made is to have all `DateTime<Utc>`, `DateTime<Local>` and `NaiveDateTime` been represented by a single `i64`.
