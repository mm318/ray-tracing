// use std::io::Write;
mod camera;
mod color;
mod hittable;
mod ray;
mod vec3;
mod utils;

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

fn ray_color(r: &ray::Ray, world: &dyn hittable::Hittable, depth: u32) -> color::Color {
    if depth <= 0 {
        return color::Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = hittable::HitRecord::new();
    if world.hit(&r, &0.001, &f32::INFINITY, &mut rec) {
        // let target = rec.point() + rec.normal() + ray::Vector::random_in_unit_sphere();
        // let target = rec.point() + rec.normal() + ray::Vector::random_unit_vector();
        let target = rec.point() + ray::Vector::random_in_hemisphere(rec.normal());
        return ray_color(
            &ray::Ray::new(rec.point().clone(), target - rec.point()),
            world,
            depth - 1,
        ) * 0.5;
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return color::Color::new(1.0, 1.0, 1.0) * (1.0 - t) + color::Color::new(0.5, 0.7, 1.0) * t;
}

fn render(
    image_width: &usize,
    image_height: &usize,
    cam: &camera::Camera,
    samples_per_pixel: &usize,
    max_depth: &u32,
    world: &dyn hittable::Hittable,
) {
    let mut buffer = vec![rgb::RGBA8::new(0, 0, 0, std::u8::MAX); image_width * image_height];

    for j in 0..*image_height {
        println!("Scanlines remaining: {}", image_height - j);
        // std::io::stdout().flush().unwrap();
        let row_offset = (image_height - 1 - j) * image_width;
        for i in 0..*image_width {
            let mut pixel_color = color::Color::new(0.0, 0.0, 0.0);
            for _s in 0..*samples_per_pixel {
                let u = (i as f32 + utils::random_double(&0.0, &1.0)) / (image_width - 1) as f32;
                let v = (j as f32 + utils::random_double(&0.0, &1.0)) / (image_height - 1) as f32;
                let r = cam.get_ray(&u, &v);
                pixel_color += &ray_color(&r, world, *max_depth);
            }
            buffer[row_offset + i] = color::write_color(&pixel_color, samples_per_pixel);
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
    let samples_per_pixel = 100 as usize;
    let max_depth = 50 as u32;

    // World
    let mut world = hittable::HittableList::new_empty();
    world.add(Box::new(hittable::Sphere::new(
        ray::Point::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(hittable::Sphere::new(
        ray::Point::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let cam = camera::Camera::new(&aspect_ratio);

    // Render
    render(
        &image_width,
        &image_height,
        &cam,
        &samples_per_pixel,
        &max_depth,
        &world,
    );
}
