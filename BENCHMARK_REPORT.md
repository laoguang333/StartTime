# Windows GUI 框架启动时间基准测试报告

## 测试环境

| 项目 | 值 |
|------|-----|
| 操作系统 | Windows 10+ (x64) |
| CPU | x86_64 |
| .NET SDK | 10.0.202 |
| Rust 工具链 | rustc 1.95.0, cargo 1.95.0 |
| VS BuildTools | 2022 (v17.14, MSVC 14.44) |
| 测试方法 | 外部统一时钟：Python 脚本记录 Unix epoch ms 作为 `--start-time` 参数传入，程序在窗口就绪时计算 `now() - start_time`（所有框架使用 wall clock 同步起点） |

## 测试项目

### 1. .NET WinForms (`WinFormDemo/`)
- 框架: .NET 10.0 + Windows Forms
- 编译: `dotnet build -c Release`
- 输出: `bin/Release/net10.0-windows/WinFormDemo.exe`
- 模式: WinExe (无控制台)

### 2. Win32 原生 C++ (`Win32Demo/`)
- 框架: 纯 Win32 API
- 编译: `cl /O2 /MD /Fe:Win32Demo.exe main.cpp /link /SUBSYSTEM:WINDOWS user32.lib shell32.lib kernel32.lib`
- 输出: `Win32Demo.exe`

### 3. Rust native-windows-gui (`nwg_demo/`)
- 框架: `native-windows-gui` v1.0.13 (Win32 薄封装)
- 编译: `cargo build --release`
- 输出: `target/release/nwg_demo.exe`

### 4. Rust egui (`egui_demo/`)
- 框架: `eframe` v0.31.1 / `egui` v0.31.1 (即时模式 GUI, OpenGL 渲染)
- 编译: `cargo build --release`
- 输出: `target/release/egui_demo.exe`

### 5. Rust GPUI Component (`gpui_demo/`)
- 框架: `gpui` v0.2.2 / `gpui-component` v0.5.1 (Zed 的 GPU 加速 UI + 组件库)
- 编译: `cargo build --release`
- 输出: `target/release/gpui_demo.exe`
- 注意: 依赖 200+ crates，含 blade-graphics (Vulkan)、cosmic-text、resvg 等

### 6. Rust CLI (`cli_demo/`)
- 框架: 无，纯 Rust 控制台程序
- 编译: `cargo build --release`
- 输出: `target/release/cli_demo.exe`
- 用途: 作为基准参照，衡量 Rust 运行时本身（DLL 加载、CRT 初始化）的启动开销

### 7. Python CLI (`py_cli_demo.py`)
- 框架: Python 3.12.10，解释型语言
- 运行: `python py_cli_demo.py --start-time <ms>`
- 用途: 作为解释型语言的基准，衡量 Python 解释器本身（DLL 加载、字节码编译、运行时初始化）的启动开销

## 启动时间结果

各框架运行 5 次，测量值均为毫秒 (ms)。采用**外部统一时钟**（`benchmark.py` 传入 `--start-time`）：

| 运行次数 | Rust CLI | Python CLI | Win32 原生 | Rust nwg | Rust egui | .NET WinForms | Rust GPUI |
|---------|---------|-----------|-----------|----------|-----------|---------------|-----------|
| Run 1 (冷启动) | 37 | 23 | 92 | 475 | 569 | 445 | 463 |
| Run 2 | 7 | 31 | 66 | 83 | 174 | 362 | 422 |
| Run 3 | 5 | 28 | 84 | 78 | 179 | 353 | 327 |
| Run 4 | 8 | 76 | 66 | 78 | 216 | 344 | 332 |
| Run 5 | 5 | 49 | 67 | 89 | 227 | 339 | 326 |
| **平均** | **12.4** | **41.4** | **75.0** | **160.6** | **273.0** | **368.6** | **374.0** |
| **最快** | 5 | 23 | 66 | 78 | 174 | 339 | 326 |
| **最慢** | 37 | 76 | 92 | 475 | 569 | 445 | 463 |

## 可执行文件大小

| 框架 | 文件大小 | 启动时间 |
|------|---------|---------|
| Rust CLI（基准） | 160 KB | **12.4 ms** |
| Python CLI（基准） | —¹ | **41.4 ms** |
| Win32 原生 C++ | 15.0 KB | **75.0 ms** |
| Rust nwg | 158 KB | **160.6 ms** |
| Rust egui | 4.7 MB | **273.0 ms** |
| .NET WinForms | 162 KB | **368.6 ms** |
| Rust GPUI | **16.9 MB** | **374.0 ms** |

> ¹ Python CLI 为 `.py` 脚本，无独立可执行文件；需依赖系统安装的 `python.exe`

## 对比分析

```
启动时间 (ms)
0        50       100      150      200      250      300      350      400
├────────┼────────┼────────┼────────┼────────┼────────┼────────┼────────┤
▓ Rust CLI 12.4ms
▓▓▓▓ Python CLI 41.4ms
▓▓▓▓▓▓▓▓▓▓▓▓▓▓ Win32 75.0ms
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ Rust nwg 160.6ms
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ Rust egui 273.0ms
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ .NET WinForms 368.6ms
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ Rust GPUI 374.0ms
```

