#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        return f64::sqrt(dx * dx + dy * dy);
    }
}

#[derive(Debug)]
struct Vector {
    op: Point,
    x: f64,
    y: f64,
}

impl Vector {
    fn new(x: f64, y: f64) -> Vector {
        Vector {
            op: Point::origin(),
            x: x,
            y: y,
        }
    }

    fn is_hortogonal(&self, other: &Vector) -> bool {
        self.x * other.x + self.y * other.y == 0.0
    }

    fn norm(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }
}

fn tests_point() {
    println!("--- Point tests ---");
    let p1 = Point::origin();
    let p2 = Point::new(1.0, 1.0);
    let p3 = Point::new(2.0, 2.0);

    println!("{:?} should be (0, 0)", p1);
    println!("{:?} should be (1, 1)", p2);
    println!("{:?} should be (2, 2)", p3);

    let p4 = p2.add(&p3);
    println!("{:?} should be (3, 3)", p4);

    let p5 = p3.sub(&p2);
    println!("{:?} should be (1, 1)", p5);

    let p6 = p2.distance(&p3);
    println!("{:?} should be ~1.4142135623730951", p6);
}

fn tests_vector() {
    println!("--- Vector tests ---");
    let v1 = Vector::new(1.0, 1.0);
    let v2 = Vector::new(1.0, -1.0);

    println!("{:?} should be (1, 1)", v1);
    println!("{:?} should be (1, -1)", v2);

    let v3 = v1.is_hortogonal(&v2);
    println!("{:?} should be true", v3);

    let v4 = v1.norm();
    println!("{:?} should be ~1.4142135623730951", v4);
}

fn main() {
    tests_point();
    tests_vector();
}
