#[derive(Debug)]
#[warn(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}

#[warn(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Hello { id: i32 },
}

fn main() {
    // let coin = Coin::Quarter(UsState::Alabama);
    let coin = Coin::Dime;
    let r = value_in_cents(coin);
    println!("{}", r);

    let v = Some(3);
    if let Some(3) = v {
        println!("three");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    println!("a: {}, b: {}", a, b);

    let Point { x, y } = p;
    println!("x: {}, y: {}", x, y);

    match p {
        Point { x, y: 7 } => println!("axis x: {}", x),
        Point { x: 0, y } => println!("axis y: {}", y),
        Point { x, y } => println!("neither axis: ({}, {})", x, y),
    }

    let msg = Message::Hello { id: 13 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in n range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }

    let _p @ Point { x: px, y: py } = Point { x: 10, y: 20 };
    println!("x: {}, y: {}", px, py);
    println!("p: {:?}", _p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
