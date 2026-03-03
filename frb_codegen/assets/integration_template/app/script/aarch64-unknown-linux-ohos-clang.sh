#!/bin/sh
exec ${TOOL_HOME}/sdk/default/openharmony/native/llvm/bin/clang \
  -target aarch64-linux-ohos \
  --sysroot=${TOOL_HOME}/sdk/default/openharmony/native/sysroot \
  -D__MUSL__ \
  "$@"
