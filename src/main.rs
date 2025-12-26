mod vec3;

use vec3::Vec3;

fn render(width: u64, height: u64) {
    println!("P3\n{} {}\n255", width, height);

    for i in 0..width {
        eprint!("\rScanlines remaining: {} ", width - i);

        for j in 0..height {
            let r = i as f64 / (width - 1) as f64;
            let g = j as f64 / (width - 1) as f64;
            let b = 0.0;

            let ir = (r * 255.999) as u64;
            let ig = (g * 255.999) as u64;
            let ib = (b * 255.999) as u64;

            println!("{} {} {}", ir, ig, ib);
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
