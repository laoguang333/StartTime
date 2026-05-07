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

### 8. Rust axum (`axum_demo/`)
- 框架: `axum` v0.8 / `tokio` v1.52（异步 Web 框架）
- 编译: `cargo build --release`
- 输出: `target/release/axum_demo.exe`
- 用途: 作为 Web 框架的基准，衡量 Rust 异步运行时 + TCP 服务器初始化开销
- 注意: 依赖 tokio、hyper、http 等 61 个 crate

## 启动时间结果

各框架运行 5 次，测量值均为毫秒 (ms)。采用**外部统一时钟**（`benchmark.py` 传入 `--start-time`）：

| 运行次数 | Rust CLI | Rust axum | Python CLI | Win32 原生 | Rust nwg | Rust egui | .NET WinForms | Rust GPUI |
|---------|---------|-----------|-----------|-----------|----------|-----------|---------------|-----------|
| Run 1 (冷启动) | 8 | 10 | 21 | 80 | 102 | 182 | 310 | 354 |
| Run 2 | 6 | 8 | 23 | 77 | 86 | 177 | 380 | 367 |
| Run 3 | 7 | 8 | 21 | 79 | 89 | 190 | 297 | 342 |
| Run 4 | 5 | 8 | 22 | 72 | 94 | 179 | 294 | 336 |
| Run 5 | 6 | 9 | 22 | 74 | 95 | 183 | 291 | 357 |
| **平均** | **6.4** | **8.6** | **21.8** | **76.4** | **93.2** | **182.2** | **314.4** | **351.2** |
| **最快** | 5 | 8 | 21 | 72 | 86 | 177 | 291 | 336 |
| **最慢** | 8 | 10 | 23 | 80 | 102 | 190 | 380 | 367 |

## 可执行文件大小

| 框架 | 文件大小 | 启动时间 |
|------|---------|---------|
| Rust CLI（基准） | 160 KB | **6.4 ms** |
| Rust axum（Web 框架基准） | 1.03 MB | **8.6 ms** |
| Python CLI（基准） | —¹ | **21.8 ms** |
| Win32 原生 C++ | 15.0 KB | **76.4 ms** |
| Rust nwg | 158 KB | **93.2 ms** |
| Rust egui | 4.7 MB | **182.2 ms** |
| .NET WinForms | 162 KB | **314.4 ms** |
| Rust GPUI² | **8.3 MB** | **351.2 ms** |

> ¹ Python CLI 为 `.py` 脚本，无独立可执行文件；需依赖系统安装的 `python.exe`

## 对比分析

```
启动时间 (ms)
0        50       100      150      200      250      300      350      400
├────────┼────────┼────────┼────────┼────────┼────────┼────────┼────────┤
▓ Rust CLI 6.4ms
▓▓ Rust axum 8.6ms
▓▓▓▓▓▓ Python CLI 21.8ms
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ Win32 76.4ms
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ Rust nwg 93.2ms
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ Rust egui 182.2ms
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ .NET WinForms 314.4ms
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ Rust GPUI 351.2ms
```

### 关键发现

1. **Rust CLI 最快 (~6ms)** — 纯 Rust 运行时本身的启动开销（DLL 加载、CRT/Rust 运行时初始化），作为编译型语言的基准线
2. **Rust axum (~9ms)** — 仅比裸 Rust CLI 多 ~2ms！tokio 异步运行时 + TCP 绑定的开销极小，二进制体积增长来自静态链接的 tokio/hyper/http 等依赖
3. **Python CLI (~22ms)** — Python 解释器开销，比 Rust CLI 慢约 3.5 倍（解释器加载 + 字节码编译 + 运行时初始化）
4. **Win32 原生 C++ (~76ms)** — 零运行时框架，主要开销为 Win32 API 窗口创建和系统渲染
5. **Rust nwg (~93ms)** — Win32 薄封装，含 `nwg::init()` 初始化
6. **Rust egui (~182ms)** — OpenGL 上下文初始化占大头
7. **.NET WinForms (~314ms)** — CLR 加载和 JIT 编译是主要开销，比 Rust 运行时重约 49 倍
8. **Rust GPUI (~351ms)** — GPU 初始化（Vulkan/blade-graphics）、字体引擎、SVG 渲染

> **注：** 本次冷/热启动差异较小（可能与系统缓存状态有关）。旧方案因各框架计时起点/终点不一致存在系统性偏差。

### 框架开销分解

以 Rust CLI（6.4ms）作为编译型语言基准线，Python CLI（21.8ms）作为解释型语言参照：

| 框架 | 总启动时间 | vs Rust CLI | 主要原因 |
|------|-----------|------------|---------|
| Rust CLI | 6.4 ms | — | 基准 |
| Rust axum | 8.6 ms | +2.2 ms | tokio 运行时 + TCP 绑定 |
| Python CLI | 21.8 ms | +15.4 ms | 解释器加载、字节码编译 |
| Win32 原生 C++ | 76.4 ms | +70.0 ms | 窗口创建、系统渲染 |
| Rust nwg | 93.2 ms | +86.8 ms | 窗口创建 + nwg::init() |
| Rust egui | 182.2 ms | +175.8 ms | OpenGL 初始化 + 第一帧渲染 |
| .NET WinForms | 314.4 ms | +308.0 ms | CLR 加载 + JIT + WinForms 初始化 |
| Rust GPUI | 351.2 ms | +344.8 ms | Vulkan 初始化 + 字体引擎 + SVG 渲染 |

### 体积 vs 启动时间权衡

| 框架 | 文件大小 | 启动时间 | 类型 |
|------|---------|---------|------|
| Rust CLI | 160 KB | **6.4 ms** | 编译型基准 |
| Rust axum | **1.03 MB** | **8.6 ms** | Web 框架基准 |
| Python CLI | — | **21.8 ms** | 解释型基准 |
| Win32 原生 C++ | 15 KB | **76.4 ms** | 原生 GUI |
| Rust nwg | 158 KB | **93.2 ms** | Win32 封装 |
| Rust egui | 4.7 MB | **182.2 ms** | 即时模式 GUI |
| .NET WinForms | 162 KB | **314.4 ms** | 托管 GUI |
| Rust GPUI² | **8.3 MB** | **351.2 ms** | GPU 加速 GUI |

> ² GPUI 通过 `default-features=false`（去除 wayland/x11）+ `lto=true` + `codegen-units=1` + `strip=true` + `opt-level="z"` 从 16.9 MB 优化至 8.3 MB（-51%）。启动时间不受影响。

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
| Rust axum | `./axum_demo/` |
