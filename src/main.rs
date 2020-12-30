// use std::io::Write;
mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod utils;
mod vec3;

fn ray_color(r: &ray::Ray, world: &dyn hittable::Hittable, depth: u32) -> color::Color {
    if depth <= 0 {
        return color::Color::zero();
    }

    let mut rec = hittable::HitRecord::new();
    if world.hit(&r, &0.001, &f32::INFINITY, &mut rec) {
        let mut scattered = ray::Ray::zero();
        let mut attenuation = color::Color::zero();
        if rec
            .material()
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return color::Color::zero();
        }
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
            let mut pixel_color = color::Color::zero();
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
    let R = (std::f32::consts::PI / 4.0).cos();
    let mut world = hittable::HittableList::new_empty();

    let material_ground =
        std::rc::Rc::new(material::Lambertian::new(color::Color::new(0.8, 0.8, 0.0)));
    let material_center =
        std::rc::Rc::new(material::Lambertian::new(color::Color::new(0.1, 0.2, 0.5)));
    let material_left = std::rc::Rc::new(material::Dielectric::new(1.5));
    let material_right =
        std::rc::Rc::new(material::Metal::new(color::Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(hittable::Sphere::new(
        ray::Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(hittable::Sphere::new(
        ray::Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(hittable::Sphere::new(
        ray::Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(hittable::Sphere::new(
        ray::Point::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left,
    )));
    world.add(Box::new(hittable::Sphere::new(
        ray::Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let lookfrom = ray::Point::new(3.0, 3.0, 2.0);
    let lookat = ray::Point::new(0.0, 0.0, -1.0);
    let vup = ray::Vector::new(0.0, 1.0, 0.0);
    let dist_to_focus = (&lookfrom - &lookat).length();
    let aperture = 2.0;
    let cam = camera::Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        &20.0,
        &aspect_ratio,
        &aperture,
        &dist_to_focus,
    );

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
