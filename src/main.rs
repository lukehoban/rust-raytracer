extern crate image;

use std::f64::INFINITY;
use std::path::Path;
use image::ImageBuffer;

#[derive(Clone)]
struct Vector { x: f64, y: f64, z: f64 }
impl Vector {
    fn times(k: f64, v: &Vector) -> Vector {
        Vector {x: k*v.x, y: k*v.y, z: k*v.z}
    }
    fn plus(v1: &Vector, v2: &Vector) -> Vector {
        Vector {x: v1.x+v2.x, y: v1.y+v2.y, z: v1.z+v2.z}
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
        let mag = Vector::mag(v);
        let div = if mag == 0.0 { INFINITY } else { 1.0 / mag };
        Vector::times(div, &v)
    }
    fn cross(v1: &Vector, v2: &Vector) -> Vector {
        Vector { x: v1.y*v2.z-v1.z*v2.y, y: v1.z*v2.x-v1.x*v2.z, z: v1.x*v2.y-v1.y*v2.x }
    }
    fn dot_pos_neg<T, F1, F2>(v1: &Vector, v2: &Vector, pos: F1, neg: F2) -> T
            where F1: FnOnce(f64) -> T, F2: FnOnce(f64) -> T {
        let d = Vector::dot(&v1, &v2);
        if d > 0.0 { pos(d) } else { neg(d) }
    }
}

struct Color { r: f64, g: f64, b: f64 }
impl Color {
  fn scale(k: f64, c: &Color) -> Color {
      Color {r: k*c.r, g: k*c.g, b: k*c.b}
  }
  fn plus(v1: &Color, v2: &Color) -> Color {
      Color {r: v1.r+v2.r, g: v1.g+v2.g, b: v1.b+v2.b}
  }
  fn times(v1: &Color, v2: &Color) -> Color {
      Color {r: v1.r*v2.r, g: v1.g*v2.g, b: v1.b*v2.b}
  }
  fn to_drawing_color(&self) -> [u8; 3] {
    let legalize = |d| if d > 1.0 { 255 } else { (d * 255.0) as u8 };
    [legalize(self.r), legalize(self.g), legalize(self.b)]
  }
  fn white() -> Color { Color {r: 1.0, g: 1.0, b: 1.0 } }
  fn grey() -> Color { Color {r: 0.5, g: 0.5, b: 0.5 } }
  fn black() -> Color { Color {r: 0.0, g: 0.0, b: 0.0 } }
  fn background() -> Color { Color::black() }
}

struct Ray<'a> { dir: &'a Vector, start: &'a Vector }
struct Intersect<'a> { thing: &'a Thing, dist: f64}
struct Light { pos: Vector, color: Color }
struct Camera { pos: Vector, forward: Vector, right: Vector, up: Vector }
impl Camera {
    fn new(pos: Vector, look_at: Vector) -> Camera {
        let forward = Vector::norm(&Vector::minus(&look_at,&pos));
        let down = Vector { x: 0.0, y: -1.0, z: 0.0};
        let right = Vector::times(1.5, &Vector::norm(&Vector::cross(&forward, &down)));
        let up = Vector::times(1.5, &Vector::norm(&Vector::cross(&forward, &right)));
        Camera { pos: pos, forward: forward, right: right, up: up }
    }
}
struct Scene { things: Vec<Box<Thing>>, lights: Vec<Light>, camera: Camera }

trait Thing {
    fn normal(&self, pos: &Vector) -> Vector;
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<Intersect<'a>>;
    fn surface(&self) -> &Surface;
}

struct Sphere { center: Vector, radius: f64, surface: Box<Surface> }
impl Thing for Sphere {
    fn surface(&self) -> &Surface { &*self.surface }
    fn normal(&self, pos: &Vector) -> Vector {
        Vector::norm(&Vector::minus(&pos, &self.center))
    }
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<Intersect<'a>> {
        let eo = Vector::minus(&self.center, &ray.start);
        Vector::dot_pos_neg(&eo, &ray.dir, |v| {
            let disc = self.radius*self.radius - Vector::dot(&eo, &eo) + v*v;
            if disc < 0.0 { None } else { Some(Intersect { thing: self, dist: v - disc.sqrt() }) }
        }, |_| None)
    }
}

struct Plane { norm: Vector, offset: f64, surface: Box<Surface> }
impl Thing for Plane {
    fn surface(&self) -> &Surface { &*self.surface }
    fn normal(&self, _: &Vector) -> Vector { self.norm.clone() }
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<Intersect<'a>> {
        Vector::dot_pos_neg(&self.norm, &ray.dir, |_| None, |denom| {
            let dist = (Vector::dot(&self.norm, &ray.start) + self.offset) / -denom;
            Some(Intersect{thing: self, dist: dist})
        })
    }
}

trait Surface {
    fn diffuse(&self, pos: &Vector) -> Color;
    fn specular(&self, pos: &Vector) -> Color;
    fn reflect(&self, pos: &Vector) -> f64;
    fn roughness(&self) -> i32;
}

struct Shiny;
impl Surface for Shiny {
    fn diffuse(&self, _: &Vector) -> Color {
        Color::white()
    }
    fn specular(&self, _: &Vector) -> Color {
        Color::grey()
    }
    fn reflect(&self, _: &Vector) -> f64 {
        0.7
    }
    fn roughness(&self) -> i32 { 250 }
}

struct Checkerboard;
impl Surface for Checkerboard {
    fn diffuse(&self, pos: &Vector) -> Color {
        if 0 == (pos.z.floor() + pos.x.floor()) as u32 % 2 { Color::white() } else { Color::black() }
    }
    fn specular(&self, _: &Vector) -> Color {
        Color::white()
    }
    fn reflect(&self, pos: &Vector) -> f64 {
        if 0 == (pos.z.floor() + pos.x.floor()) as u32 % 2 { 0.1 } else { 0.7 }
    }
    fn roughness(&self) -> i32 { 150 }
}

