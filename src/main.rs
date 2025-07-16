use mavis_lib;
use opencv::{core, imgcodecs, imgproc, prelude::*};

fn main() {
    let mat_to_save = mavis_lib::utils::screenshot_to_mat().unwrap();
}
