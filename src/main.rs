use rays::{Color, Ray, Screen, Vec3};
use rays::errors::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn color(ray: &Ray) -> Color {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)).into()
}

fn main() -> Result<()> {
    let mut screen = Screen::new(WIDTH, HEIGHT).unwrap();

    screen.run(|fb| {
        let height = fb.height() as f32;
        let width = fb.width() as f32;

        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horiz = Vec3::new(4.0, 0.0, 0.0);
        let vert = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let u = x as f32 / width;
                let v = y as f32 / height;
                let r = Ray::new(origin,
                                 lower_left_corner + u * horiz + v * vert);
                let col = color(&r);
                fb.set(x, y, col);
                

                
//                let r = x as f32 / width;
//                let g = y as f32 / height;
//                let b = 0.2;
//
//                let color = Color::new(r, g, b)?;
//                fb.set(x, y, color);
            }
        }
        Ok(())
    })?;

    Ok(())
}
