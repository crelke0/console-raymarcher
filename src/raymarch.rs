pub mod raymarch {
  use crate::vector::vector::Vec3D;
  
  #[derive(Debug)]
  pub struct Hit {
    pub pos: Vec3D,
    pub normal: Vec3D,
  }

  pub trait Raymarchable {
    fn sdf(&self, pos: &Vec3D) -> f32;
    fn normal(&self, pos: &Vec3D) -> Vec3D {
      let e = 0.001;
      let d = self.sdf(pos);
      let option = Vec3D::new(
        d - self.sdf(&(*pos - Vec3D::new(e, 0.0, 0.0))),
        d - self.sdf(&(*pos - Vec3D::new(0.0, e, 0.0))),
        d - self.sdf(&(*pos - Vec3D::new(0.0, 0.0, e)))
      ).normalize();
      match option {
        Some(e) => e,
        None => Vec3D::new(1.0, 0.0, 0.0)
      }
    }
  }

  pub struct Sphere {
    pub pos: Vec3D,
    pub r: f32
  }
  impl Sphere {
    pub fn new(pos: Vec3D, r: f32) -> Sphere {
      Sphere {
        pos: pos,
        r: r
      }
    }
  }
  impl Raymarchable for Sphere {
    fn sdf(&self, pos: &Vec3D) -> f32 {
      self.pos.distance(pos) - self.r
    }
    fn normal(&self, pos: &Vec3D) -> Vec3D {
      match (*pos - self.pos).normalize() {
        Some(x) => x,
        None => Vec3D::new(1.0, 0.0, 0.0)
      }
    }
  }

  pub struct Torus {
    pub pos: Vec3D,
    pub r1: f32,
    pub r2: f32
  }
  impl Torus {
    pub fn new(pos: Vec3D, r1: f32, r2: f32) -> Torus {
      Torus {
        pos: pos,
        r1: r1,
        r2: r2
      }
    }
  }
  impl Raymarchable for Torus {
    fn sdf(&self, pos: &Vec3D) -> f32 {
      let pos2d = Vec3D::new(pos.x - self.pos.x, 0.0, pos.z - self.pos.z);
      let q = Vec3D::new(pos2d.length() - self.r1, pos.y - self.pos.y, 0.0);
      return q.length() - self.r2;
    }
  }
  pub fn raymarch(ray_o: Vec3D, ray_d: Vec3D, objects: &[impl Raymarchable], max_steps: i32, smallest_step: f32) -> Option<Hit> {
    let mut pos = ray_o;
    for _ in 0..max_steps {
      let option_closest_info = objects
        .iter()
        .map(|o| (o.sdf(&pos), o)) // get sdf of each object
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap()); // find the object closest to the point
      let (dist, object) = match option_closest_info {
          Some((a, b)) => (a, b),
          None => { return None; } // length of objects is zero
      };
      if dist.abs() < smallest_step {
        return Some(Hit {
          pos: pos,
          normal: object.normal(&pos)
        });
      }
      pos += dist*ray_d;
    }
    return None;
  }
}