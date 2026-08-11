# flutter_rust_bridge 鸿蒙适配完善 PRD

## 1. 文档信息

| 项目 | 内容 |
|---|---|
| 文档状态 | Draft |
| 适用仓库 | `flutter_rust_bridge` |
| 目标平台 | HarmonyOS / OpenHarmony（Flutter OHOS 社区工具链） |
| 目标版本 | 以仓库 CI 固定的 Flutter OHOS、OpenHarmony SDK 和 Rust 工具链为主基线 |
| 产品目标 | 将现有“可生成、可交叉编译、可构建 HAP”的基础支持提升为可验证、可回归、可维护的生产级支持 |

## 2. 背景

flutter_rust_bridge 已具备 OHOS 平台的基础集成能力，包括：

- `create` / `integrate` 命令识别 `ohos` 平台；
- CargoKit app 和 plugin OHOS scaffold；
- Rust `arm64`、`armv7`、`x86_64` 交叉编译目标；
- OHOS clang、linker、sysroot 配置；
- Dart 运行时加载 `lib<crate>.so`；
- CI 中的 OHOS 工程生成检查和 arm64 HAP 构建；
- HarmonyOS/OpenHarmony 环境搭建文档。

当前实现已经打通核心构建链路，并完成一台 HarmonyOS PC 真机上的扩展功能矩阵与连续稳定性验证。专用真机 runner、多个集成形态、工具链兼容策略和发布级质量门禁仍未完成，因此不能把当前状态定义为 100% 生产级适配。

## 3. 产品目标

### 3.1 核心目标

在明确声明的支持范围内，使 OHOS 达到与 Android/iOS 接近的工程可靠性：

1. 用户可以稳定创建或接入 Flutter + Rust OHOS 工程。
2. Rust 动态库能够正确交叉编译并打包进 HAP。
3. 应用能够在真实 OHOS 设备上启动并完成双向 FFI 调用。
4. FRB 主要功能在 OHOS 上具备自动化回归覆盖。
5. OHOS 回归失败能够阻断合并或发布。
6. 支持范围、工具链版本和已知限制有清晰文档。

### 3.2 “100% 适配”的定义

这里的 100% 不表示兼容所有历史 HarmonyOS/OpenHarmony、Flutter OHOS 和 DevEco Studio 版本，而是指：

- 所有“正式声明支持”的平台组合均通过自动化测试；
- 所有 P0 验收项均完成；
- 不存在会导致工程无法生成、无法构建、无法启动或无法调用 Rust 的已知阻断问题；
- 未支持的 ABI、设备形态、Flutter 分支或集成后端被明确标记为“不支持”或“实验性”。

## 4. 非目标

首阶段不承诺：

- 兼容所有第三方 Rust crate；
- 兼容所有历史 OHOS SDK 和非主流 Flutter OHOS fork；
- 替代 Flutter OHOS、DevEco Studio、Hvigor 或 OpenHarmony SDK 自身的问题修复；
- 在没有上游能力的情况下强行支持 Native Assets；
- 首阶段完成 HarmonyOS 应用商店发布、审核和商业签名体系。

## 5. 用户画像与典型场景

### 5.1 新项目开发者

希望通过一条命令创建可运行的 Flutter + Rust OHOS 应用：

```shell
flutter_rust_bridge_codegen create my_app --platforms ohos --skip-fvm-install
```

### 5.2 既有 Flutter OHOS 项目

希望在已有工程中加入 Rust 模块，同时不破坏现有 HarmonyOS scaffold、签名和模块配置。

### 5.3 Flutter plugin 作者

希望发布包含 Rust FFI 的 Flutter plugin，并由 OHOS app 正确编译和打包动态库。

### 5.4 HarmonyOS PC / 多设备开发者

希望明确知道 arm64、x86_64 等目标的支持状态，并在手机、平板、2-in-1 或 PC 场景中获得可复现结果。

## 6. 当前能力盘点

