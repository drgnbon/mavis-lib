use opencv::{core, imgproc, prelude::*};
use screenshots;

pub fn screenshot_to_mat() -> Result<core::Mat, Box<dyn std::error::Error>> {
    let screens = screenshots::Screen::all()?;
    let screen = &screens[0];

    let image = screen.capture()?;
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
