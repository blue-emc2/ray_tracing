use crate::vec3::Color;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = ((pixel_color.x * scale) * 255.999) as i32;
    let g = ((pixel_color.y * scale) * 255.999) as i32;
    let b = ((pixel_color.z * scale) * 255.999) as i32;

    println!("{} {} {}", r, g, b);
}
