use opencv::{
    Result,
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

pub fn detect_faces(image_path: &str, cascade_path: &str) -> Result<Vec<core::Rect>> {
    let img = imread(image_path, IMREAD_COLOR)?;
    let mut gray = Mat::default();
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let mut faces = types::VectorOfRect::new();
    let mut face_cascade = objdetect::CascadeClassifier::new(cascade_path)?;

    face_cascade.detect_multi_scale(
        &gray,
        &mut faces,
        1.1,
        3,
        objdetect::CASCADE_SCALE_IMAGE,
        core::Size::new(30, 30),
        core::Size::new(0, 0),
    )?;

    Ok(faces.to_vec())
}

pub fn extract_face_features(image_path: &str, face_rect: core::Rect) -> Result<Vec<u8>> {
    let img = imread(image_path, IMREAD_COLOR)?;
    let mut gray = Mat::default();
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

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

