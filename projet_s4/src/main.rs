mod traitement_image;
use traitement_image::redim::redim;
use traitement_image::filtrage::*;
use traitement_image::convert_to_grey::*;
use traitement_image::dataset::*;
use traitement_image::face_detection1::*;
use traitement_image::detect::*;
use traitement_image::detect_image::*;
use image::*;
use process_path;
use std::path::PathBuf;
slint::include_modules!();
use std::fs::File;
use std::io::{self, Write};
use ndarray::Array2;
use opencv::{
    Result,
    prelude::*,
    objdetect,
    highgui,
    imgproc::*,
    core::*,
    types,
    videoio,
    imgcodecs,
};
fn main()->Result<(),slint::PlatformError> {
   
    let app = AppWindow::new()?;

    app.on_click({
        let app_handle = app.as_weak();
        move || {
            let app = app_handle.unwrap();
            detect();
        }
    });
    app.run(); 
    Ok(())
} 
