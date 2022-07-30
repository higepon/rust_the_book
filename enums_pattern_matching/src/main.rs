enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("called");
    }
}

enum Option2<T> {
    None,
    Some(T),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let four = IpAddrKind::V4;
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let home = IpAddr2::V4(String::from("127.0.0.1"));

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from(":1"));

    let m = Message::Quit;
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let coin = Coin::Quarter(UsState::Alabama);
    let value: u32 = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State = {:?}", state);
            25
        }
    };

    let five = Some(5);

    if let Some(val) = plus_one(five) {
        println!("val = {:?}", val);
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
