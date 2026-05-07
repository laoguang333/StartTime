use std::time::{SystemTime, UNIX_EPOCH};

fn get_now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut start_time_ms: u128 = 0;
    for i in 1..args.len().saturating_sub(1) {
        if args[i] == "--start-time" {
            start_time_ms = args[i + 1].parse().unwrap_or(0);
            break;
        }
    }
    if start_time_ms == 0 {
        start_time_ms = get_now_ms();
    }

    let app = axum::Router::new();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();

    let elapsed_ms = get_now_ms() - start_time_ms;
    println!("axum Demo - Startup: {} ms", elapsed_ms);

    axum::serve(listener, app).await.unwrap();
}
