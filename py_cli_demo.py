import sys
import time

def get_now_ms():
    return int(time.time() * 1000)

def main():
    start_time_ms = 0
    args = sys.argv[1:]
    for i in range(len(args) - 1):
        if args[i] == "--start-time":
            try:
                start_time_ms = int(args[i + 1])
            except ValueError:
                pass
            break

    if start_time_ms == 0:
        start_time_ms = get_now_ms()

    elapsed = get_now_ms() - start_time_ms
    print(f"Python CLI Demo - Startup: {elapsed} ms")

if __name__ == "__main__":
    main()
