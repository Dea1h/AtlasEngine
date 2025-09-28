mod server;

use server::websocket;

fn main() {
    websocket::fetch_market_metrics();
}
