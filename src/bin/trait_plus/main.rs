use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0)
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot::fly");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard::fly");
    }
}

impl Human {
    fn fly(&self) {
        println!("Human::fly");
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(Millimeters(1) + Meters(1), Millimeters(2));

    let human = Human;
    human.fly();
    Pilot::fly(&human);
    Wizard::fly(&human);
    println!(
        "{:?}",
        // 完全限定语法
        // <Type as Trait>::function(receiver_if_method, next_arg, ...);
        // 向 Rust 编译器提供了类型注解，也就是 Add 就是 Point，从而执行Point的add方法
        // 第一个参数是方法接收器 receiver，即self
        <Point as Add>::add(Point { x: 1, y: 2 }, Point { x: 2, y: 1 })
    );
}
