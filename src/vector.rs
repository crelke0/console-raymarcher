pub mod vector {
  use std::ops;

  #[derive(Clone, Copy, Debug)]
  pub struct Vec3D {
    pub x: f32,
    pub y: f32,
    pub z: f32
  }

  impl Vec3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self{
      return Self {
        x: x,
        y: y,
        z: z
      };
    }
    pub fn length(&self) -> f32{
      (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    pub fn distance(&self, rhs: &Vec3D) -> f32 {
      (*self - *rhs).length()
    }
    pub fn normalize(&self) -> Option<Vec3D> {
      let l = self.length();
      if l == 0.0 { return None };
      Some(*self/l)
    }
    pub fn dot(&self, rhs: &Vec3D) -> f32 {
      self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }
    pub fn cross(&self, rhs: &Vec3D) -> Vec3D {
      Vec3D::new(
        self.y*rhs.z - self.z*rhs.y,
        self.z*rhs.x - self.x*rhs.z,
        self.x*rhs.y - self.y*rhs.x
      )
    }
    pub fn abs(&self) -> Vec3D {
      Vec3D::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
    pub fn max_component(&self) -> f32 {
      self.x.max(self.y).max(self.z)
    }
    pub fn max(&self, rhs: &Vec3D) -> Vec3D {
      Vec3D::new(self.x.max(rhs.x), self.y.max(rhs.y), self.z.max(rhs.z))
    }
  }
  impl ops::Add<Vec3D> for Vec3D {
    type Output = Self;
    fn add(self, rhs: Vec3D) -> Self::Output {
      Vec3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
  }
  impl ops::AddAssign<Vec3D> for Vec3D {
    fn add_assign(&mut self, rhs: Vec3D) {
      self.x += rhs.x;
      self.y += rhs.y;
      self.z += rhs.z;
    }
  }
  impl ops::Mul<f32> for Vec3D {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
      Vec3D::new(self.x*rhs, self.y*rhs, self.z*rhs)
    }
  }
  impl ops::Mul<Vec3D> for f32 {
    type Output = Vec3D;
    fn mul(self, rhs: Vec3D) -> Self::Output {
      Vec3D::new(rhs.x*self, rhs.y*self, rhs.z*self)
    }
  }
  impl ops::Div<f32> for Vec3D {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
      Vec3D::new(self.x/rhs, self.y/rhs, self.z/rhs)
    }
  }
  impl ops::Sub<Vec3D> for Vec3D {
    type Output = Vec3D;
    fn sub(self, rhs: Vec3D) -> Self::Output {
      Vec3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
  }
  impl ops::Neg for Vec3D {
    type Output = Vec3D;
    fn neg(self) -> Self::Output {
      self*-1.0
    }
  }
}