use opencv::{core, imgproc, prelude::*};
use screenshots;

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
pub fn screenshot_area_to_mat(
    start_pos: (i32, i32),
    end_pos: (u32, u32),
) -> Result<core::Mat, Box<dyn std::error::Error>> {
    let screens = screenshots::Screen::all()?;
    let screen = &screens[0];

    let image = screen.capture_area(start_pos.0, start_pos.1, end_pos.0, end_pos.1)?;
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

/// Calculates the geometric center point of a rectangle.
///
/// # Arguments
/// * `x` - Left coordinate of the rectangle
/// * `y` - Top coordinate of the rectangle
/// * `w` - Width of the rectangle (must be > 0)
/// * `h` - Height of the rectangle (must be > 0)
///
/// # Returns
/// `(u32, u32)` tuple representing the center coordinates:
/// - First element: Horizontal center (`x + w/2`)
/// - Second element: Vertical center (`y + h/2`)
pub fn average_point(x: u32, y: u32, w: u32, h: u32) -> (u32, u32) {
    (x + (w / 2), y + (h / 2))
}
