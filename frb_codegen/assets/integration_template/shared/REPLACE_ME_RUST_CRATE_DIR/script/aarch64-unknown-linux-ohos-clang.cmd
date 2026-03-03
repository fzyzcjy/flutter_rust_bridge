@echo off
"D:\ProgramFiles\DeveloperToolKit\Jetbrains\Huawei\DevEco Studio\sdk\default\openharmony\native\llvm\bin\clang.exe" -target aarch64-linux-ohos  --sysroot="D:\ProgramFiles\DeveloperToolKit\Jetbrains\Huawei\DevEco Studio\sdk\default\openharmony\native\sysroot" -D__MUSL__ %*
