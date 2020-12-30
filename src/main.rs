// use std::io::Write;
mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod utils;
mod vec3;

fn random_scene() -> hittable::HittableList {
    let mut world = hittable::HittableList::new_empty();

    let ground_material =
        std::rc::Rc::new(material::Lambertian::new(color::Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(hittable::Sphere::new(
        ray::Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let ref_point = ray::Point::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::random_double(&0.0, &1.0);
            let center = ray::Point::new(
                a as f64 + 0.9 * utils::random_double(&0.0, &1.0),
                0.2,
                b as f64 + 0.9 * utils::random_double(&0.0, &1.0),
            );

            if (&center - &ref_point).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo =
                        color::Color::random(&0.0, &1.0) * color::Color::random(&0.0, &1.0);
                    let sphere_material = std::rc::Rc::new(material::Lambertian::new(albedo));
                    world.add(Box::new(hittable::Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = color::Color::random(&0.5, &1.0);
                    let fuzz = utils::random_double(&0.0, &0.5);
                    let sphere_material = std::rc::Rc::new(material::Metal::new(albedo, fuzz));
                    world.add(Box::new(hittable::Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    // glass
                    let sphere_material = std::rc::Rc::new(material::Dielectric::new(1.5));
                    world.add(Box::new(hittable::Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = std::rc::Rc::new(material::Dielectric::new(1.5));
    world.add(Box::new(hittable::Sphere::new(
        ray::Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = std::rc::Rc::new(material::Lambertian::new(color::Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(hittable::Sphere::new(
        ray::Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = std::rc::Rc::new(material::Metal::new(color::Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(hittable::Sphere::new(
        ray::Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    return world;
}

fn ray_color(r: &ray::Ray, world: &dyn hittable::Hittable, depth: u32) -> color::Color {
    if depth <= 0 {
        return color::Color::zero();
    }

    let mut rec = hittable::HitRecord::new();
    if world.hit(&r, &0.001, &f64::INFINITY, &mut rec) {
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
                let u = (i as f64 + utils::random_double(&0.0, &1.0)) / (image_width - 1) as f64;
                let v = (j as f64 + utils::random_double(&0.0, &1.0)) / (image_height - 1) as f64;
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
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200 as usize;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 500 as usize;
    let max_depth = 50 as u32;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = ray::Point::new(12.0, 2.0, 3.0);
    let lookat = ray::Point::new(0.0, 0.0, 0.0);
    let vup = ray::Vector::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
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
