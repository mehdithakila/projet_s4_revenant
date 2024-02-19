use image::*;
use std::panic;

pub fn filtrage(image : &DynamicImage) -> DynamicImage{
    let (width, height) = image.dimensions();
    let mut image_filtre = DynamicImage::new_rgb8(width, height);
    for x in 0..width{
        for y in 0..height{
            let mut mediane = vec![];
            for i in 0..3{
                for j in 0..3{
                    let pixel = panic::catch_unwind(|| image.get_pixel(x + i - 1,y + j - 1));
                    if pixel.is_ok(){
                        let pixel = pixel.unwrap();
                        let value_pixel = (0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32) as u8;
                        mediane.push(value_pixel);
                    }
                    else{
                        mediane.push(0);
                    }
                }
            }
            mediane.sort();
            let mut index = 0;
            for i in mediane.iter(){
                if index == 4{
                    let pixel_filtre = Rgba([*i , *i , *i, *i]);
                        image_filtre.put_pixel(x , y , pixel_filtre);
                    break;
                }
                index += 1;
            }
        }
    }
    image_filtre
}
