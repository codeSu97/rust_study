static MAX: u32 = 0;

fn foo() {}

fn main() {
    let hello = "hello world".to_string();
    let data = Box::new(1);

    // string literals 指向RODATA地址
    println!("RODATA: {:p}", "hello world");

    // static 变量在DATA section
    println!("DATA (static var): {:p}", &MAX);

    // function 在 TEXT
    println!("TEXT (function): {:p}", foo as *const ());

    // String结构体 分配在栈上，所以其引用指向一个栈地址
    println!("STACK (&hello): {:p}", &hello);

    // 需要通过解引用获取其堆上的数据，然后取引用
    println!("HEAP (&*hello): {:p}", &*hello);

    // Box实现了 Point trait特征，无需额外解引用
    println!("HEAP (box impl Pointer) {:p}, {:p}", data, &*data);

    let x = "hello world";
    println!("x= {:p}", x);
    bar();
}

fn bar() {
    let v = "hello world";
    println!("v= {:p}", v);
}
