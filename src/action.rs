use crate::input::mouse::mouse_control::{mouse_left_click, mouse_set_position};
use crate::input::{keyboard, mouse};
use crate::utils::DisplayArea;
use crate::vision::{ocv, tsrt};
use crate::{utils, vision};
use opencv::{core, imgcodecs, imgproc, prelude::*};
use std::thread;
use std::time::{Duration, Instant};



pub fn input_text_simulated(
    treshold: f32,
    template: &Mat,
    perm_area: ((i32, i32), (u32, u32)),
    resolution: (u32, u32),
    text: &str,
) {
}

pub fn wait_for_image(
    threshold: f32,
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
        match ocv::ocv::is_template_on_image(threshold,&screenshot, target, active_area) {
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
    Err(format!("Template not founded"))
}

pub fn click_on_target(
    threshold: f32,
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
    let point = match ocv::ocv::find_template_in_image(threshold,&screenshot, target) {
        Ok(Some(area)) => active_area.from_relative(area).get_average_point(),
        Ok(None) => {
            eprintln!("Образец не найден");
            (0, 0)
        }
        Err(e) => {
            eprintln!("Ошибка нахождения образца: {:?}", e);
            (0, 0)
        }
    };

    //перенести мышь
    mouse_set_position(point.0 as u32, point.1 as u32,
         screen_resolution.0, screen_resolution.1);

    //кликнуть
    mouse_left_click(delay);

    Ok(())
}

pub fn find_object() -> DisplayArea {
    DisplayArea::from_points(0, 0, 0, 0)
}

pub fn extract_text() -> Result<String, String> {
    Ok(format!("ZZZZZZ"))
}
