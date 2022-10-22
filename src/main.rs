enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
    V8,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {}
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Coin for luck");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Coin from state {:?}", state);
            25
        },
    }
}
fn main() {
    let home = IpAddr::V4(127, 0, 0, 0);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let coin_quarter= Coin::Quarter(UsState::Alaska);
    let value_of_coin = value_in_cents(coin_quarter);

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("Three"),
        _ => (),
    };
    if let Some(i) = some_u8_value {
        println!("Value is {}", i);
    } else {
        println!("Is not value1")
    };
}
