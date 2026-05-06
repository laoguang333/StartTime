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

## 启动时间结果

各框架运行 5 次，测量值均为毫秒 (ms)。采用**外部统一时钟**（`benchmark.py` 传入 `--start-time`）：

| 运行次数 | Win32 原生 | Rust nwg | Rust egui | .NET WinForms | Rust GPUI |
|---------|-----------|----------|-----------|---------------|-----------|
| Run 1 (冷启动) | 78 | 99 | 160 | 265 | 304 |
| Run 2 | 91 | 117 | 174 | 267 | 348 |
| Run 3 | 96 | 107 | 173 | 266 | 301 |
| Run 4 | 99 | 107 | 168 | 253 | 299 |
| Run 5 | 86 | 118 | 170 | 265 | 299 |
| **平均** | **90.0** | **109.6** | **169.0** | **263.2** | **310.2** |
| **最快** | 78 | 99 | 160 | 253 | 299 |
| **最慢** | 99 | 118 | 174 | 267 | 348 |

## 可执行文件大小

| 框架 | 文件大小 | 启动时间 (新方案) | 启动时间 (旧方案) |
|------|---------|-------------------|-------------------|
| Win32 原生 C++ | 15.0 KB | 90.0 ms | 57.6 ms |
| Rust nwg | 158 KB | 109.6 ms | 60.0 ms |
| .NET WinForms | 162 KB | 263.2 ms | 218.6 ms |
| Rust egui | 4.7 MB | 169.0 ms | 68.4 ms |
| Rust GPUI | **16.9 MB** | 310.2 ms | 521.4 ms |

## 对比分析

```
启动时间 (ms)
0        50       100      150      200      250      300      350
├────────┼────────┼────────┼────────┼────────┼────────┼────────┤
▓ Win32 90.0ms
▓▓ Rust nwg 109.6ms (含 nwg::init())
▓▓▓▓ Rust egui 169.0ms (含 OpenGL 初始化)
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ .NET WinForms 263.2ms (含 CLR+JIT)
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ Rust GPUI 310.2ms (含 Vulkan/GPU)
```

### 关键发现

1. **Win32 原生 C++ 最快** (~90ms) — 零运行时，外部统一时钟下进程创建到窗口就绪的总耗时
2. **Rust nwg 第二** (~110ms) — 修复后含 `nwg::init()` 初始化耗时，较旧方案增加约 50ms
3. **Rust egui 第三** (~169ms) — OpenGL 上下文初始化占大头；wall clock 测量值高于旧的 `Instant` 内部计时
4. **.NET WinForms 较慢** (~263ms) — 外部时钟纳入了 CLR 加载和 JIT 编译，较旧方案增加约 45ms
5. **Rust GPUI 从最慢改善到第四** (~310ms) — 修复 render bug 后不再显示递增时间；GPU 初始化开销仍然存在，但比旧 bug 版本降低了约 200ms

> **注：** 旧方案因各框架的计时起点/终点不一致，存在系统性偏差（nwg 漏了 init，WinForms 漏了 CLR，GPUI render 每帧递增）。新方案统一为外部 wall clock，数据更可靠。

### 体积 vs 启动时间权衡

- Win32 原生体积最小 (15KB)，启动最快，但开发效率低
- Rust nwg 小体积 (158KB) + 快速启动，但生态不活跃
- Rust egui 体积较大 (4.7MB) 但启动快、跨平台
- .NET WinForms 启动较慢但开发效率高，适合业务密集型桌面应用
- Rust GPUI 体积最大 (16.9MB，含 GPU 着色器/字体/SVG/Vulkan)，组件最丰富

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
