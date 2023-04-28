pub mod render {
  use crate::vector::vector::Vec3D;
  use crate::raymarch::raymarch::{Raymarchable, raymarch};
  pub struct Camera {
    pub pos: Vec3D,
    pub dir: Vec3D,
    pub focal: f32
  }
  impl Camera {
    pub fn new(pos: Vec3D, dir: Vec3D, focal: f32) -> Camera {
      Camera {
        pos: pos,
        dir: dir,
        focal: focal
      }
    }
  
    fn ray_dir(&self, uvx: f32, uvy: f32) -> Vec3D {
      let right = self.dir.cross(&Vec3D::new(0.0, 0.0, 1.0));
      let up = right.cross(&self.dir);
      right*uvx + up*uvy + self.dir*self.focal
    }
  }
  
  pub fn render(objects: &[&dyn Raymarchable], camera: &Camera, light: &Vec3D, width: u16, height: u16, sub_pixel_sample: u8, font_aspect: f32, max_steps: i32, smallest_step: f32) {
    let gradient: Vec<_> = String::from(" .:-=+*#%@").chars().collect();
    let length = (width*height).into();
    let mut output = String::with_capacity(length);
    let max_dim = width.max(height);
    let sub_pixel_sqrt = (sub_pixel_sample as f32).sqrt();
    for y in 0..height {
      for x in 0..width {
        let mut total_brightness = sub_pixel_sample as f32/10.0;
        for i in 0..sub_pixel_sample {
          let rx = (i as f32%sub_pixel_sqrt)/sub_pixel_sqrt;
          let ry = (i as f32/sub_pixel_sqrt)/sub_pixel_sqrt;
          let uvx = ((x as f32 + rx)/max_dim as f32*2.0-1.0)*font_aspect;
          let uvy = (y as f32 + ry)/max_dim as f32*2.0-1.0;
          let ray_dir = camera.ray_dir(uvx, uvy);
          let hit = match raymarch(camera.pos, ray_dir, objects, max_steps, smallest_step) {
            Some(x) => x,
            None => {
              continue;
            }
          };
          let light_dir = match (*light - hit.pos).normalize() {
            Some(x) => x,
            None => Vec3D::new(1.0, 0.0, 0.0)
          };
          total_brightness += (hit.normal.dot(&light_dir) + 1.0)/2.0;
        }
        let avg_brightness = total_brightness/sub_pixel_sample as f32;
        let char = gradient[(avg_brightness*((gradient.len() - 1) as f32)) as usize];
        output.push(char);
      }
      output.push('\n');
    }
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", output);
  }
}