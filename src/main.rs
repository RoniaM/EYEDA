use opencv::{
    highgui,
    imgproc,
    prelude::*,
    Result,
    videoio::CAP_ANY,
    videoio::VideoCapture,
    objdetect::CascadeClassifier,
    imgproc::rectangle,
    core::Scalar,
};


fn main() -> Result<()> {
    let GREEN =

    let eye_cascade = CascadeClassifier::new("righteye_2splits.xml");

    let mut cam = VideoCapture::new(0, CAP_ANY)?;

    let opened = VideoCapture::is_opened(&cam)?;
    if !opened { panic!("Unable to open default camera!"); }
    loop {
        let mut frame = Mat::default();

        cam.read(&mut frame)?;

        if frame.size()?.width > 0 {
            let mut gray = Mat::default();

            imgproc::cvt_color(
                &frame,
                &mut gray,
                imgproc::COLOR_BGR2GRAY,
                0,
            )?;

            let eyes = eye_cascade.unwrap().detect_multi_scale(&gray);

            for (eye_x, eye_y, eye_width, eye_height) in eyes {
                rectangle(image, (eye_x, eye_y) + (eye_x + eye_width, eye_y + eye_height), Scalar::from(0, 255, 0), 2, 1, 2)
            }

            highgui::imshow("frame", &gray)?;
        }
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }
    Ok(())
}