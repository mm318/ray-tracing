// use std::io::Write;
mod aabb;
mod camera;
mod color;
mod hittable;
mod hittable_box;
mod hittable_sphere;
mod material;
mod ray;
mod texture;
mod utils;
mod vec3;

use utils::RayTracingFloat;

fn random_scene() -> hittable::HittableList {
    let mut objects = hittable::HittableList::new_empty();

    let checker = std::rc::Rc::new(texture::CheckerTexture::new(
        color::Color::new(0.2, 0.3, 0.1),
        color::Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = std::rc::Rc::new(material::Lambertian::new_with_texture(checker));
    objects.add(std::rc::Rc::new(hittable_sphere::Sphere::new(
        ray::Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let ref_point = ray::Point::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::random_double(&0.0, &1.0);
            let center = ray::Point::new(
                a as RayTracingFloat + 0.9 * utils::random_double(&0.0, &1.0),
                0.2,
                b as RayTracingFloat + 0.9 * utils::random_double(&0.0, &1.0),
            );

            if (&center - &ref_point).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo =
                        color::Color::random(&0.0, &1.0) * color::Color::random(&0.0, &1.0);
                    let sphere_material = std::rc::Rc::new(material::Lambertian::new(albedo));
                    let center2 =
                        &center + ray::Vector::new(0.0, utils::random_double(&0.0, &0.5), 0.0);
                    objects.add(std::rc::Rc::new(hittable_sphere::MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = color::Color::random(&0.5, &1.0);
                    let fuzz = utils::random_double(&0.0, &0.5);
                    let sphere_material = std::rc::Rc::new(material::Metal::new(albedo, fuzz));
                    objects.add(std::rc::Rc::new(hittable_sphere::Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    // glass
                    let sphere_material = std::rc::Rc::new(material::Dielectric::new(1.5));
                    objects.add(std::rc::Rc::new(hittable_sphere::Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = std::rc::Rc::new(material::Dielectric::new(1.5));
    objects.add(std::rc::Rc::new(hittable_sphere::Sphere::new(
        ray::Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = std::rc::Rc::new(material::Lambertian::new(color::Color::new(0.4, 0.2, 0.1)));
    objects.add(std::rc::Rc::new(hittable_sphere::Sphere::new(
        ray::Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = std::rc::Rc::new(material::Metal::new(color::Color::new(0.7, 0.6, 0.5), 0.0));
    objects.add(std::rc::Rc::new(hittable_sphere::Sphere::new(
        ray::Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // return objects;
    return hittable::HittableList::new(std::rc::Rc::new(
        hittable::BVH_Node::new_from_hittable_list(objects, &0.0, &1.0),
    ));
}

fn cornell_box() -> hittable::HittableList {
    let red = std::rc::Rc::new(material::Lambertian::new(color::Color::new(
        0.65, 0.05, 0.05,
    )));
    let white = std::rc::Rc::new(material::Lambertian::new(color::Color::new(
        0.73, 0.73, 0.73,
    )));
    let green = std::rc::Rc::new(material::Lambertian::new(color::Color::new(
        0.12, 0.45, 0.15,
    )));
    let light = std::rc::Rc::new(material::DiffuseLight::new(color::Color::new(
        15.0, 15.0, 15.0,
    )));

    let mut objects = hittable::HittableList::new_empty();

    objects.add(std::rc::Rc::new(hittable_box::YZ_Rect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    )));
    objects.add(std::rc::Rc::new(hittable_box::YZ_Rect::new(
        0.0, 555.0, 0.0, 555.0, 0.0, red,
    )));
    objects.add(std::rc::Rc::new(hittable_box::XZ_Rect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    objects.add(std::rc::Rc::new(hittable_box::XZ_Rect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(std::rc::Rc::new(hittable_box::XZ_Rect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(std::rc::Rc::new(hittable_box::XY_Rect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let mut box1 = std::rc::Rc::new(hittable_box::Box::new(
        ray::Point::new(0.0, 0.0, 0.0),
        ray::Point::new(165.0, 330.0, 165.0),
        white.clone(),
    )) as std::rc::Rc<dyn hittable::Hittable>;
    box1 = std::rc::Rc::new(hittable::Rotate_Y::new(box1, 15.0));
    box1 = std::rc::Rc::new(hittable::Translate::new(
        box1,
        ray::Vector::new(265.0, 0.0, 295.0),
    ));
    objects.add(box1);

    let mut box2 = std::rc::Rc::new(hittable_box::Box::new(
        ray::Point::new(0.0, 0.0, 0.0),
        ray::Point::new(165.0, 165.0, 165.0),
        white,
    )) as std::rc::Rc<dyn hittable::Hittable>;
    box2 = std::rc::Rc::new(hittable::Rotate_Y::new(box2, -18.0));
    box2 = std::rc::Rc::new(hittable::Translate::new(
        box2,
        ray::Vector::new(130.0, 0.0, 65.0),
    ));
    objects.add(box2);

    return objects;
}

fn ray_color(
    r: &ray::Ray,
    background: &color::Color,
    world: &dyn hittable::Hittable,
    depth: u32,
) -> color::Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return color::Color::zero();
    }

    // If the ray hits nothing, return the background color.
    let mut rec = hittable::HitRecord::new();
    if !world.hit(&r, &0.001, &RayTracingFloat::INFINITY, &mut rec) {
        return background.clone();
    }

    let mut scattered = ray::Ray::zero();
    let mut attenuation = color::Color::zero();
    let emitted = rec.material().emitted(&rec.u, &rec.v, &rec.p).clone();

    if !rec
        .material()
        .scatter(r, &rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }

    return emitted + attenuation * ray_color(&scattered, background, world, depth - 1);
}

fn render(
    image_width: &usize,
    image_height: &usize,
    cam: &camera::Camera,
    samples_per_pixel: &usize,
    max_depth: &u32,
    world: &dyn hittable::Hittable,
    background: &color::Color,
) {
    let mut buffer = vec![rgb::RGBA8::new(0, 0, 0, std::u8::MAX); image_width * image_height];

    for j in 0..*image_height {
        println!("Scanlines remaining: {}", image_height - j);
        // std::io::stdout().flush().unwrap();
        let row_offset = (image_height - 1 - j) * image_width;
        for i in 0..*image_width {
            let mut pixel_color = color::Color::zero();
            for _s in 0..*samples_per_pixel {
                let u = (i as RayTracingFloat + utils::random_double(&0.0, &1.0))
                    / (image_width - 1) as RayTracingFloat;
                let v = (j as RayTracingFloat + utils::random_double(&0.0, &1.0))
                    / (image_height - 1) as RayTracingFloat;
                let r = cam.get_ray(&u, &v);
                pixel_color += &ray_color(&r, background, world, *max_depth);
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
    let max_depth = 50 as u32;
    let dist_to_focus = 10.0;
    let vup = ray::Vector::new(0.0, 1.0, 0.0);

    if false {
        // settings taken from "Ray Tracing in One Weekend"

        // World
        let world = random_scene();
        let background = color::Color::new(0.70, 0.80, 1.00);

        // Image
        let aspect_ratio = 3.0 / 2.0;
        let image_width = 1200 as usize;
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        let samples_per_pixel = 500 as usize;

        // Camera
        let lookfrom = ray::Point::new(13.0, 2.0, 3.0);
        let lookat = ray::Point::new(0.0, 0.0, 0.0);
        let vfov = 20.0;
        let aperture = 0.1;
        let cam = camera::Camera::new(
            &lookfrom,
            &lookat,
            &vup,
            &vfov,
            &aspect_ratio,
            &aperture,
            &dist_to_focus,
            &0.0,
            &1.0,
        );

        // Render
        render(
            &image_width,
            &image_height,
            &cam,
            &samples_per_pixel,
            &max_depth,
            &world,
            &background,
        );
    } else {
        // settings taken from "Ray Tracing: The Next Weekend"

        // World
        let world = cornell_box();
        let background = color::Color::zero();

        // Image
        let aspect_ratio = 1.0;
        let image_width = 600 as usize;
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        let samples_per_pixel = 200 as usize;

        // Camera
        let lookfrom = ray::Point::new(278.0, 278.0, -800.0);
        let lookat = ray::Point::new(278.0, 278.0, 0.0);
        let vfov = 40.0;
        let aperture = 0.0;
        let cam = camera::Camera::new(
            &lookfrom,
            &lookat,
            &vup,
            &vfov,
            &aspect_ratio,
            &aperture,
            &dist_to_focus,
            &0.0,
            &1.0,
        );

        // Render
        render(
            &image_width,
            &image_height,
            &cam,
            &samples_per_pixel,
            &max_depth,
            &world,
            &background,
        );
    }
}
