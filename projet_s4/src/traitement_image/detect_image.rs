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
use image::*;
pub fn detect_image()
{
    let mut image = imgcodecs::imread("Alicia_Witt_0001.jpg", opencv::imgcodecs::IMREAD_COLOR).unwrap();

    // Check if the image is loaded successfully
    if image.size().unwrap().width == 0 || image.size().unwrap().height == 0 {
        panic!("Failed to load the image!");
    }

    // Load the Haar cascade classifier for face detection
    let mut face_cascade = objdetect::CascadeClassifier::new("/usr/share/opencv4/haarcascades/haarcascade_frontalface_alt.xml").unwrap();

    // Detect faces in the image
    let mut faces = Vector::<Rect>::new();
    face_cascade.detect_multi_scale(&image, &mut faces, 1.1, 2, 0,Size {width :30,height:30}, Size{width:0, height:0}).unwrap();
    // Draw rectangles around the detected faces
    for face in faces {
        let pt1 = Point::new(face.x, face.y);
        let pt2 = Point::new(face.x + face.width, face.y + face.height);        
        let rect =Rect::new(pt1.x, pt1.y, pt2.x - pt1.x, pt2.y - pt1.y);
        rectangle(&mut image,rect, Scalar::new(0.0, 255.0, 0.0, 0.0), 2, 8, 0).unwrap();
    }

    // Display the image with detected faces
    highgui::imshow("Face Detection", &image).unwrap();
    highgui::wait_key(0).unwrap();
}
