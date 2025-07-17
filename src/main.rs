use std::time::Duration;

use crate::core::Vector;
use mavis_lib::{self, action::{click_on_target, wait_for_image}, input::mouse::mouse_control, utils::DisplayArea, vision::ocv};
use opencv::{core, imgcodecs, imgproc, prelude::*};
use mavis_lib::utils;
fn main() {
    // mavis_lib::action::input_text_simulated(
    //     "text",
    //     "path_to_target",
    //     (1920, 1080),
    //     ((9, 8), (7, 6)),
    //     0.6,
    // );
    // let img = mavis_lib::utils::screenshot_area_to_mat((100, 100), (100, 100)).unwrap();
    // // Save image
    // imgcodecs::imwrite("filename.png", &img, &opencv::).unwrap();

    
    // // Вызываем метод size() через трейт Rectangle
    // let ((min_x, min_y), (width, height)) = mavis_lib::utils::Area::size(&area);
    // println!("x: {} y: {} w: {} h: {}", min_x, min_y, width, height);
    

    // let target = imgcodecs::imread("../image.png", imgcodecs::IMREAD_COLOR).unwrap();



    // let active_area = DisplayArea::from_rectangle (
    //     960,540,960,540
    // );

    // let active_area2 = DisplayArea::from_rectangle (
    //     0,0,1920,1080
    // );

    // let source = utils::screenshot_area_to_mat(&active_area2).unwrap();

    
    // ocv::ocv::is_target_on_image(
    //     0.75, 
    //     &source, 
    //     &target, 
    //     &active_area
    // ).unwrap();


    // match wait_for_image(
    //     0.75, 
    //     &target, 
    //     &active_area, 
    //     1, 
    //     Duration::from_secs(20)
    // ){
    //     Ok(())=>{
    //         println!("Попался шараноид");
    //     }
    //     Err(_)=>{
    //         println!("хуй");
    //     }
    // }


    


    // click_on_target(
    //     0.75, 
    //     &target, 
    //     &active_area, 
    //     (1920,1080), 
    //     Duration::from_millis(200)
    // ).unwrap();


    // let t = mavis_lib::action::extract_text(&target, "cache.png").unwrap();
    // println!("Text occurated: {}",t);

    

    // let cords = ocv::ocv::find_target_in_image(0.75, &source, &target).unwrap();
    // let (x,y) = cords.from_relative(active_area).get_average_point();
    // println!("x: {},y: {}",x,y);
    // mouse_control::mouse_set_position(x as u32, y as u32, 1920, 1080);


    let active_area = DisplayArea::from_points(200, 0, 1920, 1080);
    //let source = utils::screenshot_area_to_mat(&active_area).unwrap();
    let target = imgcodecs::imread("../image.png", imgcodecs::IMREAD_COLOR).unwrap();




    let _ = mavis_lib::action::input_text_simulated(
        0.75,
        &target,
        &active_area,
        (1920, 1080),
        Duration::from_millis(100),
        "aboba",
    )
    .unwrap();




}
