use std::collections::HashMap;

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

    // HashMap new创建
    let mut hash_map = HashMap::new();
    hash_map.insert("姓名", "张三");
    hash_map.insert("年纪", "21");
    for kv in &hash_map {
        println!("{}: {}", kv.0, kv.1);
    }
    for (k, v) in &hash_map {
        println!("{}: {}", k, v);
    }

    // HashMap 迭代器+collect创建
    let map_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];
    let map: HashMap<_, _> = map_list.into_iter().collect();
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }
    // get获取
    // 返回Option<&i32>类型，查询不到None，查询到Some(&i32)
    // &i32是对HashMap值的借用，不然可能会发生所有权转移
    println!("{:?}", map.get(&String::from("中国队")));

    // insert更新
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));
    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));
    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5
                       // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入

    // 在已有值上更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // 根据空格来切分字符串(英文单词都是通过空格切分)
    for word in text.split_whitespace() {
        // or_insert 返回了 &mut v 引用，因此可以通过该可变引用直接修改 map 中对应的值
        let count = map.entry(word).or_insert(0);
        // 使用 count 引用时，需要先进行解引用 *count，否则会出现类型不匹配
        *count += 1;
    }
    println!("{:?}", map);
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
