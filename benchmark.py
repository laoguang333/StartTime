"""
Windows GUI Framework Startup Time Benchmark
Usage: python benchmark.py [--runs N] [--program NAME ...]

Measures startup time by passing an external Unix epoch ms timestamp
via --start-time to each program, then reading the window title to
extract the self-reported elapsed time.
"""

import subprocess
import time
import re
import sys
import ctypes
from ctypes import wintypes
from pathlib import Path
from statistics import mean

user32 = ctypes.windll.user32

WNDENUMPROC = ctypes.WINFUNCTYPE(wintypes.BOOL, wintypes.HWND, wintypes.LPARAM)

ROOT = Path(__file__).parent

PROGRAMS = {
    "Win32 原生":    ROOT / "Win32Demo" / "Win32Demo.exe",
    "Rust nwg":      ROOT / "nwg_demo" / "target" / "release" / "nwg_demo.exe",
    "Rust egui":     ROOT / "egui_demo" / "target" / "release" / "egui_demo.exe",
    ".NET WinForms": ROOT / "bin" / "Release" / "net10.0-windows" / "WinFormDemo.exe",
    "Rust GPUI":     ROOT / "gpui_demo" / "target" / "release" / "gpui_demo.exe",
}

TITLE_PATTERN = re.compile(r'^(WinForm|Win32|NWG|egui|GPUI) Demo - Startup: (\d+) ms$')


def get_window_title(hwnd):
    length = user32.GetWindowTextLengthW(hwnd) + 1
    buffer = ctypes.create_unicode_buffer(length)
    user32.GetWindowTextW(hwnd, buffer, length)
    return buffer.value


def find_startup_window(timeout=30):
    result_hwnd = [None]
    result_ms = [None]

    def callback(hwnd, _):
        title = get_window_title(hwnd)
        m = TITLE_PATTERN.match(title)
        if m:
            result_hwnd[0] = hwnd
            result_ms[0] = int(m.group(2))
            return False
        return True

    enum_proc = WNDENUMPROC(callback)
    deadline = time.time() + timeout

    while time.time() < deadline:
        result_hwnd[0] = None
        result_ms[0] = None
        user32.EnumWindows(enum_proc, 0)
        if result_hwnd[0] is not None:
            return result_hwnd[0], result_ms[0]
        time.sleep(0.05)

    return None, None


def close_window(hwnd):
    WM_CLOSE = 0x0010
    user32.PostMessageW(hwnd, WM_CLOSE, 0, 0)


def run_benchmark(exe_path, runs=5):
    if not exe_path.exists():
        print(f"  [SKIP] 未找到: {exe_path}")
        return []

    results = []
    for r in range(1, runs + 1):
        start_ms = int(time.time() * 1000)
        try:
            proc = subprocess.Popen(
                [str(exe_path), "--start-time", str(start_ms)],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )
        except Exception as e:
            print(f"  [ERR] 启动失败: {e}")
            break

        hwnd, elapsed = find_startup_window(timeout=30)

        if hwnd is None:
            print(f"  [TIMEOUT] Run {r}: 未检测到窗口")
            proc.kill()
            proc.wait(timeout=5)
            break

        # Wait a moment then close
        time.sleep(0.2)
        close_window(hwnd)

        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()
            proc.wait(timeout=5)

        results.append(elapsed)
        print(f"  Run {r}: {elapsed} ms")
        time.sleep(0.5)

    return results


def main():
    runs = 5
    selected = list(PROGRAMS.keys())

    i = 1
    while i < len(sys.argv):
        if sys.argv[i] == "--runs" and i + 1 < len(sys.argv):
            runs = int(sys.argv[i + 1])
            i += 2
        elif sys.argv[i] == "--program" and i + 1 < len(sys.argv):
            selected = [sys.argv[i + 1]]
            i += 2
        else:
            i += 1

    all_results = {}

    for name in selected:
        exe = PROGRAMS.get(name)
        if exe is None:
            # Try matching by substring
            for key, path in PROGRAMS.items():
                if name.lower() in key.lower():
                    exe = path
                    name = key
                    break
            if exe is None:
                print(f"未知程序: {name}, 可用选项: {', '.join(PROGRAMS.keys())}")
                continue

        print(f"\n=== {name} ===")
        results = run_benchmark(exe, runs=runs)
        if results:
            all_results[name] = results

    # Output Markdown table
    if not all_results:
        print("\n没有收集到任何结果。")
        return

    print("\n\n## 基准测试结果")
    print()
    headers = list(all_results.keys())
    print("| 运行次数 | " + " | ".join(headers) + " |")
    print("|---------|" + "|".join("-----------" for _ in headers) + "|")

    max_runs = max(len(v) for v in all_results.values())
    for r in range(max_runs):
        row = [f"Run {r + 1}{' (冷启动)' if r == 0 else ''}"]
        for name in headers:
            vals = all_results[name]
            if r < len(vals):
                row.append(str(vals[r]))
            else:
                row.append("-")
        print("| " + " | ".join(row) + " |")

    avg_row = ["**平均**"]
    for name in headers:
        vals = all_results[name]
        avg_row.append(f"**{mean(vals):.1f}**")
    print("| " + " | ".join(avg_row) + " |")

    min_row = ["**最快**"]
    for name in headers:
        vals = all_results[name]
        min_row.append(f"**{min(vals)}**")
    print("| " + " | ".join(min_row) + " |")

    max_row = ["**最慢**"]
    for name in headers:
        vals = all_results[name]
        max_row.append(f"**{max(vals)}**")
    print("| " + " | ".join(max_row) + " |")


if __name__ == "__main__":
    main()
