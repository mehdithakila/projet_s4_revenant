use opencv::dnn::DNN_BACKEND_CUDA;
use opencv::{
    prelude::*,
    objdetect,
    highgui,
    imgproc,
    core,
    types,
    videoio,
    imgcodecs::{imread, IMREAD_COLOR},
    face::{LBPHFaceRecognizer, FaceRecognizer},
};
use crate::traitement_image::data_base::* ;
use rusqlite::{params, Connection};
use std::fmt;
pub enum MyError {
    OpenCv(opencv::Error),
    Sqlite(rusqlite::Error),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::OpenCv(err) => write!(f, "OpenCV error: {}", err),
            MyError::Sqlite(err) => write!(f, "SQLite error: {}", err),
        }
    }
}

impl From<opencv::Error> for MyError {
    fn from(err: opencv::Error) -> MyError {
        MyError::OpenCv(err)
    }
}

impl From<rusqlite::Error> for MyError {
    fn from(err: rusqlite::Error) -> MyError {
        MyError::Sqlite(err)
    }
}

fn compare_faces(features1: &[u8], features2: &[u8]) -> f64 {
        let dist_squared: f64 = features1.iter().zip(features2.iter())
                        .map(|(&a, &b)| (a as f64 - b as f64).powi(2))
                        .sum();
    let dist = dist_squared.sqrt();
    dist
}

fn find_face_in_db(features: &[u8]) -> Result<Option<String>,MyError> {
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

pub fn extract_face_features_cam(image: &Mat, face_rect: core::Rect) -> Result<Vec<u8>,MyError> {
    let mut gray = Mat::default();
    imgproc::cvt_color(&image, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let face = Mat::roi(&gray, face_rect)?;

    // Assurer que la matrice face est continue
    let mut face_contiguous = Mat::default();
    face.copy_to(&mut face_contiguous)?;

    let mut lbph = <dyn LBPHFaceRecognizer>::create(1, 8, 8, 8, 50.0)?; // Paramètres par défaut

    // Convertir la région d'intérêt en un vecteur de Mat
    let faces = types::VectorOfMat::from_iter(vec![face_contiguous.clone()]);

    // L'étiquette est arbitraire ici car nous utilisons une seule image
    let labels = types::VectorOfi32::from_iter(vec![0]);

    lbph.train(&faces, &labels)?;

    // Extraire l'histogramme
    let histograms = lbph.get_histograms()?;

    // Convertir les histogrammes en un vecteur de bytes
    let histogram = histograms.get(0)?;
    let mut features_vec = Vec::new();
    features_vec.extend_from_slice(histogram.data_bytes()?);

    Ok(features_vec)
}

pub fn extract_webcam()->Result<(),MyError>{
    create_db()?;
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
        //println!("{:?}", faces);
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
                let features = extract_face_features_cam(&img, face)?;

                match find_face_in_db(&features.clone())? 
                { 
                    Some(name) => println!("Le visage est reconnu : {}", name),
                    None => {
                        println!("Le visage n'est pas dans la base de données");
                        let _ = save_face("007",features.clone());
                    }
                        // Sauvegarder le visage avec un nom arbitraire
                            
                                                
                            
                }
            }    
        }
        

        highgui::imshow("gray", &img)?;
        let test = highgui::wait_key(1)?;
        if test == 32 {
            let _ = highgui::destroy_all_windows();
            break;}

    }
    Ok(())
}
