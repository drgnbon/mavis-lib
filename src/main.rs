use mavis_lib;
use opencv::{core, imgcodecs, imgproc, prelude::*};

fn main() {
    mavis_lib::action::input_text_simulated(
        "text",
        "path_to_template",
        (1920, 1080),
        ((9, 8), (7, 6)),
        0.6,
    );
}