| 能力 | 当前状态 | 说明 |
|---|---|---|
| OHOS 平台识别 | 已完成 | 能检测 OHOS Flutter，并支持显式 `--platforms ohos` |
| app scaffold | 已完成 | CargoKit app 模板包含 OHOS 工程 |
| plugin scaffold | 已实现、验证不足 | 有 OHOS plugin 模板，但缺少完整构建和消费端 CI |
| Rust 交叉编译 | 基本完成 | 声明支持 arm64、armv7、x86_64 |
| Dart 动态库加载 | 已完成 | OHOS 按共享库方式加载 `.so` |
| HAP 构建 | 已扩展、待 CI 验证 | CI 覆盖 create app，并新增 integrate app 与 plugin example arm64 场景 |
| HAP 内容校验 | 已加入本轮优化 | 构建后检查目标 Rust `.so` 是否存在于 HAP |
| 构建环境预检 | 已加入本轮优化 | 构建前检查 OHOS Flutter、native SDK 目录、路径字符和 JDK `jar` |
| CI 工具链固定 | 已加入本轮优化 | Flutter OHOS commit 与 HarmonyOS CLI tools 均使用固定版本 |
| existing app 构建 | 已加入本轮优化、待 CI 验证 | CI 新增 `flutter_via_integrate` OHOS scaffold 生成、arm64 HAP 构建和 `.so` 校验 |
| plugin example 构建 | 已加入本轮优化、待 CI 验证 | CI 新增 `flutter_package/example` arm64 HAP 构建，并从 plugin Rust manifest 校验 `.so` |
| 真机启动 | 本地自动化验证已通过 | HarmonyOS PC 真机已由脚本连续完成安装、启动、日志判定和清理，CI 尚未覆盖 |
| Dart → Rust 调用 | 扩展 smoke test 已通过 | 同步、异步、Stream、Result、结构体/枚举及 64 KiB bytes 已在真机验证 |
| Rust → Dart 回调 | 真机验证已通过 | callback 输入与返回值已纳入确定性结果标记 |
| async / Stream / opaque 等特性 | 真机验证已通过 | 扩展功能矩阵连续运行 20 次无失败，仍需接入专用真机 runner |
| Native Assets | 未确认 | 无 OHOS 专属模板和 CI 结论 |
| SDK 兼容矩阵 | 未建立 | CI 只验证固定工具链提交 |
| 发布级支持声明 | 未完成 | README 和跨平台文档仍主要列出六个平台 |

## 7. 未优化项与需求清单

## 7.1 P0：真机端到端回归

### 问题

当前 OHOS CI 只生成工程和构建 HAP，没有证明 HAP 能安装、启动以及真正调用 Rust。仓库内现有 OHOS Hypium 示例测试只执行普通字符串断言，不覆盖 FRB。

### 需求

建立真实设备或可靠模拟器上的端到端测试：

1. 构建 debug HAP；
2. 对测试 HAP 完成签名；
3. 使用 `hdc` 安装；
4. 启动应用；
5. 执行 FRB smoke test；
6. 收集退出码、设备日志和测试结果；
7. 测试完成后卸载应用并归档日志。

### 最小功能用例

- Dart 调用同步 Rust 函数；
- Dart 调用异步 Rust 函数；
- Rust 返回结构体、枚举、`Result` 和大字节数组；
- Rust Stream 持续向 Dart 推送数据；
- Rust 调用 Dart callback；
- opaque 对象创建、方法调用和释放；
- Rust panic / error 能转换为可诊断的 Dart 错误；
- 连续调用和重复启动不出现动态库加载失败。

### 验收标准

- 以上用例在 arm64 OHOS 真机连续运行 20 次无失败；
- CI 能保存 HAP、测试日志和设备日志；
- 任何用例失败都会让对应质量门禁失败。

### 当前进展

2026-08-11 已在 HUAWEI MateBook Pro（HarmonyOS PC，`arm64-v8a`，API 23，`2in1`）完成端到端扩展 smoke test：使用 Flutter `3.35.8-ohos-0.0.3` 构建 debug HAP，确认包内含 `libs/arm64-v8a/librust_lib_flutter_via_create.so`，随后由脚本完成签名 HAP 安装、`EntryAbility` 启动、按进程收集日志和卸载。同步调用、async、Stream、Rust → Dart callback、opaque 对象修改与释放、可诊断 `Result` 错误转换、结构体/枚举传输和 64 KiB 字节数组均通过，设备日志输出 `FRB_OHOS_SMOKE_RESULT=PASS`。

