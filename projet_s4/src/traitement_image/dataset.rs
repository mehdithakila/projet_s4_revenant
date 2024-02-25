use image::*;
use std::fs;
use std::path::*;
use super::redim::*;
pub fn test_images(required_no:usize, path:&str)-> (Vec<DynamicImage>,Vec<DynamicImage>)

{
    let entries=fs::read_dir(path).unwrap();
    //images for training
    let mut images_path_for_training=Vec::new();
    let mut labels_for_training=Vec::new();
    let mut no_of_images_for_training=Vec::new();
    //images for testing
    let mut images_path_for_testing=Vec::new();
    let mut labels_for_testing=Vec::new();
    let mut no_of_images_for_testing=Vec::new();
    let mut images_target=Vec::new();
    let mut per_no=0;
    for entry in entries
    {

            let entry =entry.unwrap();
            let entry_path=entry.path();
            if entry_path.is_dir()
            {
                let images_der=fs::read_dir(&entry_path).unwrap();
                let cpt=images_der.count();
                if cpt>=required_no
                {
                    let images_der=fs::read_dir(&entry_path).unwrap();
                    let mut i=0;
                    for (idx,img_name) in images_der.enumerate()
                    {
                        let img_name = img_name.unwrap();
                        let img_path = img_name.path();
                        let img =redim(&img_path.to_string_lossy());
                        if i<required_no
                        {
                            images_path_for_training.push(img.clone());
                            labels_for_training.push(per_no);
                            if no_of_images_for_training.len()>per_no
                            {
                                no_of_images_for_training[per_no]+=1;
                            }
                            else
                            {
                                no_of_images_for_training.push(1);;
                            }
                            if i==0
                            {
                                images_target.push(entry_path.clone());
                            }
                        }
                        else
                        {
                            
                            images_path_for_testing.push(img);
                            labels_for_testing.push(per_no);
                            
                            if no_of_images_for_testing.len()>per_no
                            {
                                no_of_images_for_testing[per_no]+=1;
                            }
                            else
                            {
                                no_of_images_for_testing.push(1);
                            }
                        }
                        i+=1;
                    }
                    per_no+=1;
                }
            }
        
    }
    (images_path_for_training,images_path_for_testing)
    
    
}
