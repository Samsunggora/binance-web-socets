mod models;
use tungstenite::connect;
use url::Url;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";
static ETH_USDT: &str = "ethusdt";
static BTC_USDT: &str = "btcusdt";
fn main() {
    let binance_url_ethusdt = format!("{}/ws/{}@kline_1m", BINANCE_WS_API, ETH_USDT);
    let binance_url_btcusdt = format!("{}/ws/{}@kline_1m", BINANCE_WS_API, BTC_USDT);

    let (mut first_socket, first_response) =
        connect(Url::parse(&binance_url_ethusdt).unwrap()).expect("Can't connect.");

    let (mut second_socket, second_response) =
        connect(Url::parse(&binance_url_btcusdt).unwrap()).expect("Can't connect.");

    loop {
        let first_msg = first_socket.read_message().expect("Error reading message");
        let first_msg = match first_msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                panic!("Error getting text");
            }
        };

        let second_msg = second_socket.read_message().expect("Error reading message");
        let second_msg = match second_msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                panic!("Error getting text");
            }
        };

        let first_parsed: models::MyData = serde_json::from_str(&first_msg).expect("Can't parse");
        let second_parsed: models::MyData = serde_json::from_str(&second_msg).expect("Can't parse");
        let o_sum =
            first_parsed.k.o.parse::<f64>().unwrap() + second_parsed.k.c.parse::<f64>().unwrap();
        let h_sum =
            first_parsed.k.h.parse::<f64>().unwrap() + second_parsed.k.h.parse::<f64>().unwrap();
        let l_sum =
            first_parsed.k.l.parse::<f64>().unwrap() + second_parsed.k.l.parse::<f64>().unwrap();
        let c_sum =
            first_parsed.k.c.parse::<f64>().unwrap() + second_parsed.k.c.parse::<f64>().unwrap();
        let ends_data = models::EndData {
            timestamp: first_parsed.k.t,
            open_price: o_sum,
            close_price: c_sum,
            high_price: h_sum,
            low_price: l_sum,
        };
        let stream_data = models::StreamData {
            stream: String::from("btcusdt+ethusd@1m"),
            data: ends_data,
        };

        let json = serde_json::to_string(&stream_data).unwrap();

        println!("{}", json);
    }
}
