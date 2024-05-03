// Basic syntax example
use std::f64::consts::PI;

pub trait Geometry {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn volume(&self) -> f64;
    fn location(&self) -> (f64, f64);
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

/// Documentación incrustada para Geometry Rectangle
impl Geometry for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        (self.width * 2.0) + (self.height * 2.0)
    }

    fn volume(&self) -> f64 {
        unimplemented!()
    }

    fn location(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}


/// Documentación incrustada para Geometry Circle
impl Geometry for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }

    fn volume(&self) -> f64 {
        unimplemented!()
    }

    fn location(&self) -> (f64, f64) {
        (self.x, self.y)
    }

}



fn print_location(shape: &dyn Geometry) {
    let (x, y) = shape.location();
    println!("Area of the shape: {}, {}", x, y);
}

fn print_area(shape: &dyn Geometry) {
    println!("Area of the shape: {}", shape.area());
}

fn main() {
    let r = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 20.0,
    };

    let c = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };

    print_area(&r);
    print_area(&c);

    print_location(&c);
}
