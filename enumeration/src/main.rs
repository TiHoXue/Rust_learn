enum Message {
    Start,
    Write(String),
    Calculate { number_1: i32, number_2: i32 },
    Stop(i32, i32),
}

impl Message {
    fn call(&self) {
        // method body
        println!("Calling");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // other states...
}

enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coins) -> u8 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter(state) => {
            println!("{state:?}'s quarter.");
            25
        }
    }
}

enum NumberEnum {
    One(u32),
    Two(u32),
    Three(u32),
    Four(u32),
    Five(u32),
    Six(u32),
    Seven(u32),
    Eight(u32),
    Nine(u32),
    Ten(u32),
}

impl NumberEnum {
    fn get_number_less_than_six(number_enum: NumberEnum) -> Option<u32> {
        match number_enum {
            NumberEnum::One(n) => Some(n),
            NumberEnum::Two(n) => Some(n),
            NumberEnum::Three(n) => Some(n),
            NumberEnum::Four(n) => Some(n),
            NumberEnum::Five(n) => Some(n),
            _ => {
                println!("Greater than 6.");
                None
            }
        }
    }
}

fn main() {
    let msg = Message::Start;
    msg.call();

    let msg = Message::Stop(1,2);
    msg.call();

    // Option
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);

    // Coins
    let penny = value_in_cents(Coins::Penny);
    println!("penny: {}", penny);
    let nickle = value_in_cents(Coins::Nickel);
    println!("nickle: {}", nickle);
    let dime = value_in_cents(Coins::Dime);
    println!("dime: {}", dime);
    let quarter = value_in_cents(Coins::Quarter(UsState::Alaska));
    println!("quarter: {}", quarter);

    // match
    let four = NumberEnum::get_number_less_than_six(NumberEnum::Four(4));
    match four {
        None => println!("Get None"),
        Some(n) => println!("Get {n}"),
    }
}
