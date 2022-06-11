fn main() {
    let my_name = String::from("Pascal");
    greet(my_name);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..];
    let ss = &s[..];
    println!("{}, {}, {}", hello, world, ss);

    let sss = "中国人";
    // let a = &sss[0..2];  // 切片的索引必须落在字符之间的边界位置，也就是 UTF-8 字符的边界，例如中文在 UTF-8 中占用三个字节,下面的代码就会崩溃
    let a = &sss[0..3];
    println!("{}", a);
}

fn greet(name: String) {
    println!("{}", name);
}
