
trait Drawable {
  fn points(&self) -> Vec<Point>;
  fn draw(&self) -> String {
    let points = self.points();
    points.iter()
    .fold("".to_string(), 
      |r, p| if r.len() == 0 { format!("M {} {}", p.x, p.y) } else { format!("{} L {} {}", r, p.x, p.y) }
    ) + format!(" L {} {}", points[0].x, points[0].y).as_str()
  }
}

fn draw<T>(object: &T) -> String 
where T: Drawable
{
  object.draw()
}

#[derive(Clone, Copy)]
pub struct Point {
  x: f32,
  y: f32,
}

impl Point {
  pub fn new(x: f32, y: f32) -> Point { Point { x, y } }
}

pub struct Square { 
  p1: Point,
  p2: Point,
  p3: Point,
  p4: Point,
}

impl Default for Square {
  fn default() -> Square {
    Square {
      p1: Point::new(0.0, 0.0),
      p2: Point::new(1.0, 0.0),
      p3: Point::new(1.0, 1.0),
      p4: Point::new(0.0, 1.0),
    }
  }
}

impl Drawable for Square {
  fn points(&self) -> Vec<Point> {
    vec![ self.p1, self.p2, self.p3, self.p4 ]
  }
}

pub struct Triangle { 
  p1: Point,
  p2: Point,
  p3: Point,
}

impl Default for Triangle {
  fn default() -> Triangle {
    Triangle {
      p1: Point::new(0.0, 0.0),
      p2: Point::new(1.0, 0.0),
      p3: Point::new(0.0, 1.0),
    }
  }
}

impl Drawable for Triangle {
  fn points(&self) -> Vec<Point> {
    vec![ self.p1, self.p2, self.p3 ]
  }
}

fn main() {
  println!("Triangle: {}", draw(&Triangle::default()));
  println!("Square: {}", draw(&Square::default()));
  println!("or...");
  println!("Triangle: {}", Triangle::default().draw());
  println!("Square: {}", Square::default().draw());
}
