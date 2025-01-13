use super::material::Color;

use std::io::Write;

fn to_int(x: f64) -> isize {
    (x.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0 + 0.5) as isize
}

pub fn save_ppm(file_name: &str, image: &[Color], width: u32, height: u32) {
    let mut file = std::fs::File::create(file_name).unwrap();
    write!(file, "P3\n{} {}\n255\n", width, height).unwrap();
    for i in 0..width * height {
        let color = image[i as usize];
        write!(
            file,
            "{} {} {} ",
            to_int(color.r()),
            to_int(color.g()),
            to_int(color.b())
        )
        .unwrap();
    }
}
