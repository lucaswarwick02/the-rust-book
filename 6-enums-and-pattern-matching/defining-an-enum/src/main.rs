// Define an enum which could be one of the following values
enum IpAddrKind {
    V4,
    V6,
}   
// Structs can store enums as normal
struct IpAddr {
    kind:IpAddrKind,
    address: String,
}


fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    let some_number = Some(5);  // Can be set to None as well!
    let some_char = Some('e');

    let absent_number: Option<usize> = None;  // Can be set to a uint as well!
}