fn closest_intersection<'a>(ray: &Ray, scene: &'a Scene) -> Option<Intersect<'a>> {
  let mut closest = INFINITY;
  let mut closest_inter = None;
  for thing in &scene.things {
      match thing.intersect(&ray) {
          Some(inter) => {
              if inter.dist < closest {
                  closest = inter.dist;
                  closest_inter = Some(inter);
              }
          },
          _ => {}
      }
  }
  closest_inter
}

fn test_ray(ray: &Ray, scene: &Scene) -> Option<f64> {
    closest_intersection(&ray, &scene).map(|isect| isect.dist)
}

fn trace_ray(ray: &Ray, scene: &Scene, depth: u32) -> Color {
    closest_intersection(&ray, &scene).map_or(Color::background(), |isect| shade(&isect, &scene, &ray, depth))
}

const MAXDEPTH: u32 = 5;

fn shade(isect: &Intersect, scene: &Scene, ray: &Ray, depth: u32) -> Color {
    let d = ray.dir;
    let pos = Vector::plus(&Vector::times(isect.dist, &d), &ray.start);
    let normal = isect.thing.normal(&pos);
    let reflect_dir = Vector::minus(&d, &Vector::times(2.0 * Vector::dot(&normal, &d), &normal));
    let natural_color = Color::plus(
        &Color::background(),
        &get_natural_color(isect.thing, &pos, &normal, &reflect_dir, &scene));
    let reflected_color = if depth >= MAXDEPTH { Color::grey() } else {
        get_reflection_color(isect.thing, pos, reflect_dir, &scene, depth)
    };
    Color::plus(&natural_color, &reflected_color)
}

fn get_reflection_color(thing: &Thing, pos: Vector, rd: Vector, scene: &Scene, depth: u32) -> Color {
    let ray = Ray { start: &pos, dir: &rd };
    Color::scale(thing.surface().reflect(&pos), &trace_ray(&ray, &scene, depth + 1))
}

fn get_natural_color(thing: &Thing, pos: &Vector, normal: &Vector, rd: &Vector, scene: &Scene) -> Color {
    let color_light = |light: &Light| {
        let ldis = Vector::minus(&light.pos, &pos);
        let livec = Vector::norm(&ldis);
        let neat_isect = test_ray(&Ray {start: &pos, dir: &livec}, &scene);
        let is_in_shadow = neat_isect.map_or(false, |isect| isect <= Vector::mag(&ldis));
        if is_in_shadow {
            None
        } else {
            let lcolor = Vector::dot_pos_neg(&livec, &normal, |illum| {
                Color::scale(illum, &light.color)
            }, |_| Color::black());
            let scolor = Vector::dot_pos_neg(&livec, &Vector::norm(&rd), |specular| {
                Color::scale(specular.powi(thing.surface().roughness()), &light.color)
            }, |_| Color::black());
            Some(Color::plus(&Color::times(&thing.surface().diffuse(pos), &lcolor),
                                           &Color::times(&thing.surface().specular(pos), &scolor)))
        }
    };
    scene.lights.iter().filter_map(color_light).fold(Color::black(), |acc, col| Color::plus(&acc, &col))
}


fn default_scene() -> Scene {
    Scene {
        things: vec![
            Box::new(Plane { norm: Vector {x: 0.0, y: 1.0, z: 0.0}, offset: 0.0, surface: Box::new(Checkerboard) }),
            Box::new(Sphere { center: Vector { x: 0.0, y: 1.0, z: -0.25}, radius: 1.0, surface: Box::new(Shiny) }),
            Box::new(Sphere { center: Vector { x: -1.0, y: 0.5, z: 1.5}, radius: 0.5, surface: Box::new(Shiny) })
        ],
        lights: vec![
            Light { pos: Vector { x: -2.0, y: 2.5, z: 0.0 }, color: Color { r: 0.49, g: 0.07, b: 0.07 } },
            Light { pos: Vector { x: 1.5, y: 2.5, z: 1.5 }, color: Color { r: 0.07, g: 0.07, b: 0.49 } },
            Light { pos: Vector { x: 1.5, y: 2.5, z: -1.5 }, color: Color { r: 0.07, g: 0.49, b: 0.071 } },
            Light { pos: Vector { x: 0.0, y: 3.5, z: 0.0 }, color: Color { r: 0.21, g: 0.21, b: 0.35 } }
        ],
        camera: Camera::new(Vector {x: 3.0, y: 2.0, z: 4.0}, Vector { x: -1.0, y: 0.5, z: 0.0})
    }
}

fn render_to_file(scene: &Scene, width: u32, height: u32, path: &Path) {
    let get_point = |x,y| {
        let recenter_x = |x: f64| (x - ((width as f64) / 2.0))  / (2.0 * (width as f64));
        let recenter_y = |y: f64| -(y - ((height as f64) / 2.0)) / (2.0 * (height as f64));
        Vector::norm(
            &Vector::plus(
                &scene.camera.forward,
                &Vector::plus(
                    &Vector::times(recenter_x(x as f64), &scene.camera.right),
                    &Vector::times(recenter_y(y as f64), &scene.camera.up))))
    };

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let ray = Ray { start: &scene.camera.pos, dir: &get_point(x,y) };
        let color = trace_ray(&ray, &scene, 0).to_drawing_color();
        image::Rgb(color)
    });

    let _ = img.save(path);
}

fn main() {
    println!("Rendering...");

    render_to_file(&default_scene(), 512, 512, &Path::new("out.png"));

    println!("Finished!  Open out.png to see the results.")
}
