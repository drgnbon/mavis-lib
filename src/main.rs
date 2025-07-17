use crate::core::Vector;
use mavis_lib::{self, utils::Rectangle};
use opencv::{core, imgcodecs, imgproc, prelude::*};

fn main() {
    // mavis_lib::action::input_text_simulated(
    //     "text",
    //     "path_to_template",
    //     (1920, 1080),
    //     ((9, 8), (7, 6)),
    //     0.6,
    // );
    // let img = mavis_lib::utils::screenshot_area_to_mat((100, 100), (100, 100)).unwrap();
    // // Save image
    // imgcodecs::imwrite("filename.png", &img, &opencv::).unwrap();

    // let area = mavis_lib::utils::Area {
    //     s_x: 10,
    //     s_y: 10,
    //     e_x: 10,
    //     e_y: 10,
    // };

    // // Вызываем метод size() через трейт Rectangle
    // let ((min_x, min_y), (width, height)) = mavis_lib::utils::Area::size(&area);
    // println!("x: {} y: {} w: {} h: {}", min_x, min_y, width, height);
}
