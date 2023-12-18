# Async in Rust

> Author: @AlienKevin

This library does not yet support returning a Future type from Rust and this has to do with the difficulty of uniting the various approaches to async in Rust. The [Rust Book](https://rust-lang.github.io/async-book/01_getting_started/03_state_of_async_rust.html#language-and-library-support) summarized the current state of async support succinctly:
> The most fundamental traits, types and functions, such as the Future trait are provided by the standard library. The async/await syntax is supported directly by the Rust compiler.

> Many utility types, macros and functions are provided by the futures crate. They can be used in any async Rust application.

> Execution of async code, IO and task spawning are provided by "async runtimes", such as Tokio and async-std. Most async applications, and some async crates, depend on a specific runtime.

While the futures crate provides an executor called `futures::executor::block_on`, libraries that use Tokio runtime cannot use this executor. According to [Rust-lang community wiki](https://runrust.miraheze.org/wiki/Async_crate_comparison), crates like Tokio that provide both a runtime and IO abstractions often have their IO depend on the runtime. This can make it difficult to write runtime-agnostic code. First, we demonstrate a common use case of async programming in Rust by attempting to fetch the content of a file from the internet using the popular HTTP Client [Reqwest](https://docs.rs/reqwest/0.11.6/reqwest/):

```rust,ignore
use anyhow;

async fn get() -> anyhow::Result<String> {
    let url = "https://link/to/file/download";
    let data = reqwest::get(url).await?.text().await?;
    Ok(data)
}
```

When you try to generate bindings for the `get` function, the generated code will contain errors because this library does not support returning Future from Rust.

## Mismatched runtime
The next logic thing to try would be to convert the asynchronous code to synchronous by directly blocking the current thread and execute the code. For our first attempt, we wrap `futures::executor::block_on` around an async block containing reqwest calls.

```rust,ignore
use anyhow;
use futures::executor::block_on;

fn get() -> anyhow::Result<String> {
    block_on(async {
        let url = "https://link/to/file/download";
        let data = reqwest::get(url).await?.text().await?;
        Ok(data)
    })
}
```

Since Reqwest uses the Tokio runtime instead of the futures runtime, our code panicked with the error "there is no reactor running, must be called from the context of a Tokio 1.x runtime". To fix this error, we have two ways to execute async codes using the Tokio runtime. Approach 1 is the simplest and uses the convenient [`tokio::main`](https://docs.rs/tokio/1.14.0/tokio/attr.main.html) macro to turn an async function to a synchronous one. Approach 2 requires you to explicitly create a new Tokio runtime and use its block_on function to run the future to completion.

## Approach 1 (macro)
```rust,ignore
use anyhow;

#[tokio::main(flavor = "current_thread")]
async fn get() -> anyhow::Result<String> {
    let url = "https://link/to/file/download";
    let data = reqwest::get(url).await?.text().await?;
    Ok(data)
}
```
It has the following dependencies:
```toml
[dependencies]
futures = "0.3"
reqwest = "0.11.6"
tokio = { version = "1.14.0", features = ["rt", "macros"] }
anyhow = { version = "1.0.49" }
```

## Approach 2 (runtime)
```rust,ignore
use anyhow;
use tokio::runtime::Runtime;

fn get() -> anyhow::Result<String> {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let url = "https://link/to/file/download";
        let data = reqwest::get(url).await?.text().await?;
        Ok(data)
    })
}
```
It has the following dependencies:
```toml
[dependencies]
futures = "0.3"
reqwest = "0.11.6"
tokio = { version = "1.14.0", features = ["rt-multi-thread"] }
anyhow = { version = "1.0.49" }
```

## Plain futures
If you are using the plain futures crate without runtimes like Tokio, you should be safe to wrap the asynchronous code in an async block and use the [`futures::executor::block_on`](https://docs.rs/futures/0.3.18/futures/executor/fn.block_on.html) to run the future to completion:

```rust,ignore
use futures::executor::block_on;

async fn hello_world() -> String {
    "hello, world!".to_string()
}

fn get() -> String {
    block_on(async {
        hello_world().await
    })
}

fn main() {
    println!("{}", get()); // prints "hello, world!"
}
```

## Avoid async
Lastly, you can avoid async code all together by using synchronously/blocking version of the functions if they are available. In Reqwest, there's a module called `reqwest::blocking` designed specifically for this purpose. So you can achieve the same thing above without using async.

```rust,ignore
use anyhow;
use reqwest;

fn get() -> anyhow::Result<String> {
    let url = "https://link/to/file/download";
    let data = reqwest::blocking::get(url)?.text()?;
    Ok(data)
}
```
It has the following dependencies:
```toml
[dependencies]
futures = "0.3"
reqwest = { version = "0.11.6", features = ["blocking"] }
anyhow = { version = "1.0.49" }
```

