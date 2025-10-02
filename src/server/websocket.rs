/*
* Websocket For Getting Realtime Info from Trading Platforms.
* Trading Platforms Are:
* Binance
*/
#![allow(warnings)]

#[derive(Debug, serde::Deserialize)]
struct Ticker {
    #[serde(rename = "e")]
    event_type: String, // Event type
    #[serde(rename = "E")]
    event_time: u64, // Event time (timestamp)
    #[serde(rename = "s")]
    symbol: String, // Symbol
    #[serde(rename = "p")]
    price_change: String, // Price change
    #[serde(rename = "P")]
    price_change_percentage: String, // Price change percent
    #[serde(rename = "w")]
    weighted_average_price: String, // Weighted average price
    #[serde(rename = "x")]
    previous_close: String, // First trade(F)-1 price (previous close)
    #[serde(rename = "c")]
    last_price: String, // Last price
    #[serde(rename = "Q")]
    last_quantity: String, // Last quantity
    #[serde(rename = "b")]
    best_bid_price: String, // Best bid price
    #[serde(rename = "B")]
    best_bid_quantity: String, // Best bid quantity
    #[serde(rename = "a")]
    best_ask_price: String, // Best ask price
    #[serde(rename = "A")]
    best_ask_quantity: String, // Best ask quantity
    #[serde(rename = "o")]
    open_price: String, // Open price
    #[serde(rename = "h")]
    high_price: String, // High price
    #[serde(rename = "l")]
    low_price: String, // Low price
    #[serde(rename = "v")]
    total_traded_base_asset_volume: String, // Total traded base asset volume
    #[serde(rename = "q")]
    total_traded_quote_asset_volume: String, // Total traded quote asset volume
    #[serde(rename = "O")]
    statistics_open_time: u64, // Statistics open time
    #[serde(rename = "C")]
    statistics_close_time: u64, // Statistics close time
    #[serde(rename = "F")]
    first_trade_id: u64, // First trade ID
    #[serde(rename = "L")]
    last_trade_id: u64, // Last trade Id
    #[serde(rename = "n")]
    total_number_trades: u64, // Total number of trades
}

#[derive(Debug, serde::Deserialize)]
struct Trade {
    #[serde(rename = "e")]
    event_type: String, // Event type
    #[serde(rename = "E")]
    event_time: u64, // Event Time
    #[serde(rename = "s")]
    symbol: String, // Symbol
    #[serde(rename = "t")]
    trade_id: u64, // Trade ID
    #[serde(rename = "p")]
    price: String, // Price
    #[serde(rename = "q")]
    quantity: String, // Quantity
    #[serde(rename = "T")]
    trade_time: u64, // Trade Time
    #[serde(rename = "m")]
    market_maker: bool, // Is Market Maker?
    #[serde(rename = "M")]
    ignore: bool, // Ignore (Always True)
}

#[derive(Debug, serde::Deserialize)]
struct Stream {
    #[serde(rename = "stream")]
    stream: String, // Stream Name
    #[serde(rename = "data")]
    data: Trade, // Stream Data
}

pub fn fetch_market_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let host: String = match std::env::var("TRADING_HOSTNAME") {
        Ok(val) => {
            println!("HOSTNAME: {}", val);
            val
        }
        Err(e) => {
            eprintln!("Error: HOSTNAME NOT FOUND FOR TRADING PLATFORM. {}", e);
            // "wss://testnet.binancefuture.com/ws-fapi/v1".to_string()
            // "wss://stream.binance.com:9443/ws/btcusdt@trade".to_string()
            // "wss://testnet.binancefuture.com/ws-fapi/v1/btcusdt@trade".to_string()
            "wss://stream.binance.com:9443/stream?streams=btcusdt@trade/ethusdt@trade/bnbusdt@trade"
                .to_string()
        }
    };

    if let Err(e) = url::Url::parse(&host) {
        eprintln!("Error: Host Is Not Valid: {}", e);
        return Err("490".into());
    }

    let (mut socket, response) = match tungstenite::connect(&host) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error: WEBSOCKET CONNECTION FAILED: {}", e);
            return Err("Error: 410".into());
        }
    };
    println!(
        "Connected to Binance, HTTP response code: {}",
        response.status()
    );

    loop {
        match socket.read() {
            Ok(msg) => match msg {
                tungstenite::Message::Text(json) => match serde_json::from_str::<Stream>(&json) {
                    Ok(stream) => {
                        println!("{}:{}", stream.stream, stream.data.price);
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
    return Ok(());
}
