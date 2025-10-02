/*
* Websocket For Getting Realtime Info from Trading Platforms.
* Trading Platforms Are:
* Binance
*/

#[derive(Debug, serde::Deserialize)]
struct Trade {
    event_type: String,                      // Event type
    event_time: u32,                         // Event time (timestamp)
    symbol: String,                          // Symbol
    price_change: String,                    // Price change
    price_change_percentage: String,         // Price change percent
    weighted_average_price: String,          // Weighted average price
    previous_close: String,                  // First trade(F)-1 price (previous close)
    last_price: String,                      // Last price
    last_quantity: String,                   // Last quantity
    best_bid_price: String,                  // Best bid price
    best_bid_quantity: String,               // Best bid quantity
    best_ask_price: String,                  // Best ask price
    best_ask_quantity: String,               // Best ask quantity
    open_price: String,                      // Open price
    high_price: String,                      // High price
    low_price: String,                       // Low price
    total_traded_base_asset_volume: String,  // Total traded base asset volume
    total_traded_quote_asset_volume: String, // Total traded quote asset volume
    statistics_open_time: u32,               // Statistics open time
    statistics_close_time: u32,              // Statistics close time
    first_trade_id: u32,                     // First trade ID
    last_trade_id: u32,                      // Last trade Id
    total_number_trades: u32,                // Total number of trades
}

pub fn fetch_market_metrics() {
    let host: String = match std::env::var("TRADING_HOSTNAME") {
        Ok(val) => {
            println!("HOSTNAME: {}", val);
            val
        }
        Err(e) => {
            eprintln!("Error: HOSTNAME NOT FOUND FOR TRADING PLATFORM. {}", e);
            "wss://stream.binance.com:9443/ws/btcusdt@ticker".to_string()
            // "wss://testnet.binancefuture.com/ws-fapi/v1/btcusdt@trade".to_string()
        }
    };

    if let Err(e) = url::Url::parse(&host) {
        eprintln!("Error: Host Is Not Valid: {}", e);
        return;
    }

    let (mut socket, response) = match tungstenite::connect(&host) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error: WEBSOCKET CONNECTION FAILED: {}", e);
            return;
        }
    };
    println!(
        "Connected to Binance, HTTP response code: {}",
        response.status()
    );

    loop {
        match socket.read() {
            Ok(msg) => match msg {
                tungstenite::Message::Text(json) => match serde_json::from_str::<Trade>(&json) {
                    Ok(trade) => {
                        println!("{}", trade.last_price);
                    }
                    Err(e) => {
                        eprintln!("Error: Parsing Json: {}", e);
                    }
                },
                tungstenite::Message::Ping(p) => {
                    println!("Ping From Server");
                    socket
                        .write(tungstenite::Message::Pong(p))
                        .expect("Error: Failed To Write To Server");
                }
                tungstenite::Message::Pong(_) => {
                    println!("Pong Received");
                }
                tungstenite::Message::Close(c) => {
                    println!("Connection Closed: {:?}", c);
                    break;
                }
                _ => {}
            },
            Err(e) => {
                eprintln!("Error: Failed To Receive Message From Server, {}", e);
                break;
            }
        };
    }
    return;
}
