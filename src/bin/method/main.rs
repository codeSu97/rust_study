struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    fn area(&self) -> f64 {
        println!("x: {}, y: {}, radius: {}", self.x, self.y, self.radius);
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn main() {
    let circle = Circle::new(1f64, 1f64, 1f64);
    // println!("{:?}", circle.new())
    println!("{:?}", circle.area());
}
