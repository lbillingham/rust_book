fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Hello, world!");
    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home is {:?}", home);
    println!("loopback is {:?}", loopback);
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route (ip_type: IpAddrKind) {
    println!("lalala {:?}", ip_type);
}

