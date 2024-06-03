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
use traitement_image::detect_extract_webcam::*;
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

    let dist_squared: f64 = features1.iter().zip(features2.iter())
                        .map(|(&a, &b)| (a as f64 - b as f64).powi(2))
                        .sum();
    let dist = dist_squared.sqrt();
    dist
}

fn find_face_in_db(features: &[u8]) -> Result<Option<String>> {
    let conn = Connection::open("faces.db")?;
    let mut stmt = conn.prepare("SELECT name, features FROM faces")?;
    let face_iter = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        let features: Vec<u8> = row.get(1)?;
        Ok((name, features))
    })?;
    println!("not here");
    for face in face_iter {
        let (name, db_features) = face?;
        if compare_faces(features, &db_features) < 50.0 { // seuil de similarité
            return Ok(Some(name));
        }
    }
    Ok(None)
}

fn extract_feature(path : &str) -> Result<Vec<u8>, Box<dyn std::error::Error>>{
    create_db()?;

    let cascade_path = "/usr/share/opencv4/haarcascades/haarcascade_frontalface_alt.xml";
    let face_image = path;

    let faces = detect_faces(face_image, cascade_path)?;
    if faces.is_empty() {
        println!("No faces detected");
        return Ok(Vec::new());
    }

    let face_rect = faces[0]; // Pour simplifier, on prend le premier visage détecté
    let features = extract_face_features(face_image, face_rect)?;
    Ok(features.clone())
}

fn in_db(features: &[u8]) -> Result<bool>{
    match find_face_in_db(&features.clone())? {
        Some(name) => Ok(true),
        None => Ok(false),
    }
}

/* */
fn main()->Result<(),slint::PlatformError> {
    let _ = create_db();
    let app = AppWindow::new()?;
    app.set_viewable(false);
    app.on_camera({
        let app_handle = app.as_weak();
        move || {
            let app = app_handle.unwrap();
            let _ =extract_webcam();

        }
    });
    app.on_load({
        let app_handle = app.as_weak();
        move || {
            let app = app_handle.unwrap();
            if Path::new(&(app.get_input()).to_string()).exists() {
                let mut image = image::open(Path::new(&(app.get_input()).to_string())).expect("Error loading image").into_rgba8();
                let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                    image.as_raw(), image.width(), image.height());
                let new = Image::from_rgba8(pixel_buffer);
                app.set_picture_source(new);
            }
            else {app.set_viewable(true);}
        }
    });
    app.on_input_changed({
        let app_handle = app.as_weak();
        move |input| {
            let app = app_handle.unwrap();
            app.set_input(input);
        }
    });
    app.on_save_changed({
        let app_handle = app.as_weak();
        move |save| {
            let app = app_handle.unwrap();
            app.set_save(save);
        }
    });
    app.on_apply_gs({
        let app_handle = app.as_weak();
        move || {
            let app = app_handle.unwrap();
            if Path::new(&(app.get_input()).to_string()).exists() {
                let mut image = image::open(Path::new(&(app.get_input()).to_string())).expect("Error loading image").into_rgba8();
                let mut dynimage = DynamicImage::ImageRgba8(image);
                dynimage = convert_to_grey(&dynimage);
                image = dynimage.to_rgba8();
                let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                    image.as_raw(), image.width(), image.height());
                let new = Image::from_rgba8(pixel_buffer);
                app.set_picture_source(new);
            }
            else {app.set_viewable(true);}
        }
    });
    app.on_apply_filter({
        let app_handle = app.as_weak();
        move || {
            let app = app_handle.unwrap();
            if Path::new(&(app.get_input()).to_string()).exists() {
                let mut image = image::open(Path::new(&(app.get_input()).to_string())).expect("Error loading image").into_rgba8();
                let mut dynimage = DynamicImage::ImageRgba8(image);
                dynimage = filtrage(&dynimage);
                image = dynimage.to_rgba8();
                let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                    image.as_raw(), image.width(), image.height());
                let new = Image::from_rgba8(pixel_buffer);
                app.set_picture_source(new);
            }
            else {app.set_viewable(true);}
        }   
    });
    app.on_detect({
        let app_handle = app.as_weak();
        move || {
            let app = app_handle.unwrap();
            if Path::new(&(app.get_input()).to_string()).exists() {
                let _ = detect_image(&(app.get_input()));
            }
            else {app.set_viewable(true);}
        }
    });
    app.on_add_to_db({
        let app_handle = app.as_weak();
        move || {
            let app = app_handle.unwrap();
            if Path::new(&(app.get_input()).to_string()).exists() {
                let mut image = image::open(Path::new(&(app.get_input()).to_string())).expect("Error loading image").into_rgba8();
                let mut dynimage = DynamicImage::ImageRgba8(image);
                dynimage = filtrage(&dynimage);
                image = dynimage.to_rgba8();
                let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                    image.as_raw(), image.width(), image.height());
                let new = Image::from_rgba8(pixel_buffer);
                app.set_picture_source(new);
                let cascade_path = "/usr/share/opencv4/haarcascades/haarcascade_frontalface_alt.xml";
                let image_path = &app.get_input();
                let faces = detect_faces(image_path,cascade_path).expect("Not found");
                if faces.is_empty() {
                    println!("No faces detected");
                }
                else {
                    let face_rect = faces[0];
                    let features = extract_face_features(image_path, face_rect).expect("Not found");
                    match find_face_in_db(&features.clone()).expect("Not found") {
                        Some(name) => {
                            app.set_feedback(format!("This face was already in the database as {}",name).into());
                            //println!("Le visage est reconnu : {}", name)
                    },
                        None => {
                            //println!("Le visage n'est pas dans la base de données");
                            app.set_feedback("This face was not in the database and was saved".into());
                            // Sauvegarder le visage avec un nom arbitraire
                            save_face(&app.get_save(), features.clone());
                        }
                    }
                    
                }
                app.set_viewable2(true);
            }
            else {app.set_viewable(true);}
        }
    });
    app.run(); 
    Ok(())
}
/* 
fn main() -> Result<(), Box<dyn std::error::Error>> {
  /*  create_db()?;

    let cascade_path = "/usr/share/opencv4/haarcascades/haarcascade_frontalface_alt.xml";
    let face_image = "./mehdi_tlaghi.jpg";

    let faces = detect_faces(face_image, cascade_path)?;
    println!("not detect_faces");
    if faces.is_empty() {
        println!("No faces detected");
        return Ok(());
    }

    let face_rect = faces[0]; // Pour simplifier, on prend le premier visage détecté
    let features = extract_face_features(face_image, face_rect)?;
    println!("not extract");
    match find_face_in_db(&features.clone())? {
        Some(name) => println!("Le visage est reconnu : {}", name),
        None => {
            println!("Le visage n'est pas dans la base de données");
            // Sauvegarder le visage avec un nom arbitraire
            save_face("mehdi tlaghi", features.clone())?;
        }
    }
    */
    let _ =extract_webcam();
    
    Ok(())
}*/

