// enum store variants
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// enum can store variants of different type
#[derive(Debug)]
enum Message {
    Quit,
    Write(String),
    Move {x: i32, y: i32},
    ChangeColor(String)
}

// enum can have methods
impl Message {
    fn call(&self) {
        
    }
}

// enum inside a struct
struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

fn main() {
    // Get instance of enum
    // "V4" & "V6" variants are namespaced under enum
    // "ipv4" & "ipv6" are both type IpAddrKind
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    // Pass enum to function
    route(&ipv4);
    route(&ipv6);

    let localhost_v4 = IpAddr {
        kind: ipv4,
        addr: String::from("127.0.0.1"),
    };

    let localhost_v6 = IpAddr {
        kind: ipv6,
        addr: String::from("::1"),
    };

    println!("{:?}",localhost_v6.kind);

    // Store data directly inside enum
    // rather than using struct to give it a meaning
    let change_color = Message::ChangeColor(String::from("red"));

    println!("change_color is {:?}", change_color);
}

// Functions accepting enum as argument
fn route(ipaddr: &IpAddrKind) {

}
