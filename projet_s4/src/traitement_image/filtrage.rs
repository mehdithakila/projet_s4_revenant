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
                    let value_pixel = ((pixel[3]  as u32) << 24) | ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2]  as u32);
                    mediane.push(value_pixel);
                }
            }
            mediane.sort();
            let mut index = 0;
            for i in mediane.iter(){
                if index == 4{
                    let pixel_filtre = Rgba([((*i  >> 16) & 0xFF) as u8 , ((*i >> 8) & 0xFF) as u8 , (*i & 0xFF) as u8 , ((*i >> 24) & 0xFF) as u8]);
                    image_filtre.put_pixel(x , y , pixel_filtre);
                    break;
                }
                index += 1;
            }
        }
    }
    image_filtre
}
