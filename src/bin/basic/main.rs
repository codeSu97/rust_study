// use std::any::type_name;

fn main() {
    let a = 10;
    let b: i32 = 20;
    let mut c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("( a + b) + (c + d) = {}", e);

    c = 40_i32;
    println!("c = {}", c);

    let _f = 10;

    assert!((0.1_f32 + 0.2 - 0.3).abs() < 0.000001);

    // for i in '你'..='我' {
    //     print!("{}", i);
    // }

    plus_or_minus(5);

    add_two(1, 2);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    x + 5
}

fn add_two(i: i32, j: i32) -> i32 {
    i + 1 + j
}

// fn type_p<T>(_: T) {
//     println!("{:?}", { type_name::<T>() });
// }
