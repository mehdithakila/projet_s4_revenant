mod traitement_image;
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
