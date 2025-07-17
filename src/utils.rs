use opencv::{core, imgproc, prelude::*};
use screenshots;

pub struct DisplayArea {
    s_x: i32,
    s_y: i32,
    e_x: i32,
    e_y: i32,
}

impl DisplayArea {

    pub fn from_relative(&self,basic_area: &DisplayArea) -> Self{
        
        let ((x,y),_) = basic_area.get_points();
        
        Self { 
            s_x: self.s_x+x, 
            s_y: self.s_y+y, 
            e_x: self.e_x+x, 
            e_y: self.e_y+y, 
        }
    }

    pub fn from_rectangle(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self {
            s_x: x,
            s_y: y,
            e_x: x + w as i32,
            e_y: y + h as i32,
        }
    }

    pub fn from_points(s_x: i32, s_y: i32, e_x: i32, e_y: i32) -> Self {
        Self {
            s_x: std::cmp::min(s_x, e_x),
            s_y: std::cmp::min(s_y, e_y),
            e_x: std::cmp::max(s_x, e_x),
            e_y: std::cmp::max(s_y, e_y),
        }
    }

    pub fn get_rectangle(&self) -> ((i32, i32), (u32, u32)) {
        (
            (
                std::cmp::min(self.s_x, self.e_x),
                std::cmp::min(self.s_y, self.e_y),
            ),
            (
                (self.s_x - self.e_x).abs() as u32,
                (self.s_y - self.e_y).abs() as u32,
            ),
        )
    }

    pub fn get_points(&self) -> ((i32, i32), (i32, i32)) {
        ((self.s_x, self.s_y), (self.e_x, self.e_y))
    }

    pub fn get_average_point(&self) -> (i32, i32) {
        ((self.s_x + self.e_x) / 2, (self.s_y + self.e_y) / 2)
    }
}

/// Captures a screen area and converts it to an OpenCV Mat in BGR format.
///
/// # Arguments
/// * `start_pos` - (x, y) coordinates of the capture area's top-left corner
/// * `end_pos` - (width, height) dimensions of the capture area from start_pos
///
/// # Returns
/// `Result<core::Mat, Box<dyn Error>>` containing:
/// - Success: BGR formatted OpenCV Mat (8UC3)
/// - Error: If capture or conversion fails
///
/// # Behavior
/// - Captures from primary screen only (first screen in list)
/// - Converts RGBA screenshot to OpenCV BGR format
/// - Preserves original image dimensions
/// - Uses unsafe blocks for Mat creation and data copying
///
/// # Safety
/// - Unsafe operations used for Mat creation and data copying
/// - Caller must ensure valid screen coordinates
pub fn screenshot_area_to_mat(area: &DisplayArea) -> Result<core::Mat, Box<dyn std::error::Error>> {
    let screens = screenshots::Screen::all()?;
    let screen = &screens[0];
    let ((x, y), (width, height)) = area.get_rectangle();
    let image = screen.capture_area(x, y, width, height)?;
    let (width, height) = (image.width() as i32, image.height() as i32);

    let mat = unsafe {
        let mut mat = core::Mat::new_rows_cols(height, width, core::CV_8UC4)?;
        core::Mat::from_slice(image.as_raw())?
            .reshape(4, height)?
            .copy_to(&mut mat)?;
        mat
    };

    let mut bgr_mat = core::Mat::default();
    imgproc::cvt_color(
        &mat,
        &mut bgr_mat,
        imgproc::COLOR_RGBA2BGR,
        0,
        core::AlgorithmHint::ALGO_HINT_DEFAULT,
    )?;

    Ok(bgr_mat)
}