同一签名 HAP 随后连续运行 20 次，20/20 全部通过；每轮均重新安装、启动、判定结果并自动卸载，独立设备日志保存在测试产物目录。因此，本机 arm64 真机功能矩阵与连续稳定性验收已经完成。受本机 SDK 组件和项目默认签名配置影响，测试时仍临时使用 OpenHarmony API 21 构建配置、`default` 设备类型及已有 DevEco 调试签名材料；这些临时工程修改在测试后均已回退。原 bundle 自动签名、HarmonyOS runtimeOS 构建以及专用 CI 真机 runner 仍是 P0 未完成项。

仓库现已新增 `./frb_internal ohos-device-smoke`，把“检查设备、保护已有 bundle、安装签名 HAP、启动 Ability、按进程采集 hilog、等待 Rust 结果标记、保存日志、自动卸载”固化为可重复命令。quickstart 只有在完整扩展矩阵全部成功后才输出 `FRB_OHOS_SMOKE_RESULT=PASS`。该命令不接管签名凭据，仍需调用方提供已签名且使用专用测试 bundle 的 HAP；接入专用真机 runner 后，才能把当前本地自动化验证升级为合并或发布质量门禁。

## 7.2 P0：集成形态覆盖不足

### 问题

当前 OHOS CI 主要覆盖 `flutter_via_create + CargoKit + app`，没有形成完整组合矩阵。

### 需求

至少覆盖以下场景：

| 场景 | 生成检查 | HAP 构建 | 真机运行 |
|---|---:|---:|---:|
| create app + CargoKit | 必须 | 必须 | 必须 |
| integrate existing app + CargoKit | 必须 | 必须 | 必须 |
| create plugin + CargoKit | 必须 | 必须 | 通过 example app 验证 |
| plugin 被已有 OHOS app 消费 | 必须 | 必须 | 必须 |
| Native Assets | 技术预研 | 技术预研 | 上游支持后决定 |

### 验收标准

- CargoKit 的 app、existing app、plugin 三类场景均纳入 CI；
- `integrate` 不覆盖或删除用户已有 OHOS 配置；
- plugin example HAP 内包含 plugin 对应 Rust 动态库。

### 当前进展

CI 构建矩阵已从单一 `flutter_via_create` 扩展到 `flutter_via_integrate` 和 `flutter_package/example`：existing app 场景会先使用 OHOS Flutter 生成并执行 FRB integrate；plugin 场景会生成 plugin 与 example app 的 OHOS scaffold。二者随后都完成 arm64 release HAP 构建，并按 app 或 plugin 的 Rust manifest 校验 `.so`。新增任务仍需在正式 CI 中首次验证；独立的“已有 OHOS app 消费 plugin”fixture 和用户配置保护 fixture 尚未完成。

## 7.3 P0：工具链版本与 SDK 兼容策略

### 问题

当前 CI 使用固定 Flutter OHOS 提交，用户文档则主要描述通用环境搭建。不同 Flutter OHOS、Dart、DevEco Studio、Hvigor 和 SDK 组合可能出现：

- Dart SDK 下限不匹配；
- `SDK component missing`；
- HarmonyOS 与 OpenHarmony `runtimeOS` 或 SDK 组件不匹配；
- FVM 下载官方 Flutter，覆盖 OHOS fork；
- SDK 路径包含空格或中文导致 Rust 工具链失败。

### 需求

1. 定义一套主支持工具链，并记录精确版本或提交；
2. 定义 N-1 兼容工具链或明确只支持主工具链；
3. 在 codegen/build 前增加环境预检；
4. 预检失败时输出可执行的修复建议；
5. 区分 HarmonyOS SDK、OpenHarmony SDK 和 Rust native SDK 路径；
6. 文档提供 CI 同款环境配置。

### 建议预检项

