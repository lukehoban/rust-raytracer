# Rust Raytracer

A simple raytracer implementation to try out Rust.  

![Two spheres and a plane.](out.png)

## Notes

When you need to store references in data structures, you need to parameterize them over lifetimes.  This was something I didn't see covered in most intro's to Rust's ownership model.

```rust
struct Intersect<'a> { thing: &'a Thing, dist: f64}

fn closest_intersection<'a>(ray: &Ray, scene: &'a Scene) -> Option<Intersect<'a>> { /* */ }
```

Some of the higher-level `Iterator` APIs are nice here - for example:

```rust
scene.lights.iter().filter_map(color_light).fold(Color::black(), |acc, col| Color::plus(&acc, &col))
```

Authoring your own higher order functions is surprisingly cumbersome:

```rust
fn dot_pos_neg<T, F1, F2>(v1: &Vector, v2: &Vector, pos: F1, neg: F2) -> T
        where F1: FnOnce(f64) -> T, F2: FnOnce(f64) -> T {
    let d = Vector::dot(&v1, &v2);
    if d > 0.0 { pos(d) } else { neg(d) }
}
```

The `Vector` struct could have implemented the `Copy` trait and avoid all the borrowing of references to Vector objects.  Performance wise this version is slightly faster.
