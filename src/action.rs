use crate::input::{keyboard, mouse};
use crate::utils;
use crate::vision::{ocv, tsrt};
use anyhow::Ok;
use opencv::{core, imgcodecs, imgproc, prelude::*};

pub fn input_text_simulated(
    text: &str,
    path_to_template: &str,
    resolution: (u32, u32),
    perm_area: ((i32, i32), (u32, u32)),
    treshold: f32,
) {
}

pub fn wait_for_image() {}

pub fn click_on_target() {}

pub fn find_object() {}

pub fn extract_text() {}
