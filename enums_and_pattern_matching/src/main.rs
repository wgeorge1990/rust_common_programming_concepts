fn main() {
    println!("--------------------------");
    println!("ENUMS AND PATTERN MATCHING");
    println!("--------------------------");
    #[derive(Debug)]
    struct Ipv4Addr {
        address: String,
    }
    #[derive(Debug)]
    struct Ipv6Addr {
        address: (u32, u32, u32, u32),
    }

    #[derive(Debug)]
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    let home = Ipv4Addr { address: String::from("1.27.0.0.1")};

    let home_v4 = IpAddr::V4(home);
    println!("{:?}", home_v4);



}
