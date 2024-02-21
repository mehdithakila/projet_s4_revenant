use image::*;


pub fn redim(path: &str)->DynamicImage
{
    let image =open(path).unwrap();
    image.resize(224,224,image::imageops::FilterType::Lanczos3);
    //image.save_with_format("redim_image/new",ImageFormat::Jpeg).expect("Impossible d'enregistrer l'image");
    image
}
