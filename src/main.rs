use std::error::Error;

use my_ray_tracer::basics::image::*;
use my_ray_tracer::basics::{coord3::*, vec3::Vec3};

fn main() -> Result<(), Box<dyn Error>> {
    Coord3::new();
    Vec3::new();

    let mut img = Img::new_from(300, 200)?;
    let mut i = 0.0;
    for _ in 0..6000 {
        let _ = img.append(i, i, i);
        i += 0.00001;
    }
    if let Err(e) = img.produce() {
        if let Some(image_err) = e.downcast_ref::<ImageErr>() {
            image_err.handle();
        } else {
            return Err(e);
        }
    }

    Ok(())
}