- `flutter --version` 是否为 OHOS fork；
- Dart 版本是否满足生成工程的 SDK 约束；
- `flutter create --help` 是否包含 `ohos`；
- `OHOS_SDK_HOME` 是否指向包含 `llvm/bin` 和 `sysroot` 的 native 目录；
- JDK、Node、ohpm、hvigor、hdc 是否可执行；
- 工程 `runtimeOS` 与安装的 SDK 组件是否匹配；
- Rust OHOS target 是否已安装；
- SDK 路径是否包含不支持的字符。

### 验收标准

- 支持矩阵文档包含精确版本；
- 常见环境错误在真正编译前被识别；
- CI 主工具链可以从空白 runner 完整安装并通过构建。

### 当前进展

- CI 已固定 Flutter OHOS commit `6d7e5b43fb43bb85ba0a59e3469299ebcf45a637`；
- HarmonyOS command-line tools 已从浮动的 `latest` 固定为 `6.1.1.280`；
- OHOS HAP 构建前已检查 Flutter 是否声明 `ohos`、`OHOS_SDK_HOME` 路径与 native SDK 核心组件，以及可用的 HAP 解包工具；
- HAP 内容检查已支持 `jar` 优先、`unzip` 回退，避免 DevEco 精简 JBR 未包含 `jar` 时误判环境不可用；
- 仍需补齐 Dart 约束、Node/ohpm/hvigor/hdc、`runtimeOS`/SDK 组件以及 Rust target 的统一用户侧预检命令。

## 7.4 P0：HAP 原生库打包完整性

### 问题

单纯执行 `flutter build hap` 成功，不能保证目标 Rust `.so` 已进入 HAP，也不能保证 ABI 正确。

### 需求

- 从 Rust `Cargo.toml` 解析 crate 名称；
- 按 Rust 命名规则计算 `lib<crate>.so`；
- 检查至少生成一个 HAP；
- 检查 HAP 中包含 `libs/arm64-v8a/lib<crate>.so`；
- 缺失时输出 HAP 路径、期望库名和 ABI。

### 当前进展

该需求已在本轮实现，仍需在仓库正式 OHOS CI 环境中完成一次端到端验证。

## 7.5 P1：ABI 与构建模式覆盖

### 问题

代码声明支持 arm64、armv7、x86_64，但 CI 没有证明所有目标都能构建。debug、profile、release 的行为也没有形成验证矩阵。

### 需求

1. arm64 作为正式支持和发布门禁；
2. x86_64 根据 OHOS PC/模拟器需求决定正式支持或实验性支持；
3. armv7 根据 Flutter OHOS 和设备生态现状决定保留、降级或移除；
4. debug 和 release 必须构建；
5. profile 至少完成一次周期性验证；
6. 每个 HAP 校验对应 ABI 的 Rust `.so`。

### 验收标准

- 支持表中每个“正式支持”ABI 都有 CI 证据；
- 未测试 ABI 不再被无条件声明为稳定支持；
- debug/release 均能在目标设备启动并调用 Rust。

## 7.6 P1：Native Assets 支持结论

### 问题

仓库已经提供 Native Assets 后端，但当前没有 OHOS 专属实现或测试结论。

### 需求

进行技术预研并输出 ADR：

- Flutter OHOS 是否支持 Dart hooks/code assets；
- OHOS target 是否能被 Dart Native Assets target model 表达；
- 动态库是否能正确进入 HAP；
- 加载路径和 ABI 是否可解析；
- 上游缺失能力是否可以合理补齐。

### 验收标准

二选一：

- 完成 Native Assets OHOS 实现及 CI；或
- 在 CLI 和文档中明确标记 OHOS 仅支持 CargoKit，并在用户选择不支持组合时提前报错。

## 7.7 P1：错误诊断与开发体验

### 问题

OHOS 构建涉及 Flutter fork、Hvigor、OHPM、SDK、CMake、Clang 和 Cargo，多层错误容易只表现为笼统的构建失败。

### 需求

