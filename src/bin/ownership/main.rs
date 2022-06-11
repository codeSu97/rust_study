fn main() {
    // 所有权
    let s = String::from("Hello"); // s进入作用域
    takes_ownership(s); // s转移作用域到 tasks_ownership函数里
                        // s 失效
                        // println!("takes_ownership remove s: {}", s);  // s1已经转移作用域，所以不可使用

    let x = 5; // x 进入作用域
    makes_copy(x); // x本应该转移作用域到 makes_copy函数里，但是i32是Copy的，所以x没有转移作用域，后面可以继续使用
    println!("makes_copy remove x: {}", x); // 继续使用x

    // 引用和借用
    let x = 5; // 把5绑定到x上
    let y = &x; // 把x的引用绑定到y上
    assert_eq!(5, x);
    assert_eq!(5, *y); // *y，解引用，即访问y所指向的值

    // 不可变引用
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("the length of '{}' is {}", s1, len);
    let x1 = 10;
    eq_i32(&x1);

    // 可变引用
    let mut s2 = String::from("Hello");
    change(&mut s2);
    println!("mut s2: {}", s2);

    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);
    // println!("{} and {}", r1, r2);
} // 先把x移出作用域，然后移出s(但是s已经被移走了，所以没有特殊操作)

fn change(s: &mut String) {
    s.push_str(", world");
}

fn eq_i32(x: &i32) {
    assert_eq!(10, *x); // 基本类型需要解引用
}

fn calculate_length(s: &String) -> usize {
    println!("len: {}", s.len());
    assert_eq!("Hello", *s);
    s.len()
}

fn makes_copy(x: i32) {
    // x进入作用域
    println!("{}", x);
} // x移出作用域，没有特殊操作

fn takes_ownership(s: String) {
    // s进入作用域
    println!("{}", s);
} // s移出作用域，并调用 drop 方法释放占用的内存
