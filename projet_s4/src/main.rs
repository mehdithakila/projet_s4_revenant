mod traitement_image;
use traitement_image::redim::redim;
use traitement_image::filtrage::*;
use traitement_image::convert_to_grey::*;
use traitement_image::dataset::*;
use traitement_image::face_detection1::*;
use image::*;
use process_path;
use std::path::PathBuf;
slint::include_modules!();
use std::fs::File;
use std::io::{self, Write};
use ndarray::Array2;

fn main() -> Result<(), slint::PlatformError> {
    let srcpath = process_path::get_executable_path(); //recuperation dynamique du path ou se trouve
                                                       //l'executable
    let mut pathvalid = PathBuf::new();  
    match srcpath {
        None => println!("The process path could not be determined"),
        Some(mut p) => { p.pop(); p.pop(); p.pop(); p.pop(); pathvalid = p;}
    }   //test si srcpath est bien valide et revient au niveau du dossier projet_s4_revenant
        //le path sera retenu dans pathvalid qui pourra etre reutiliser pour tout les paths des
        //fichiers qu'on veut recupere ou enregistre. (il suffit de creer une nouvelle variable et
        //push le path a partir du dossier projet_s4_revenant)
    let mut path_image : String = pathvalid.to_str().unwrap().to_string();
    path_image.push_str("/data/lfw/test_bruit/Medianfilterp.png"); //ajouter le path pour trouver
                                                                       //l'image
    let path = path_image;

    let image = redim(&path);
    let image_filtre = filtrage(&image);
    let mut path_image_save = pathvalid.to_str().unwrap().to_string(); 
    path_image_save.push_str("/projet_s4/src/redim_image/filtre.jpg"); // path pour enregistrer l'image dans un fichier deja excitant ou sinon cela creer un nouveau fichier
    let enregistrer = image_filtre.save(&path_image_save);
    if enregistrer.is_ok(){
        let result = enregistrer.unwrap();
        println!("{}", true);
    }
    else{
        println!("{}", false);
    }
    
    // let path ="Alicia_Witt_0001.jpg";
    let img=redim(&path);
    let mut path_image_save = pathvalid.to_str().unwrap().to_string(); 
    path_image_save.push_str("/projet_s4/src/redim_image/grey.jpg"); 
    let grey_scale = convert_to_grey(&image);
    let enregistrer = grey_scale.save(&path_image_save);


    //let a =pca(&m,1);
    /*let mut file = match File::create("data.txt") {
        Err(why) => panic!("Impossible de créer le fichier : {}", why),
        Ok(file) => file,
    };
    // Parcourir les éléments de l'array2 et les écrire dans le fichier
    for row in m.genrows() {
        for &value in row {
            // Écrire la valeur dans le fichier
            write!(file, "{:?} ", value).expect("Impossible d'écrire dans le fichier");
        }
        // Aller à la ligne pour la prochaine rangée
        writeln!(file).expect("Impossible d'écrire dans le fichier");
    }
*/
    let path1="../../../images/ORL/";
    
    let (images_for_training,images_for_testing)=test_images(8,&path1);
    calculate_covariance_matrix(&images_for_training);
    let app = AppWindow::new()?;

    app.on_click({
        let app_handle = app.as_weak();
        move || {
            let app = app_handle.unwrap();
            dbg!("loading image");
        }
    });
    app.run()

}
