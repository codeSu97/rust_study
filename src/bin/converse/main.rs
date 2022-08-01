use std::convert::TryInto;

fn main() {
    /*
     * 类型转换
     */

    // as 转换
    // Rust不允许两种不同类型的值进行比较
    // 范围较小的类型转换为范围较大的类型
    let a: i32 = 10;
    let b: u16 = 100;
    if a < (b as i32) {
        println!("Ten is less than one hundred.");
    }
    println!("{}", i8::MAX);
    // 内存地址转换指针
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize;
    let second_address = first_address + 4;
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);
    // 转换不具有传递性 就算 e as U1 as U2 是合法的，也不能说明 e as U2 是合法的（e 不能直接转换成 U2）

    // try_into
    let a1: u8 = 10;
    let b2: u16 = 15;
    let b_: u8 = b2.try_into().unwrap();
    if a1 < b_ {
        println!("Ten is less than one hundred.");
    }
}
