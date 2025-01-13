use super::material::Color;

use std::io::Write;

fn save_ppm(file_name: &str, image: &[Color], width: u32, height: u32) {
    let mut file = std::fs::File::create(file_name).unwrap();
    write!(file, "P3\n{} {}\n255\n", width, height).unwrap();
    for y in 0..height {
        for x in 0..width {
            let pixel = image[(height - y - 1) as usize * width as usize + x as usize];
            let r = (pixel.x * 255.99) as u32;
            let g = (pixel.y * 255.99) as u32;
            let b = (pixel.z * 255.99) as u32;
            write!(file, "{} {} {}\n", r, g, b).unwrap();
        }
    }
}
