mod server;

use server::websocket;

fn main() {
    let _ = websocket::fetch_market_metrics();
}
