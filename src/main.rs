#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    Michigan,
    Arizona,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    value_in_coins(Coin::Penny);
    value_in_coins(Coin::Quarter(UsState::Arizona));
}

fn value_in_coins(coin: Coin) -> u32 {
    // "match" - keyword
    // "coin" - expression (can return any value)
    // "{ sts }" - arm => LHS - pattern, RHS - expression
    // patterns searched in order and stop at matching one
    match coin {
        Coin::Penny => {
            println!("Lucy Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Arms can bind to value of matching pattern
        // When a quarter matches, the value it holds is
        // bound to "state"
        // This way we can extract value from enum
        Coin::Quarter(state) => {
            println!("Quarter is from the state of {:?}", state);
            25
        },
    }
}