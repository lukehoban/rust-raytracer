

struct Vector { x: f64, y: f64, z: f64 }
impl Vector {
    fn times(k: f64, v: Vector) -> Vector {
        Vector {x: k*v.x, y: k*v.y, z: k*v.z}
    }
}

struct Color { r: f64, g: f64, b: f64 }
impl Color {
  fn scale(k: f64, c: Color) -> Color {
      Color {r: k*c.r, g: k*c.g, b: k*c.b}
  }
  fn toDrawingColor(c: Color) -> Color {
      
  }
}

fn main() {
    let v = Vector {x: 1.0, y:2.0, z:3.0};
    Vector::times(4.0, v);

    let c = Color {r: 1.0, g:2.0, b:3.0};
    Color::scale(4.0, c);

    println!("Hello world!");
}
