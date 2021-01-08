fn main() {
	// create instances of enum
    let four = IpAddrKind::V4;
    let sixe = IpAddrKind::V6;

    // use struct to define field as enum
    let home = IpAddr {
    	kind: IpAddrKind::V4,
    	address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
    	kind: IpAddrKind::V6,
    	address: String::from("::1"),
    };

	// instead of struct
	enum IpAddr {
		V4(u8, u8, u8, u8), // equivalent to putting enum in struct
		V6(String), // put data directly into enum variant
	}

    // attach data to enum
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}

// defined enum - IP addresses
enum IpAddrKind {
	V4,
	V6,
}

// struct IpAddr {
//	kind: IpAddrKind,
//	address: String,
// }




fn route(ip_kind: IpAddrKind) {}