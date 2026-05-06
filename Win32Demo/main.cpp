#define UNICODE
#define _UNICODE
#include <windows.h>
#include <shellapi.h>
#include <stdio.h>
#include <chrono>

using namespace std::chrono;

LRESULT CALLBACK WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam)
{
    switch (msg)
    {
    case WM_DESTROY:
        PostQuitMessage(0);
        return 0;
    }
    return DefWindowProc(hwnd, msg, wParam, lParam);
}

long long get_now_ms()
{
    return duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
}

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow)
{
    long long startTimeMs = 0;

    int argc;
    LPWSTR* argv = CommandLineToArgvW(GetCommandLineW(), &argc);
    if (argv)
    {
        for (int i = 1; i < argc - 1; i++)
        {
            if (wcscmp(argv[i], L"--start-time") == 0)
            {
                startTimeMs = _wtoi64(argv[i + 1]);
                break;
            }
        }
        LocalFree(argv);
    }

    if (startTimeMs == 0)
        startTimeMs = get_now_ms();

    const wchar_t CLASS_NAME[] = L"Win32DemoWindow";

    WNDCLASS wc = {};
    wc.lpfnWndProc = WndProc;
    wc.hInstance = hInstance;
    wc.lpszClassName = CLASS_NAME;

    RegisterClass(&wc);

    HWND hwnd = CreateWindowEx(
        0, CLASS_NAME, L"Win32 Demo",
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT, CW_USEDEFAULT, 800, 450,
        NULL, NULL, hInstance, NULL
    );

    if (!hwnd) return 0;

    ShowWindow(hwnd, nCmdShow);
    UpdateWindow(hwnd);

    long long elapsed = get_now_ms() - startTimeMs;

    wchar_t title[128];
    swprintf_s(title, L"Win32 Demo - Startup: %lld ms", elapsed);
    SetWindowText(hwnd, title);

    MSG msg = {};
    while (GetMessage(&msg, NULL, 0, 0))
    {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }
    return 0;
}
