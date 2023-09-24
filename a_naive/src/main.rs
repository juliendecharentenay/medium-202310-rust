
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

fn draw(points: &Vec<Point>) -> String {
  points.iter()
  .fold("".to_string(), 
    |r, p| if r.len() == 0 { format!("M {} {}", p.x, p.y) } else { format!("{} L {} {}", r, p.x, p.y) }
  ) + format!(" L {} {}", points[0].x, points[0].y).as_str()
}

fn main() {
  println!("Triangle: {}", draw(&Triangle::default().points()));
  println!("Square: {}", draw(&Square::default().points()));
}
