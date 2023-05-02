use tungstenite::{connect, Message, protocol::CloseFrame, protocol::frame::coding::CloseCode};
use url::Url;
use serde_json;
use std::env;

fn main() {

    // add backtrace
    let key = "RUST_BACKTRACE";
    env::set_var(key, "1");
    // connect to the WS server locally
    let (mut socket, _response) = connect(Url::parse("ws://192.168.25.30:8765").unwrap()).expect("Can't connect");
    

    let mut i: isize = 0;
    // loop forever, handling parsing each message
    while i < 20 {
        let str_i: String = i.to_string();
        // write a message containing "Hello, test" to the server
        socket.write_message(Message::Text("Testing another message".into())).unwrap();
        // socket.write_message(Message::Text(str_i.clone().into())).unwrap();

        let msg = socket.read_message().expect("Error reading message");
        println!("{:?}", msg);
        let msg = match msg {
            Message::Text(s) => { s }
            Message::Close(_o) => {"Closed frame".to_string()}
            _ => { std::panic::panic_any(msg) }
        };
        println!("{:?}", msg);
        // let parsed: serde_json::Value = serde_json::from_str(&msg).expect("Can't parse to JSON");
        i = i + 1;
        // println!("{:?} : {:?}", parsed["result"], _response);
    };
    // let close_frame = CloseFrame {
    //     code: CloseCode::Normal,
    //     reason: "aaaa".into(),
    // };
    // socket.write_message(Message::Close(Some(close_frame).into())).unwrap();
}
