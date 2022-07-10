mod basics;

use basics::Vector3;
use std::{fs::File, io::Write};


fn main() {
    let first = Vector3::new(1.0, 2.0, 3.0);
    let second = Vector3::new(4.0, 5.0, 6.0);
    println!("{:?}", first.cross(&second).normalized());
}

fn draw_image() -> std::io::Result<()> {
    let width = 1024;
    let height = 512;
    
    let mut file = File::create("img.ppm")?;
    file.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes())?;

    for y in (0..height).rev() {
        for x in 0..width {
            let r = (x as f64) / (width as f64);
            let g = (y as f64) / (height as f64);
            let b = 0.5;
            
            let r = (256.0 * r) as u8;
            let g = (256.0 * g) as u8;
            let b = (256.0 * b) as u8;
            file.write_all(format!("{} {} {}\n", r, g, b).as_bytes())?;
        }
    }
    Ok(())
}
