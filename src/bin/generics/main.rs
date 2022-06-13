// use std::result;
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

/**
 * 泛型，generics
 */
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// const 泛型
#[warn(unused_variables)]
fn const_generics<T: std::fmt::Debug>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    println!("{:?}", val);
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let number_list = vec![30, 50, 20, 100, 70];

    let result = largest(&number_list);
    println!("The largest number is {:?}", number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'c', 'b', 'z', 's'];
    let result = largest(&char_list);
    println!("The largest char is {:?}", char_list);
    println!("The largest char is {}", result);

    println!("1 + 2 = {}", add(1, 2));

    const_generics([0u8; 0]);
    const_generics([0u8; 512]);
    // const_generics([0u8; 1024]);
}
