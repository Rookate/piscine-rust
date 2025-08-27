use std::f64::consts;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self {
            center: Point(x, y),
            radius: (radius),
        }
    }

    pub fn intersect(&self, other: Circle) -> bool {
        let dx = other.center.0 - self.center.0;
        let dy = other.center.1 - self.center.1;
        let d2 = dx * dx + dy * dy;

        let sum = self.radius + other.radius;
        let dif = (self.radius - other.radius).abs();
        d2 <= sum * sum && d2 >= dif * dif
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        const PI: f64 = consts::PI;
        PI * (self.radius * self.radius)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        let dx = other.0 - self.0;
        let dy = other.1 - self.1;

        ((dx * dx) + (dy * dy)).sqrt()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        let circle1 = Circle {
            center: Point(80.0, 115.0),
            radius: 30.0,
        };
        let point_a = Point(1.0, 1.0);
        let point_b = Point(0.0, 0.0);
        println!("circle = {:?} area = {}", circle, circle.area());
        println!("circle = {:?} diameter = {}", circle, circle.diameter());
        println!("circle1 = {:?} diameter = {}", circle1, circle1.diameter());
        println!(
            "circle and circle1 intersect = {}",
            circle.intersect(circle1)
        );

        println!(
            "distance between {:?} and {:?} is {}",
            point_a,
            point_b,
            point_a.distance(point_b)
        );
    }
}
