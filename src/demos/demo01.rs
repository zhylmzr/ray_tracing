use crate::ray::Ray;
use crate::vec3::Vec3;

use Vec3 as Color;
use Vec3 as Point3;

use std::fs;
use std::io::{self, Write};

const FILE_NAME: &'static str = "dist/demo01.ppm";

fn ray_color(r: &Ray) -> Vec3 {
    let unit_direction = r.direction().unit_vector();
    // 射线y轴 [-1, 1] 映射到 [0, 1]
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color(1.0, 1.0, 1.0) + t * Color(0.5, 0.7, 1.0)
}

pub fn run() {
    // 图片
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // 摄像头
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

    // 渲染
    let mut image_data = format!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let u = i as f64 / image_width as f64;
            let v = j as f64 / image_height as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let col = ray_color(&r);
            image_data.push_str(&col.gen_colors());
        }
    }

    fs::write(FILE_NAME, image_data).unwrap();

    eprintln!("\nDone.");
}
