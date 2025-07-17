use crate::input::mouse::mouse_control::{mouse_left_click, mouse_set_position};
use crate::input::{keyboard, mouse};
use crate::utils::DisplayArea;
use crate::vision::ocv::ocv::find_target_in_image;
use crate::vision::{ocv, tsrt};
use crate::{utils, vision};
use opencv::{core, imgcodecs, imgproc, prelude::*};
use std::thread;
use std::time::{Duration, Instant};



pub fn input_text_simulated(
    rec: f32,
    target: &Mat,
    active_area: &DisplayArea,
    resolution: (u32, u32),
    delay: Duration,
    text: &str,
) -> Result<(), String> {
    let screenshot: Mat = match utils::screenshot_area_to_mat(active_area) {
        Ok(mat) => mat,
        Err(e) => return Err(format!("input_text_simulated: {}", e)),
    };
    let mouse_pos = match ocv::ocv::find_target_in_image(rec, &screenshot, target) {
        Ok(relative_area) => relative_area.from_relative(active_area).get_average_point(),
        Err(e) => return Err(format!("input_text_simulated: {}", e)),
    };

    mouse::mouse_control::mouse_set_position(
        mouse_pos.0 as u32,
        mouse_pos.1 as u32,
        resolution.0,
        resolution.1,
    );

    mouse::mouse_control::mouse_left_click(delay);
    keyboard::keyboard_control::type_unicode_text(text);

    Ok(())
}

pub fn wait_for_image(
    recognition: f32,
    target: &Mat,
    active_area: &DisplayArea,
    fps_lock: u8,
    maximum_expectation: Duration,
) -> Result<(), String> {

    let min_delay_between_ticks: Duration = Duration::from_secs_f64(1. / fps_lock as f64);
    let st_func_time: Instant = Instant::now();

    while st_func_time.elapsed() < maximum_expectation {
        let st_tick_time: Instant = Instant::now();

        // Сделать скриншота
        let screenshot = match utils::screenshot_area_to_mat(active_area) {
            Ok(img) => img,
            Err(e) => return Err(format!("Ошибка захвата скриншота: {:?}", e)),
        };


        // Проверка наличия шаблона
        match ocv::ocv::is_target_on_image(recognition,&screenshot, target, active_area) {
            Ok(true) => return Ok(()),
            Ok(false) => (),
            Err(e) => return Err(format!("Ошибка поиска шаблона: {:?}", e)),
        }


        let elapsed_tick = st_tick_time.elapsed();
        //println!("FPS now: {:?}", 1. / elapsed_tick.as_secs_f64());
        if elapsed_tick < min_delay_between_ticks {
            while st_tick_time.elapsed() < min_delay_between_ticks {
                thread::yield_now();
            }
        }
    }


    // Если все попытки не удались, возвращаем ошибку
    Err(format!("target not founded"))
}

pub fn click_on_target(
    recognition: f32,
    target: &Mat,
    active_area: &DisplayArea,
    screen_resolution: (u32,u32),
    delay: Duration,
) -> Result<(), String> {

    // сделать скриншот
    let screenshot = match utils::screenshot_area_to_mat(active_area) {
        Ok(img) => img,
        Err(e) => return Err(format!("Ошибка захвата скриншота: {:?}", e)),
    };


    //найти место
    let point = match ocv::ocv::find_target_in_image(recognition,&screenshot, target) {
        Ok(area) => area.from_relative(active_area).get_average_point(),
        Err(e) => {
            eprintln!("Ошибка нахождения образца: {:?}", e);
            (960,520)
            //return Err("click_on_target: Not founded".to_string())
        }
    };

    //перенести мышь
    mouse_set_position(point.0 as u32, point.1 as u32,
         screen_resolution.0, screen_resolution.1);

    //кликнуть
    mouse_left_click(delay);

    Ok(())
}

pub fn find_object(
    recognition: f32,
    source: &Mat,
    target: &Mat,
) -> Result<DisplayArea,String> {

    match find_target_in_image(recognition, source, target) {
        Ok(area)=> {
            return Ok(area)
        }
        Err(e)=>{
            return Err(format!("find_object: Error occurated from finding object: {}",e).to_string())
        }
    }

}

pub fn extract_text(
    img: &Mat,
    path_to_cache_file: &str,
) -> Result<String, String> {


    let _ = imgcodecs::imwrite(path_to_cache_file, img, &core::Vector::new());
    let text = tsrt::tsrt::read_text_from_image(path_to_cache_file);
    let _ = std::fs::remove_file(path_to_cache_file);

    Ok(text)

}
