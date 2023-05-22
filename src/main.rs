mod models;

use std::thread;
use tungstenite::connect;
use url::Url;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

fn main() {
    let handle1 = thread::spawn(|| {
        binance_url_ethusdt();
    });

    let handle2 = thread::spawn(|| {
        binance_url_btcusdt();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn binance_url_ethusdt() {
    let binance_url_ethusdt = format!("{}/ws/ethusdt@kline_1m", BINANCE_WS_API);
    let (mut socket, response) =
        connect(Url::parse(&binance_url_ethusdt).unwrap()).expect("Can't connect.");

    println!("Connected to binance stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }
    println!("______________________________________________________________________");

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                panic!("Error getting text");
            }
        };

        let parsed: models::MyData = serde_json::from_str(&msg).expect("Can't parse");
        let sum = parsed.k.o.parse::<f64>().unwrap()
            + parsed.k.c.parse::<f64>().unwrap()
            + parsed.k.h.parse::<f64>().unwrap()
            + parsed.k.l.parse::<f64>().unwrap();

        // let data: models::MyData = serde_json::from_str(parsed_data).unwrap();

        println!("{:?}", parsed);
        println!("sum: {:?}", sum);
        println!("______________________________________________________________________");
    }
}
fn binance_url_btcusdt() {
    let binance_url_btcusdt = format!("{}/ws/btcusdt@kline_1m", BINANCE_WS_API);

    let (mut socket, response) =
        connect(Url::parse(&binance_url_btcusdt).unwrap()).expect("Can't connect.");

    println!("Connected to binance stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }
    println!("______________________________________________________________________");

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                panic!("Error getting text");
            }
        };

        let parsed: models::MyData = serde_json::from_str(&msg).expect("Can't parse");
        let sum = parsed.k.o.parse::<f64>().unwrap()
            + parsed.k.c.parse::<f64>().unwrap()
            + parsed.k.h.parse::<f64>().unwrap()
            + parsed.k.l.parse::<f64>().unwrap();

        // let data: models::MyData = serde_json::from_str(parsed_data).unwrap();

        println!("{:?}", parsed);
        println!("sum: {:?}", sum);
        println!("______________________________________________________________________");
    }
}
