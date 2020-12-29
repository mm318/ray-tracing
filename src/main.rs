// use std::io::Write;
mod color;
mod hittable;
mod ray;
mod vec3;

fn hit_sphere(center: &ray::Point, radius: &f32, r: &ray::Ray) -> f32 {
    let oc = r.origin() - center.clone();
    let a = r.direction().length_squared();
    let half_b = vec3::dot(&oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: &ray::Ray) -> color::Color {
    let mut t = hit_sphere(&ray::Point::new(0.0, 0.0, -1.0), &0.5, &r);
    if t > 0.0 {
        let N = (r.at(&t) - ray::Vector::new(0.0, 0.0, -1.0)).unit_vector();
        return color::Color::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0) * 0.5;
    }

    let unit_direction = r.direction().unit_vector();
    t = 0.5 * (unit_direction.y() + 1.0);
    return color::Color::new(1.0, 1.0, 1.0) * (1.0 - t) + color::Color::new(0.5, 0.7, 1.0) * t;
}

/*
fn render_ppm(image_width: &usize, image_height: &usize) {
    let write_file = match std::fs::File::create("image.ppm") {
        Ok(file) => file,
        Err(err) => {
            println!("error: {:?}", err);
            return;
        }
    };
    let mut writer = std::io::BufWriter::new(&write_file);

    // Render
    match writeln!(&mut writer, "P3\n{} {}\n255", image_width, image_height) {
        Ok(_) => {}
        Err(err) => println!("error: {:?}", err),
    }

    for j in (0..*image_height).rev() {
        for i in 0..*image_width {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.25;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            match writeln!(&mut writer, "{} {} {}", ir, ig, ib) {
                Ok(_) => {}
                Err(err) => println!("error: {:?}", err),
            }
        }
    }
}

fn render_png(image_width: &usize, image_height: &usize) {
    let mut buffer = vec![rgb::RGBA8::new(0, 0, 0, std::u8::MAX); image_width * image_height];

    for j in 0..*image_height {
        println!("Scanlines remaining: {}", image_height - j);
        std::io::stdout().flush().unwrap();

        let row_offset = (image_height - 1 - j) * image_width;
        for i in 0..*image_width {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.25;

            buffer[row_offset + i] = color::write_color(&color::Color::new(r, g, b));
        }
    }
    println!("Done");
    // std::io::stdout().flush().unwrap();

    match lodepng::encode32_file("image.png", &buffer, *image_width, *image_height) {
        Ok(_) => println!("writing png succeeded"),
        Err(err) => println!("error: {:?}", err),
    }
}
*/

fn render(
    image_width: &usize,
    image_height: &usize,
    origin: &ray::Point,
    lower_left_corner: &ray::Point,
    horizontal: &ray::Vector,
    vertical: &ray::Vector,
) {
    let mut buffer = vec![rgb::RGBA8::new(0, 0, 0, std::u8::MAX); image_width * image_height];

    for j in 0..*image_height {
        println!("Scanlines remaining: {}", image_height - j);
        // std::io::stdout().flush().unwrap();
        let row_offset = (image_height - 1 - j) * image_width;
        for i in 0..*image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let r = ray::Ray::new(
                origin.clone(),
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(&r);
            buffer[row_offset + i] = color::write_color(&pixel_color);
        }
    }
    println!("Done");
    // std::io::stdout().flush().unwrap();

    match lodepng::encode32_file("image.png", &buffer, *image_width, *image_height) {
        Ok(_) => println!("writing png succeeded"),
        Err(err) => println!("error: {:?}", err),
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400 as usize;
    let image_height = (image_width as f32 / aspect_ratio) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = ray::Point::new(0.0, 0.0, 0.0);
    let horizontal = ray::Vector::new(viewport_width, 0.0, 0.0);
    let vertical = ray::Vector::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        &origin - &horizontal / 2.0 - &vertical / 2.0 - ray::Vector::new(0.0, 0.0, focal_length);

    // render_ppm(&image_width, &image_height);
    // render_png(&image_width, &image_height);
    render(
        &image_width,
        &image_height,
        &origin,
        &lower_left_corner,
        &horizontal,
        &vertical,
    );
}
