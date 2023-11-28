# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(rust_builder_library VERSION 0.0.1 LANGUAGES C)

add_library(rust_builder SHARED
  "rust_builder.c"
)

set_target_properties(rust_builder PROPERTIES
  PUBLIC_HEADER rust_builder.h
  OUTPUT_NAME "rust_builder"
)

target_compile_definitions(rust_builder PUBLIC DART_SHARED_LIB)