- 为缺少 `OHOS_SDK_HOME` 提供明确错误；
- 检查 `llvm/bin/clang`、`llvm-ar` 和 `sysroot`；
- 输出当前 Rust target、OHOS ABI、SDK 路径和 Flutter target platform；
- 区分依赖解析失败、SDK 组件缺失、Rust target 缺失、CMake 失败和 HAP 打包缺库；
- 在文档中建立错误码/错误文本到解决方案的映射。

### 验收标准

- P0/P1 常见错误均能在一屏日志内看到原因和下一步操作；
- 不再需要用户通过完整 verbose 日志猜测 SDK 或 ABI 问题。

### 当前进展

构建门禁现在会在执行耗时编译前汇总输出 SDK 路径缺失、路径含空白/非 ASCII 字符、clang/llvm-ar/sysroot 缺失、Flutter 非 OHOS fork 和 JDK `jar` 缺失等错误；通过时输出 SDK、Flutter target、Rust target 和 HAP ABI。其余 Hvigor、依赖解析和 `runtimeOS` 分类诊断仍待实现。

## 7.8 P1：已有工程保护

### 问题

`integrate` 面向真实项目时，用户可能已有签名、产品、模块、权限、混淆和 Hvigor 配置。模板 overlay 必须避免破坏这些内容。

### 需求

- 对已有 OHOS 文件采用可重复执行的增量修改；
- 不删除签名配置、product、module 和用户自定义 Hvigor plugin；
- 冲突时生成明确提示或补丁，而不是静默跳过关键修改；
- 对重复执行 `integrate` 做幂等测试。

### 验收标准

- 同一项目连续执行两次 `integrate` 后第二次无非预期 diff；
- 带自定义签名、权限和多模块配置的 fixture 能通过回归测试；
- 用户文件不会被整文件覆盖。

## 7.9 P1：发布和支持声明

### 问题

README、跨平台概览和安全测试文档仍主要声明 Android、iOS、Windows、Linux、macOS、Web 六个平台，OHOS 的支持级别不统一。

### 需求

- 增加平台支持状态表：Stable / Beta / Experimental / Unsupported；
- OHOS 达到 P0 前标记为 Beta；
- 明确支持的后端、ABI、Flutter OHOS 分支和 SDK；
- changelog 和发布说明列出 OHOS 已知限制；
- 发布流程增加 OHOS smoke build 或 post-release 检查。

### 验收标准

- README、网站、CLI 和发布说明的支持口径一致；
- 用户无需阅读源码即可判断自己的组合是否受支持。

## 7.10 P2：性能、稳定性与资源生命周期

### 需求

- 测量 OHOS 首次加载 Rust 动态库耗时；
- 测量同步、异步、Stream 和大数据传输性能；
- 重复创建/释放 opaque 对象，检测泄漏和崩溃；
- 前后台切换、页面重建、热重载后重复调用；
- 应用退出和重启后的 native 状态恢复；
- 建立与 Android arm64 的基准对比。

### 验收标准

- 连续压力测试无崩溃和明显资源增长；
- 性能回退超过约定阈值时产生 CI 告警；
- 已知平台差异有文档说明。

## 8. 建议测试矩阵

| 维度 | 必测组合 |
|---|---|
| 集成方式 | create app、integrate app、plugin example |
| 后端 | CargoKit；Native Assets 预研后决定 |
| 构建模式 | debug、release |
| ABI | arm64 必测；x86_64 按 PC/模拟器策略；armv7 重新评估 |
| 调用方向 | Dart → Rust、Rust → Dart |
| 调用模型 | sync、async、Stream、callback |
| 数据类型 | primitive、String、struct、enum、Result、bytes、opaque |
| 生命周期 | 正常释放、重复释放保护、应用重启、前后台切换 |
| 工具链 | 主支持版本、N-1 或兼容性基线 |
| 工程类型 | 新建工程、已有工程、自定义签名/权限、多模块工程 |

## 9. 里程碑

### M1：构建链路可信化

- [x] 固定 arm64 release 构建参数；
- [x] HAP 内 Rust `.so` 校验；
- 完成 CI 环境端到端验证；
- [x] 增加首批环境预检和明确错误；

完成标准：生成成功、构建成功和原生库打包成功均有独立门禁。

