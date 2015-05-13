extern crate image;

use std::path::Path;
use image::ImageBuffer;

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
  fn to_drawing_color(c: Color) -> Color {
    c
  }
  fn background() -> Color {
      Color {r: 0.0, g: 0.0, b: 0.0 }
  }
}

struct Ray { dir: Vector, start: Vector }
struct Intersect { thing: Box<Thing>, ray: Ray, dist: f64}
trait Thing {
    fn normal(&self, pos: Vector) -> Vector;
    fn intersect(&self, ray: Ray) -> Intersect;
}
struct Light { pos: Vector, color: Color }
struct Camera { }
struct Scene { things: Box<[Thing]>, lights: Box<[Light]>, camera: Camera }

fn intersections(ray: Ray, scene: Scene) -> Option<Intersect>{
  panic!()
}

fn traceRay(ray: Ray, scene: Scene, depth: u32) -> Color {
    match intersections(ray, scene) {
        Some(isect) => Color::background(),
        None => Color::background()
    }
}

fn defaultScene() -> Scene {
    {
        things: Box::new([]),
        lights: Box::new([]),
        camera: Camera { }
    }
}

fn main() {
    let v = Vector {x: 1.0, y:2.0, z:3.0};
    Vector::times(4.0, v);

    let c = Color {r: 1.0, g:2.0, b:3.0};
    Color::scale(4.0, c);

    println!("Hello world!");

    //Construct a new by repeated calls to the supplied closure.
    let img = ImageBuffer::from_fn(512, 512, |x, y| {
        if x % 2 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    //Write the contents of this image to the Writer in PNG format.
    let _ = img.save(&Path::new("test.png"));

}