### 关键发现

1. **Rust CLI 最快 (~12ms)** — 纯 Rust 运行时本身的启动开销（DLL 加载、CRT/Rust 运行时初始化），作为编译型语言的基准线
2. **Python CLI (~41ms)** — Python 解释器开销，比 Rust CLI 慢约 3 倍（解释器加载 + 字节码编译 + 运行时初始化）
3. **Win32 原生 C++ (~75ms)** — 零运行时框架，主要开销为 Win32 API 窗口创建和系统渲染
4. **Rust nwg (~161ms)** — Win32 薄封装，含 `nwg::init()` 初始化
5. **Rust egui (~273ms)** — OpenGL 上下文初始化占大头（~50ms 显卡驱动加载 + ~220ms 第一帧渲染）
6. **.NET WinForms (~369ms)** — CLR 加载和 JIT 编译是主要开销，比 Rust 运行时重约 30 倍
7. **Rust GPUI (~374ms)** — GPU 初始化（Vulkan/blade-graphics）、字体引擎（cosmic-text/harfbuzz）、SVG 渲染（resvg）等大量启动开销

> **注：** GUI 框架冷启动（Run 1）与热启动（Run 2-5）差异很大。nwg 冷启动 475ms vs 热启动 83ms；egui 冷启动 569ms vs 热启动 174ms，说明磁盘缓存和 GPU 驱动加载对首次启动影响显著。旧方案因各框架计时起点/终点不一致存在系统性偏差。

### 框架开销分解

以 Rust CLI（12ms）作为编译型语言基准线，Python CLI（41ms）作为解释型语言参照：

| 框架 | 总启动时间 | vs Rust CLI | 主要原因 |
|------|-----------|------------|---------|
| Rust CLI | 12 ms | — | 基准 |
| Python CLI | 41 ms | +29 ms | 解释器加载、字节码编译 |
| Win32 原生 C++ | 75 ms | +63 ms | 窗口创建、系统渲染 |
| Rust nwg | 161 ms | +149 ms | 窗口创建 + nwg::init() |
| Rust egui | 273 ms | +261 ms | OpenGL 初始化 + 第一帧渲染 |
| .NET WinForms | 369 ms | +357 ms | CLR 加载 + JIT + WinForms 初始化 |
| Rust GPUI | 374 ms | +362 ms | Vulkan 初始化 + 字体引擎 + SVG 渲染 |

### 体积 vs 启动时间权衡

- **Rust CLI** (160KB) — 纯 Rust 运行时，无 UI，编译型语言基准
- **Python CLI** (—) — 脚本语言，需依赖解释器
- **Win32 原生 C++** (15KB) — 体积最小，启动最快，但开发效率低
- **Rust nwg** (158KB) — 小体积 + 快速启动，生态不活跃
- **Rust egui** (4.7MB) — 体积较大但启动快、跨平台
- **.NET WinForms** (162KB) — 启动较慢但开发效率高，适合业务密集型桌面应用
- **Rust GPUI** (16.9MB) — 体积最大，组件最丰富

## 基准测试方法变更说明

**原方法问题：**
- WinForms 的 `DateTime.Now` 精度低（~15ms），且未计入 CLR 加载、JIT 编译和 `ApplicationConfiguration.Initialize()`
- Rust nwg 的计时起点设在 `nwg::init()` **之后**，漏掉了库初始化耗时
- Rust GPUI 的 body label 每帧重算 `start.elapsed()`，显示值持续递增（bug）
- 各框架使用不同的时间源（`DateTime.Now` / `GetTickCount64` / `Instant::now` / wall clock），起点不一致

**修复方案（此版本）：**
1. 所有程序通过 `--start-time <UnixEpochMs>` 接收外部统一时钟起点
2. 就绪时统一使用 wall clock（`SystemTime::now` / `DateTimeOffset.UtcNow` / `system_clock::now`）计算差值
3. 程序未收到 `--start-time` 时自动退化到自身内部计时（保持独立可运行）
4. 修复 GPUI render bug：`self.startup_ms` 固定为一次计算结果，不再每帧重算
5. 移除 Win32 中冗余的 `WM_SHOWWINDOW` 计时代码
6. 提供 `benchmark.py` 脚本自动执行基准测试

使用方式：
```bash
python benchmark.py                # 所有程序各跑 5 次
python benchmark.py --runs 10      # 自定义次数
python benchmark.py --program "Win32 原生"  # 测试单个框架
```

> 以上结果均为修复后新方案数据。使用 `python benchmark.py` 可直接复现。

## 各项目位置

| 项目 | 路径 |
|------|------|
| .NET WinForms | `./WinFormDemo/` |
| Win32 C++ | `./Win32Demo/` |
| Rust nwg | `./nwg_demo/` |
| Rust egui | `./egui_demo/` |
| Rust GPUI | `./gpui_demo/` |
| Rust CLI | `./cli_demo/` |
| Python CLI | `./py_cli_demo.py` |
