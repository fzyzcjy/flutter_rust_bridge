# Flutter + Rust for HarmonyOS (OpenHarmony) 示例项目

使用 `flutter_rust_bridge` 的鸿蒙适配分支，在 HarmonyOS 上实现高性能 Rust + Flutter 混合开发。

**重要提醒**：本方案目前**仅在 Windows + x86 架构虚拟机** 上完成基本测试通过。其他平台（Mac、Linux、真机等）尚未充分验证。

## 环境要求

- Windows 10 / 11（推荐 64 位专业版或以上）
- 已安装 [DevEco Studio](https://developer.huawei.com/consumer/cn/deveco-studio/)（建议 Release 稳定版，非 Beta）
- 已安装 [Rust](https://rust-lang.org/tools/install/)（通过 rustup 安装最新稳定版）
- 已安装 [Flutter（使用鸿蒙适配版本， dependency_overrides 方式）](https://gitcode.com/openharmony-tpc/flutter_flutter)

注意：**如果你希望适配安卓等其他平台，Flutter请使用3.35.7版本**，貌似是gradle版本问题，用低版本编译安卓平台会报错，当然你也可以试试用3.27等版本手动升级gradle试试水，毕竟我也没试过。
## 快速开始 - 环境配置步骤

### 1. 安装 flutter_rust_bridge_codegen（鸿蒙专用 fork）

```bash
# 强烈建议使用 --git 方式安装此 fork 版本
cargo install flutter_rust_bridge_codegen --git https://github.com/star4277/flutter_rust_bridge_ohos.git

# 安装完成后，可运行以下命令检查是否成功：
flutter_rust_bridge_codegen --version
```

### 2. 设置鸿蒙 SDK 路径环境变量（非常重要！）

鸿蒙原生 SDK 路径必须指向 native 目录，且路径中**严禁出现空格、中文、特殊字符**，否则后续构建脚本会报各种莫名其妙的错误。
推荐做法（也是目前最稳定的方式）。下方的OpenHarmony sdk主要是为了演示，正式使用时sdk尽量不要用openharmony的sdk，因为就我目前的测试来看，openharmony的sdk貌似会报错少东西

打开 DevEco Studio
创建一个空的 HarmonyOS 项目（或打开已有项目）
让 DevEco Studio 自动下载并安装 SDK（通常在默认路径）
找到实际 SDK 路径，一般类似于：
```text
C:\Users\你的用户名\HarmonyOS\sdk\default\openharmony\native
```
或
```text
C:\ProgramFiles\Huawei\DevEcoStudio\sdk\default\openharmony\native
```
将此 native 路径 设置为环境变量 OHOS_SDK_HOME 
Windows 设置方式：
```text
右键「此电脑」→ 属性 → 高级系统设置 → 环境变量
在「用户变量」或「系统变量」中新建：text变量名：OHOS_SDK_HOME
变量值：C:\Users\xxx\HarmonyOS\sdk\default\openharmony\native （替换成你真实的路径）
```

注意：不要指向 sdk 根目录，必须精确到 native 文件夹！
已知问题：直接使用 DevEco Studio 下载的 OpenHarmony SDK 在部分场景下可能无法正常被 rust 工具链识别，建议优先使用 DevEco
Studio 自己选择性下载的版本。

### 3. 创建一个测试项目

```Bash# 在你想要存放代码的目录下执行
flutter_rust_bridge_codegen create myapp
```
创建完成后会生成类似下面的结构：
```text
myapp/
├── rust/               # Rust 代码放这里
├── lib/                # Dart 代码
├── ohos/               # 鸿蒙相关（视 fork 版本而定）
├── pubspec.yaml
└── ...
```

### 4. 在 DevEco Studio 中打开鸿蒙模块并签名

用 DevEco Studio 打开项目根目录（或 ohos 目录，如果有独立模块）
选择鸿蒙模块（通常名为 entry 或 ohos）
配置签名：
点击右上角「构建」→「编辑签名配置」
创建或使用已有签名证书
建议使用默认的调试签名（Debug Profile）

### 5. 运行与调试

在 DevEco Studio 中选择 x86 架构的模拟器
点击运行（绿色三角）

目前已知：x86 架构的鸿蒙虚拟机可以正常运行，arm64 虚拟机和真机尚未充分验证。
已知限制与注意事项

仅 Windows 平台经过测试（截至 2026年3月）
路径中不能有中文、空格（包括用户名目录建议用纯英文）
Rust 交叉编译目标需包含 aarch64-unknown-linux-ohos（部分 fork 已自动处理）
如果遇到 .so 加载失败，可尝试手动指定 DynamicLibrary.open("libxxx.so")
后续可能需要额外配置 build-profile.json5 或 hvigor 脚本

### 参考与感谢
flutter_rust_bridge 官方文档：https://cjycode.com/flutter_rust_bridge/

HarmonyOS 开发者站：https://developer.huawei.com/consumer/cn/harmonyos/

欢迎提交 Issue / PR，一起完善鸿蒙上的 Flutter + Rust 生态！