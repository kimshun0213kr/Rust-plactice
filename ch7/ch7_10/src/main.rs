struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y:f64) -> Self {
        Self{x,y}
    }
}

fn main() {
    let a = Point::new(3.,5.);
    println!("{},{}",a.x,a.y);
}