/*
* Websocket For Getting Realtime Info from Trading Platforms.
* Trading Platforms Are:
* Binance
*/

pub fn fetch_market_metrics() {
    let host: String = match std::env::var("TRADING_HOSTNAME") {
        Ok(val) => {
            println!("HOSTNAME: {}", val);
            val
        }
        Err(e) => {
            eprintln!("Error: HOSTNAME NOT FOUND FOR TRADING PLATFORM. {}", e);
            "wss://stream.binance.com:9443/ws/btcusdt@tricker".to_string()
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
                tungstenite::Message::Text(txt) => println!("Received: {}", txt),
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
