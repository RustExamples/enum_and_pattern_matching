#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn using_match(){
    let value: Option<u8> = Some(5);
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Arizona);

    println!("Using `match`:");

    match value {
        Some(3) => println!("true"),
        _ => println!("false")
    }

    match coin {
        Coin::Quarter(state) => println!("quarter belongs to state {:?}", state),
        _ => count += 1
    }

    println!("coins that aren't quarter is {}", count);
}

fn using_if_let() {
    let value: Option<u8> = Some(5);
    let mut count = 0;
    let coin = Coin::Penny;

    println!("Using `if let`:");

    if let Some(5) = value {
        println!("true");
    }
    else{
        println!("false");
    }

    if let Coin::Quarter(state) = coin {
        println!("quarter belongs to state {:?}", state);
    }
    else {
        count += 1;
    }

    println!("coins that aren't quarter is {}", count);
}

fn main() {
    using_match();

    using_if_let();
}