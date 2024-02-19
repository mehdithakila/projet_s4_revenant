mod traitement_image;
use traitement_image::redim::redim;
fn main() {
    let path="../../data/lfw/Alicia_Witt/Alicia_Witt_0001.jpg";
    redim(&path);
}
