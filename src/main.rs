use std::io::{self, Write, stdout};

use raytracing_rs::vec3::{Color, Vec3};

fn write_color(mut w: impl Write, color: Color) -> io::Result<()> {
    let ir = (color.x * 255.999) as u64;
    let ig = (color.y * 255.999) as u64;
    let ib = (color.z * 255.999) as u64;

    writeln!(w, "{} {} {}", ir, ig, ib)
}

fn render(width: u64, height: u64) {
    println!("P3\n{} {}\n255", width, height);

    for i in 0..width {
        eprint!("\rScanlines remaining: {} ", width - i);

        for j in 0..height {
            let color = Color {
                x: i as f64 / (width - 1) as f64,
                y: j as f64 / (width - 1) as f64,
                z: 0.0,
            };
            let _ = write_color(stdout(), color);
        }
    }
    eprintln!("\rDone.                 ");
}

fn main() {
    let image_width = 256;
    let image_height = 256;

    let v = Vec3::new(123.0, 344.0, 111.0);
    let mut v1 = -v;
    v1 *= 2.0;

    render(image_width, image_height);
}
