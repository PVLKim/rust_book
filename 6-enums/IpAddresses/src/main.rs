#[derive(Debug)]
// Ip addresses
enum IpAddr {
    V4(u8, u8, u8, u8), // can take multiple arguments
    V6(String), // value can be of any type
}

// Message
enum Message {
    Quit,                       // no associated data ~ unit struct
    Move { x: i32, y: i32 },    // has named fields like a struct
    Write(String),              // include single string ~ tuple struct 
    ChangeColor(i32, i32, i32), // tuple struct
}
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}
