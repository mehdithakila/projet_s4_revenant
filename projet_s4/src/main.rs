mod traitement_image;
use traitement_image::redim::redim;
use traitement_image::filtrage::*;
use traitement_image::convert_to_grey::*;
use image::*;
use process_path;
use std::path::PathBuf;

fn main() {
    let srcpath = process_path::get_executable_path();
    let mut pathvalid = PathBuf::new();
    match srcpath {
        None => println!("The process path could not be determined"),
        Some(mut p) => { p.pop(); p.pop(); p.pop(); p.pop(); pathvalid = p;}
    }
    pathvalid.push("data/lfw/Alicia_Witt/Alicia_Witt_0001.jpg");
    let path = pathvalid.to_str().unwrap();
    println!("{}", path);
    let image = redim(&path);
    let image_filtre = filtrage(&image);
    /*
    let path = "redim_/image/new2.jpg";
    let enregistrer = image_filtre.save_with_format(&path, ImageFormat::Png);
    if enregistrer.is_ok(){
        let result = enregistrer.unwrap();
        println!("{}", true);
    }
    else{
        println!("{}", false);
    }
    */
}
