mod traitement_image;
use opencv::dnn::DNN_BACKEND_CUDA;
use slint::Image;
use slint::SharedPixelBuffer;
use slint::Rgba8Pixel;
use std::path::Path;
use traitement_image::redim::redim;
use traitement_image::filtrage::*;
use traitement_image::convert_to_grey::*;
use traitement_image::dataset::*;
use traitement_image::face_detection1::*;
use traitement_image::detect::*;
use traitement_image::detect_image::*;
use traitement_image::detect_extract_image::*;
use traitement_image::data_base::*;
use rusqlite::{params, Connection, Result};
use image::*;
use process_path;
use std::path::PathBuf;
slint::include_modules!();
use std::fs::File;
use std::io::{self, Write};
use ndarray::Array2;
use opencv::{
    prelude::*,
    objdetect,
    highgui,
    imgproc::*,
    core::*,
    types,
    videoio,
    imgcodecs,
};

fn compare_faces(features1: &[u8], features2: &[u8]) -> f64 {
    let dist = features1.iter().zip(features2.iter())
                        .map(|(a, b)| (a - b).pow(2))
                        .sum::<u8>() as f64;
    dist.sqrt()
}

fn find_face_in_db(features: &[u8]) -> Result<Option<String>> {
    let conn = Connection::open("faces.db")?;
    let mut stmt = conn.prepare("SELECT name, features FROM faces")?;
    let face_iter = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        let features: Vec<u8> = row.get(1)?;
        Ok((name, features))
    })?;

    for face in face_iter {
        let (name, db_features) = face?;
        if compare_faces(features, &db_features) < 50.0 { // seuil de similarité
            return Ok(Some(name));
        }
    }
    Ok(None)
}


fn main()->Result<(),slint::PlatformError> {
    let app = AppWindow::new()?;
    app.on_camera({
        let app_handle = app.as_weak();
        move || {
            let app = app_handle.unwrap();
            detect();

        }
    });
    app.on_click({
        let app_handle = app.as_weak();
        move || {
            let app = app_handle.unwrap();
            dbg!("{}",String::from(app.get_input()));
            let mut image = image::open(Path::new(&(app.get_input()).to_string())).expect("Error loading image").into_rgba8();
            let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                image.as_raw(), image.width(), image.height());
            let new = Image::from_rgba8(pixel_buffer);
            app.set_picture_source(new);
        }
    });
    app.on_input_changed({
        let app_handle = app.as_weak();
        move |input| {
            let app = app_handle.unwrap();
            app.set_input(input);
        }
    });
    app.on_apply_gs({
        let app_handle = app.as_weak();
        move || {
            let app = app_handle.unwrap();
            let mut image = image::open(Path::new(&(app.get_input()).to_string())).expect("Error loading image").into_rgba8();
            let mut dynimage = DynamicImage::ImageRgba8(image);
            dynimage = convert_to_grey(&dynimage);
            image = dynimage.to_rgba8();
            let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                image.as_raw(), image.width(), image.height());
            let new = Image::from_rgba8(pixel_buffer);
            app.set_picture_source(new);
        }
    });
    app.on_apply_filter({
        let app_handle = app.as_weak();
        move || {
            let app = app_handle.unwrap();
            let mut image = image::open(Path::new(&(app.get_input()).to_string())).expect("Error loading image").into_rgba8();
            let mut dynimage = DynamicImage::ImageRgba8(image);
            dynimage = filtrage(&dynimage);
            image = dynimage.to_rgba8();
            let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                image.as_raw(), image.width(), image.height());
            let new = Image::from_rgba8(pixel_buffer);
            app.set_picture_source(new);
        }   
    });
    app.on_detect({
        let app_handle = app.as_weak();
        move || {
            let app = app_handle.unwrap();
            detect_image();
        }
    });
    app.run(); 
    Ok(())
}
/*
fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_db()?;

    let cascade_path = "/usr/share/opencv4/haarcascades/haarcascade_frontalface_alt.xml";
    let face_image = "/home/loic/epita/S4/Projet/projet_s4_revenant/projet_s4/src/Alicia_Witt_0001.jpg";

    let faces = detect_faces(face_image, cascade_path)?;
    if faces.is_empty() {
        println!("No faces detected");
        return Ok(());
    }

    let face_rect = faces[0]; // Pour simplifier, on prend le premier visage détecté
    let features = extract_face_features(face_image, face_rect)?;
    
    match find_face_in_db(&features.clone())? {
        Some(name) => println!("Le visage est reconnu : {}", name),
        None => {
            println!("Le visage n'est pas dans la base de données");
            // Sauvegarder le visage avec un nom arbitraire
            save_face("Alicia witt", features.clone())?;
        }
    }
    
    Ok(())
}
*/
