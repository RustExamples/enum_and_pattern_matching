enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    value_in_coins(Coin::Penny);
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
        Coin::Quarter => 25,
    }
}