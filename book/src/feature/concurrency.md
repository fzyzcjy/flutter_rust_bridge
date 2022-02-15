# Concurrency

Multiple Rust functions can be running at the same time, and they will be running concurrently. This is because by default we use a thread pool to execute the Rust functions. However, you can fully customize this behavior (and even throw away the thread pool).