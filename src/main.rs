use rays::Screen;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut screen = Screen::new(WIDTH, HEIGHT).unwrap();

    screen.run(|fb| {
        let height = fb.height() as f32;
        let width = fb.width() as f32;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let r = x as f32 / width;
                let g = y as f32 / height;
                let b = 0.2;

                let value = (((255.99 * r) as u32) << 16) +
                    (((255.99 * g) as u32) << 8) +
                    ((255.99 * b) as u32);
                fb.set(x, y, value);  // TODO: make this a color.
            }
        }
    });
}
