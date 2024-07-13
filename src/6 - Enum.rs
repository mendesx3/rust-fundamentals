#[derive(Debug)]
enum TypesEnum {
    V4(String),
    V6(String),
}

struct Ipv4Addr {
    // details elided
}

struct Ipv6Addr {
    // details elided
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let v4 = TypesEnum::V4(String::from("localhost:8080"));
    let v6 = TypesEnum::V6(String::from("::1"));
    ;
    println!("{:?}!", v4);
    println!("{:?}!", v6);

    route(TypesEnum::V6);

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    print_message(msg1);
    print_message(msg2);
    print_message(msg3);
    print_message(msg4);

    //Concise Control Flow with if let
    let some_values = Some(5);
    if let Some(x) = some_values {
        println!("The value is: {}", x);
    } else {
        println!("No value found");
    }

    match some_values {
        Some(x) => println!("The value is: {}", x),
        None => println!("No value found"),
    }

    //Result
    let result: Result<i32, &str> = Ok(10);

    if let Ok(value) = result {
        println!("The value is: {}", value);
    } else {
        println!("There was an error");
    }
}
fn print_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}
fn route(v: fn(String) -> TypesEnum) {
    println!("{:?}!", v)
}