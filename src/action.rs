use std::time::Duration;

use crate::input::mouse::mouse_control::{mouse_left_click, mouse_set_position};
use crate::input::{keyboard, mouse};
use crate::utils::average_point;
use crate::{utils, vision};
use crate::vision::{ocv, tsrt};
use opencv::{core, imgcodecs, imgproc, prelude::*};
use std::thread;

pub fn input_text_simulated(
    text: &str,
    path_to_template: &str,
    resolution: (u32, u32),
    perm_area: ((i32, i32), (u32, u32)),
    treshold: f32,
) {
}

pub fn wait_for_image(
    template: &Mat,
    threshold: f32,
    max_attempts: u32,
    delay: Duration,
    active_zone_start: (i32, i32),
    active_zone_end: (u32, u32),
) -> Result<(), String> {

    for attempt in 1..=max_attempts {
        // Сделать скриншота 
        let screenshot = match utils::screenshot_area_to_mat(active_zone_start, active_zone_end) {
            Ok(img) => img,
            Err(e) => return Err(format!("Ошибка захвата скриншота: {:?}", e)),
        };

        // Проверка наличия шаблона
        match ocv::ocv::is_template_on_image(&screenshot, template, threshold) {
            Ok(true) => return Ok(()),
            Ok(false) => (),
            Err(e) => return Err(format!("Ошибка поиска шаблона: {:?}", e)),
        }

        // Задержка между попытками
        thread::sleep(delay);

        // Вывод статуса попытки (полезно для отладки)
        println!("Попытка {}/{} - шаблон не найден", attempt, max_attempts);
    }

    // Если все попытки не удались, возвращаем ошибку
    Err(format!("Template not founded {}", max_attempts))
}

pub fn click_on_target(
    target: &Mat,
    threshold: f32,
    delay: Duration,
    active_zone_start: (i32,i32),
    active_zone_end: (u32,u32),
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {

    //сделать скриншот
    let screenshot = match utils::screenshot_area_to_mat(active_zone_start, active_zone_end) {
            Ok(img) => img,
            Err(e) => return Err(format!("Ошибка захвата скриншота: {:?}", e)),
        };


    //найти место
    let point = match ocv::ocv::find_template_in_image(&screenshot, target, threshold) {
        Ok(Some((x, y, w, h))) => average_point(x, y, w, h), // и вернуть кортеж
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
    mouse_set_position(point.0, point.1, screen_width, screen_height);

    //кликнуть
    mouse_left_click(delay);

    Ok(())
}

pub fn find_object() {}

pub fn extract_text() {}
