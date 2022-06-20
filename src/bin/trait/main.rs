//! 特征
#![allow(dead_code)]
use std::convert::TryInto; // 可以舍去，Rust能够把最常用的标准库中的特征通过std::prelude模块提前引入当前作用域中
use std::fmt::{self, Display};
use std::ops::Add;

pub trait Summary {
    fn summarize_author(&self) -> String;

    // 特征默认实现，允许调用相同特征中的其他方法，哪怕该方法没有默认实现，例如上面的summarize_author
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    // 重写
    fn summarize(&self) -> String {
        format!("{} 发表了微博 {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        todo!() // TODO:
    }
}

// 特征作为函数参数
pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// 特征约束
pub fn notify_1<T: Summary>(item: &T) {
    println!("{}", &item.summarize());
}

// 多重约束
pub fn notify_some(item: &(impl Summary + Display)) {
    println!("{}", &item.summarize());
}
pub fn notify_some_1<T: Summary + Display>(item: &T) {
    println!("{}", &item.summarize());
}

#[derive(Debug)] // 使用derive派生特征，被标记的对象会自动实现对应的默认特征代码
pub struct Point<T: Add<T, Output = T>> {
    // 限制类型T必须实现Add特征，否则无法进行Add操作
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {:?}, {}>", self.name, self.data, self.state) // Vec未实现Display特征
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main() {
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };
    let weibo = Weibo {
        username: "Sunface".to_string(),
        content: "好像微博没有Tweet好用".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());

    notify(&post);
    notify(&weibo);

    notify_1(&post);
    notify_1(&weibo);

    // 引入特征
    let a: i32 = 10;
    let b: u16 = 100;
    let b_ = b.try_into().unwrap();
    if a < b_ {
        println!("Ten is less then one hundred.");
    }

    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    println!("{:?}", add(p1, p2));

    let file_x = File::new("file.txt");
    println!("{}", file_x);
    println!("{:?}", file_x);
}
