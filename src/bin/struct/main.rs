#[derive(Debug)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: dbg!(String::from("some_username1")),
        active: true,
        sign_in_count: 1,
    };
    user1.active = false;
    println!(
        "{}, {}, {}, {}",
        user1.active, user1.email, user1.username, user1.sign_in_count
    );

    let user2 = User {
        email: String::from("someone1@example.com"),
        ..user1 // 表明，凡是没有显示声明的字段，都从user1中获取，且必须在结构体尾部使用
    };
    println!("{:?}", user1.active);
    println!("{:#?}", user2);

    // 元组结构体
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    let color = Color(0, 0, 0);
    let point = Point(1, 2, 3);
    println!("{:?}", color);
    println!("{:?}", point);

    dbg!(&user2);
    println!("{}", user2.email);

    // 单元结构体
    // struct AlwaysEqual;
    // let subject = AlwaysEqual;
    // impl SomeTrait for AlwaysEqual {}
}
