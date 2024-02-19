use image::*;
//fonction qui converti en niveau gris les pixels de notre image 

pub fn convert_to_grey(image:&DynamicImage)->DynamicImage{
    let (width,height)=image.dimensions();
    let mut gray_image =DynamicImage::new_rgb8(width,height);
    for y in 0..height
        {
            for x in 0..width
            {
                    let pixel = image.get_pixel(x, y);
                    let luminance = (0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32) as u8;
                    let gray_pixel = Rgba([luminance,luminance, luminance, luminance]);
                    gray_image.put_pixel(x, y, gray_pixel);
            }
        }
    gray_image 
}
