fn main() {
    // Vector
    // create new Vector
    let v: Vec<i32> = Vec::new();
    // v.push(1);
    println!("v: {:?}", v);

    let mut v1 = Vec::new();
    v1.push(1);
    println!("v1: {:?}", v1);

    let mut v2 = Vec::with_capacity(16);
    v2.push(1);
    println!("v2: {:?}", v2);
    println!("v2[0]: {:?}", v2[0]);

    // init new Vector
    let mut v3 = vec![1, 2, 3];
    v3.push(4);
    let v3_0 = &v3[0];
    println!("v3: {:?}", v3);
    println!("v3[0]: {:?}", v3[0]);
    println!("v3.get(0): {:?}", v3.get(0));
    println!("v3_0: {:?}", v3_0);

    // 迭代遍历
    let mut iter_v = vec![1, 2, 3];
    for i in &iter_v {
        println!("{}", i);
    }
    // 迭代过程中，也可以修改元素
    for i in &mut iter_v {
        *i += 10;
        println!("{}", i);
    }

    let ip_addr_v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];
    for ip_v in ip_addr_v {
        show_addr(ip_v);
    }
    let ip_address_v: Vec<Box<dyn IpAddress>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];
    for ip_v in ip_address_v {
        ip_v.display();
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn show_addr(ip: IpAddr) {
    println!("{:?}", ip);
}

trait IpAddress {
    fn display(&self);
}

struct V4(String);
impl IpAddress for V4 {
    fn display(&self) {
        println!("{:?}", self.0);
    }
}

struct V6(String);
impl IpAddress for V6 {
    fn display(&self) {
        println!("{:?}", self.0);
    }
}