### M2：真机最小闭环

- 建立 OHOS 真机 runner 或设备测试服务；
- HAP 签名、安装、启动、调用 Rust；
- 覆盖同步、异步、Stream 和 callback smoke test。

完成标准：每个主分支变更都能证明应用在 OHOS 设备上真正运行。

### M3：集成矩阵完善

- existing app；
- plugin + example；
- 幂等和用户配置保护；
- debug/release；
- ABI 支持策略。

完成标准：声明支持的 CargoKit 组合全部进入 CI。

### M4：发布级支持

- Native Assets ADR；
- 性能和稳定性测试；
- 文档口径统一；
- post-release OHOS smoke test；
- OHOS 支持状态从 Beta 提升为 Stable。

## 10. 发布门禁

OHOS Stable 发布至少满足：

- [ ] 主支持工具链可从空白环境安装；
- [ ] create app arm64 debug/release 构建通过；
- [ ] integrate app arm64 debug/release 构建通过；
- [ ] plugin example arm64 debug/release 构建通过；
- [ ] 所有 HAP 包含正确 Rust `.so`；
- [x] 真机 Dart → Rust smoke test 通过（本地自动化）；
- [x] 真机 Rust → Dart callback 通过（本地自动化）；
- [x] async、Stream、opaque、Result 用例通过（本地自动化）；
- [x] 连续 20 次设备回归无失败（本地自动化）；
- [ ] 支持矩阵和已知限制已发布；
- [ ] OHOS 失败会阻断合并或发布；
- [ ] post-release 能验证公开发布产物或公开安装流程。

## 11. 风险与依赖

| 风险 | 影响 | 缓解措施 |
|---|---|---|
| Flutter OHOS 为社区 fork | 上游版本和接口变化频繁 | 固定提交，建立升级验证流程 |
| DevEco/Hvigor/SDK 组合复杂 | 本地可用但 CI 或其他机器失败 | 环境预检、容器化或脚本化安装 |
| 缺少公共 OHOS 真机 runner | 无法做稳定运行回归 | 自托管设备、定时测试或设备服务 |
| HarmonyOS/OpenHarmony 配置差异 | `runtimeOS` 和 SDK 组件不匹配 | 明确产品类型并分别维护 fixture |
| Native Assets 上游能力不足 | 后端无法实现 | 先做 ADR，必要时明确仅支持 CargoKit |
| 多 ABI 增加 CI 成本 | 构建时间和存储增加 | arm64 设强门禁，其他 ABI 周期性验证 |

## 12. 优先执行顺序

建议按以下顺序继续：

1. 在正式 OHOS CI 上验证 HAP 原生库检查；
2. 扩展环境预检，补齐 SDK/runtimeOS、Rust target 与 Node/ohpm/hvigor/hdc 诊断；
3. 将已通过的 arm64 真机 smoke test 接入专用设备 runner；
4. 在正式 CI 验证新增的 existing app 和 plugin example 构建任务，并补独立消费端 fixture；
5. 扩展 panic、长时间 Stream 和资源压力回归；
6. 明确 ABI 和工具链支持矩阵；
7. 完成 Native Assets ADR；
8. 统一 README、网站和发布支持声明。

## 13. 相关实现位置

- OHOS Flutter 检测：`frb_codegen/src/library/commands/flutter.rs`
- 集成模板路由：`frb_codegen/src/library/integration/integrator/overlay.rs`
- CargoKit OHOS app 模板：`frb_codegen/assets/integration_template/cargokit/app/rust_builder/ohos/`
- CargoKit OHOS plugin 模板：`frb_codegen/assets/integration_template/cargokit/plugin/ohos/`
- OHOS 动态库加载：`frb_dart/lib/src/loader/_io.dart`
- OHOS CI：`.github/workflows/ci.yaml`
- CI 任务矩阵：`tools/frb_internal/lib/src/makefile_dart/ci_plan/full_jobs.dart`
- HAP 构建与产物验证：`tools/frb_internal/lib/src/makefile_dart/build.dart`
- HarmonyOS 文档：`website/docs/guides/miscellaneous/harmony-os.md`
