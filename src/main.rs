fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six is {:?}, none is {:?}", six, none);

    let value: u8 = 3u8;

    match value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // "_" is like the default block
        _ => println!("default"),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // patterns are exhaustive
    // all possible patterns should be provided or else
    // Rust throws an error
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}