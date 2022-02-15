# Zero copy

`ZeroCopyBuffer<Vec<u8>>` (and its friends like `ZeroCopyBuffer<Vec<i8>>`) sends the data from Rust to Dart without making copies. Thus, you save the time of copying data, which can be large if your data is big (such as a high-resolution image).