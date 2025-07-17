use anyhow::{Result, anyhow};
use opencv::{
    core::{Mat, Point, Size, min_max_loc},
    imgproc::{INTER_LINEAR, TM_CCOEFF_NORMED, match_template, resize},
    prelude::*,
};
use crate::utils::DisplayArea;

pub mod ocv {

    use super::*;


    pub fn find_target_in_image(
        recognition: f32,
        source: &Mat,
        target: &Mat,
    ) -> Result<DisplayArea, Box<dyn std::error::Error>> {

        if source.empty() {
            return Err("find_target_in_image: Source image is empty".into());
        }
        if target.empty() {
            return Err("find_target_in_image: target image is empty".into());
        }

        if target.rows() > source.rows() || target.cols() > source.cols() {
            return Err("find_target_in_image: target size exceeds source image dimensions".into());
        }

        if !(0.0..=1.0).contains(&recognition) {
            return Err(format!(
                "find_target_in_image: Threshold must be between 0 and 1, got {}",
                recognition
            ).into());
        }

        // Определение диапазона масштабов от 0.5 до 2.0 с шагом 0.1
        let scales: Vec<f64> = (5..=20).map(|i| i as f64 / 10.0).collect();

        let mut best_val = -1.0;
        let mut best_loc = Point::new(0, 0);
        let mut best_scale = 1.0;

        // Перебор масштабов
        for scale in scales {
            // Вычисление нового размера шаблона
            let new_width = (target.cols() as f64 * scale).round() as i32;
            let new_height = (target.rows() as f64 * scale).round() as i32;
            if new_width <= 0 || new_height <= 0 {
                continue;
            }

            // Изменение размера шаблона
            let mut resized_target = Mat::default();
            resize(
                &target,
                &mut resized_target,
                Size::new(new_width, new_height),
                0.0,
                0.0,
                INTER_LINEAR,
            )?;

            // Проверка, что шаблон не больше изображения
            if resized_target.cols() > source.cols() || resized_target.rows() > source.rows() {
                continue;
            }

            // Выполнение сопоставления шаблона
            let mut result = Mat::default();
            match_template(
                &source,
                &resized_target,
                &mut result,
                TM_CCOEFF_NORMED,
                &Mat::default(),
            )?;

            // Поиск максимального значения и его местоположения
            let mut min_val = 0.0;
            let mut max_val = 0.0;
            let mut min_loc = Point::new(0, 0);
            let mut max_loc = Point::new(0, 0);
            min_max_loc(
                &result,
                Some(&mut min_val),
                Some(&mut max_val),
                Some(&mut min_loc),
                Some(&mut max_loc),
                &Mat::default(),
            )?;

            // Обновление лучшего результата
            if max_val > best_val {
                best_val = max_val;
                best_loc = max_loc;
                best_scale = scale;
            }
        }

        // Проверка порога соответствия
        if best_val > recognition as f64 {

            Ok(DisplayArea::from_rectangle(
                best_loc.x, 
                best_loc.y, 
                ( (target.cols() as f64) * best_scale) as u32, 
                ( (target.rows() as f64) * best_scale) as u32
            ))

        } else {
            Err("Not founded".into())
        }
    }

    pub fn is_target_on_image(
        recognition: f32,
        source: &Mat,
        target: &Mat,
        area: &DisplayArea,
    )
    ->Result<bool,Box<dyn std::error::Error>> {

        match find_target_in_image(
            recognition,
            source, 
            target, 
        ){
            Ok(_)=>{
                return Ok(true);
            }
            Err(e)=>{
                println!("find_target_in_image: Error ocurated in is_target_on_image: {}",e);
                return Ok(false);
            }
        }




        Ok((false))
    }



}
