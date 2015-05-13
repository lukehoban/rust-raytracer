extern crate image;

use std::path::Path;
use image::ImageBuffer;

struct Vector { x: f64, y: f64, z: f64 }
impl Vector {
    fn times(k: f64, v: &Vector) -> Vector {
        Vector {x: k*v.x, y: k*v.y, z: k*v.z}
    }
    fn minus(v1: &Vector, v2: &Vector) -> Vector {
        Vector {x: v1.x-v2.x, y: v1.y-v2.y, z: v1.z-v2.z}
    }
    fn dot(v1: &Vector, v2: &Vector) -> f64 {
        v1.x*v2.x+v1.y*v2.y+v1.z*v2.z
    }
    fn mag(v: &Vector) -> f64 {
        (v.x*v.x + v.y*v.y + v.z*v.z).sqrt()
    }
    fn norm(v: &Vector) -> Vector {
        panic!()
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
struct Intersect<'a> { thing: &'a Thing, ray: Ray, dist: f64}
trait Thing {
    fn normal(&self, pos: Vector) -> Vector;
    fn intersect<'a>(&'a self, ray: Ray) -> Option<Intersect<'a>>;
}
struct Light { pos: Vector, color: Color }
struct Camera { pos: Vector, lookAt: Vector }
struct Scene { things: Vec<Box<Thing>>, lights: Vec<Light>, camera: Camera }

struct Sphere { center: Vector, radius: f64 }
impl Thing for Sphere {
    fn normal(&self, pos: Vector) -> Vector {
        Vector::norm(&Vector::minus(&pos, &self.center))
    }
    fn intersect<'a>(&'a self, ray: Ray) -> Option<Intersect<'a>> {
        let eo = Vector::minus(&self.center, &ray.start);
        let v = Vector::dot(&eo, &ray.dir);
        let dist =
            if v < 0.0 {
                0.0
            } else {
                let disc = self.radius*self.radius - Vector::dot(&eo, &eo) + v*v;
                if disc < 0.0 { 0.0 } else { v - disc.sqrt() }
            };
        if dist == 0.0 {
            None
        } else {
            Some(Intersect { thing: self, ray: ray, dist: dist})
        }
    }
}


fn intersections(ray: Ray, scene: &Scene) -> Option<&Intersect>{
  panic!()
}

fn trace_ray(ray: Ray, scene: Scene, depth: u32) -> Color {
    match intersections(ray, &scene) {
        Some(isect) => Color::background(),
        None => Color::background()
    }
}

fn default_scene() -> Scene {
    Scene {
        things: vec![Box::new(Sphere { center: Vector { x: 0.0, y: 1.0, z: -0.25}, radius: 1.0 })],
        lights: vec![],
        camera: Camera { pos: Vector {x: 3.0, y: 2.0, z: 4.0}, lookAt: Vector { x: -1.0, y: 0.5, z: 0.0}}
    }
}

fn main() {
    let v = Vector {x: 1.0, y:2.0, z:3.0};
    Vector::times(4.0, &v);

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
