use image::*;
use std::panic;

pub fn filtrage(image : &DynamicImage) -> DynamicImage{
    let (width, height) = image.dimensions();
    let mut image_filtre = DynamicImage::new_rgb8(width, height);
    for x in 1..width-1{
        for y in 1..height-1{
            let mut mediane = vec![];
            for i in 0..3{
                for j in 0..3{
                    let pixel = image.get_pixel(x + i - 1,y + j - 1);
                    let value_pixel = (0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32) as u8;
                    mediane.push(value_pixel);
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
