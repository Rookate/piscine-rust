use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

/* -------------------------- Point -------------------------- */
#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::rng();
        let x = rng.random_range(0..width);
        let y = rng.random_range(0..height);
        Self { x, y }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(255, 0, 0)
    }
}

/* -------------------------- Line -------------------------- */

pub struct Line {
    p1: Point,
    p2: Point,
    color: Color,
}

impl Line {
    pub fn new(a: &Point, b: &Point) -> Self {
        let mut rng = rand::rng();
        let color = Color::rgb(
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
        );

        Self {
            p1: *a,
            p2: *b,
            color,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let a = Point::random(width, height);
        let mut b = Point::random(width, height);
        if a.x == b.x && a.y == b.y {
            b.x = (b.x + 1).min(width - 1);
        }
        Self::new(&a, &b)
    }
}

// Algo de Bresenham
impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let mut x0 = self.p1.x;
        let mut y0 = self.p1.y;
        let x1 = self.p2.x;
        let y1 = self.p2.y;

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            image.display(x0, y0, self.color());
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

/* -------------------------- Triangle -------------------------- */

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    color: Color,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        let mut rng = rand::rng();
        let color = Color::rgb(
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
        );

        Self {
            p1: *a,
            p2: *b,
            p3: *c,
            color,
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        Line {
            p1: self.p1,
            p2: self.p2,
            color: self.color(),
        }
        .draw(image);
        Line {
            p1: self.p2,
            p2: self.p3,
            color: self.color(),
        }
        .draw(image);
        Line {
            p1: self.p3,
            p2: self.p1,
            color: self.color(),
        }
        .draw(image);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

/* -------------------------- Rectangle -------------------------- */

pub struct Rectangle {
    tl: Point,
    br: Point,
    color: Color,
}

impl Rectangle {
    pub fn new(a: &Point, b: &Point) -> Self {
        let (minx, maxx) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
        let (miny, maxy) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
        let mut rng = rand::rng();
        let color = Color::rgb(
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
        );
        Self {
            tl: Point::new(minx, miny),
            br: Point::new(maxx, maxy),
            color,
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let tr = Point::new(self.br.x, self.tl.y);
        let bl = Point::new(self.tl.x, self.br.y);

        Line {
            p1: self.tl,
            p2: tr,
            color: self.color(),
        }
        .draw(image);
        Line {
            p1: tr,
            p2: self.br,
            color: self.color(),
        }
        .draw(image);
        Line {
            p1: self.br,
            p2: bl,
            color: self.color(),
        }
        .draw(image);
        Line {
            p1: bl,
            p2: self.tl,
            color: self.color(),
        }
        .draw(image);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

/* -------------------------- Rectangle -------------------------- */
pub struct Circle {
    center: Point,
    radius: i32,
    color: Color,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        let mut rng = rand::rng();
        let color = Color::rgb(
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
        );
        Self {
            center: *center,
            radius: radius.max(0),
            color,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let c = Point::random(width, height);
        let max_r = i32::min(width, height).max(1) / 2;
        let mut rng = rand::rng();
        let r = rng.random_range(5..=max_r.max(20));
        Self::new(&c, r)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let mut x = self.radius;
        let mut y = 0;
        let mut err = 1 - x;

        let cx = self.center.x;
        let cy = self.center.y;
        let col = self.color();

        let plot_octants = |ix: i32, iy: i32, img: &mut Image| {
            img.display(cx + ix, cy + iy, col.clone());
            img.display(cx + iy, cy + ix, col.clone());
            img.display(cx - iy, cy + ix, col.clone());
            img.display(cx - ix, cy + iy, col.clone());
            img.display(cx - ix, cy - iy, col.clone());
            img.display(cx - iy, cy - ix, col.clone());
            img.display(cx + iy, cy - ix, col.clone());
            img.display(cx + ix, cy - iy, col.clone());
        };

        while x >= y {
            plot_octants(x, y, image);
            y += 1;
            if err < 0 {
                err += 2 * y + 1;
            } else {
                x -= 1;
                err += 2 * (y - x + 1);
            }
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}
