use std::{f32::consts::TAU};
mod vector;
use crate::vector::vector::Vec3D;
mod raymarch;
use crate::raymarch::raymarch::*;
mod render;
use crate::render::render::{Camera, render};

const SMALLEST_STEP: f32 = 0.05;
const MAX_STEPS: i32 = 20;


fn demo(i: usize) {
  if i == 0 {
    let mut theta = 0f32;
    let mut counter = 0;
    let objects = [Sphere::new(Vec3D::new(-0.5, 0.0, 0.0), 2.5), Sphere::new(Vec3D::new(4.7, 1.2, 0.5), 2.0), Sphere::new(Vec3D::new(-3.7, -3.2, -2.5), 1.5)];
    let light = Vec3D::new(3.0, 3.0, -2.0);
    let mut camera = Camera::new(Vec3D::new(0.0, 0.0, 0.0), Vec3D::new(0.0, 1.0, 0.0), 1.0);
    loop {
      camera.pos = Vec3D::new(theta.cos()*20.0, theta.sin()*20.0, -10.0);
      camera.dir = match (Vec3D::new(0.0,0.0,2.0)-camera.pos).normalize() {
        Some(x) => x,
        None => {panic!();}
      };
      render(&objects, &camera, &light, 100, 70, 1, 0.4, MAX_STEPS, SMALLEST_STEP);
      counter += 1;
      theta += 0.1;
      theta %= TAU;
      if counter > 200 { break; }
    }
  } else if i == 1 {
    let mut objects = [Torus::new(Vec3D::new(0.0, 0.0, 0.0), 3.0, 1.0)];
    let light = Vec3D::new(2.0, 1.0, -2.0);
    let mut camera = Camera::new(Vec3D::new(0.0, 0.0, 0.0), Vec3D::new(0.0, 1.0, 0.0), 1.0);
    let mut theta = 0f32;
    let mut counter = 0;
    loop {
      camera.pos = Vec3D::new(theta.cos()*20.0, theta.sin()*20.0, 0.0);
      camera.dir = match (Vec3D::new(0.0,0.0,2.0)-camera.pos).normalize() {
        Some(x) => x,
        None => {panic!();}
      };
      objects[0].pos.z = (theta*1.0).sin()*1.2;
      render(&objects, &camera, &light, 100, 70, 1, 0.4, MAX_STEPS, SMALLEST_STEP);
      counter += 1;
      theta += 0.1;
      theta %= TAU;
      if counter > 200 { break; }
    }
  }
}
fn main() {
  demo(0);
}
