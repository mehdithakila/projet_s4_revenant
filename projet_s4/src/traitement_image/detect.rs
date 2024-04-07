use opencv::{
    Result,
    prelude::*,
    objdetect,
    highgui,
    imgproc,
    core,
    types,
    videoio,
};
pub fn detect()->Result<()>{
    let mut camera = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    // Use the following command to find the actual location of your xml files
    //sudo find / -name haarcascade_frontalface_default.xml
    //Haarcascade for eye detection
    //let xml = "/usr/local/share/opencv4/haarcascades/haarcascade_eye.xml";
    //Haarcascade for face detection
    //let xml = "/usr/local/share/opencv4/haarcascades/haarcascade_frontalface_default.xml";
    let xml ="/usr/share/opencv4/haarcascades/haarcascade_frontalface_default.xml";
    let mut face_detector = objdetect::CascadeClassifier::new(xml)?;
    let mut img = Mat::default();
    loop{
        camera.read(&mut img)?;
        let mut gray = Mat::default();
        imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
        let mut faces = types::VectorOfRect::new();
        face_detector.detect_multi_scale(
            &gray, 
            &mut faces, 
            1.1, 
            10,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size::new(10, 10),
            core::Size::new(0, 0)
        )?;
        println!("{:?}", faces);
        if faces.len() > 0{
            for face in faces.iter(){
                imgproc::rectangle(
                    &mut img,
                    face,
                    core::Scalar::new(0f64, 255f64, 0f64, 0f64),
                    2,
                    imgproc::LINE_8,
                    0
                )?;
            }    
        }
        highgui::imshow("gray", &img)?;
        highgui::wait_key(1)?;
    }
    Ok(())
}
