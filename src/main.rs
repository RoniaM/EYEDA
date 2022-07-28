use opencv::{
    highgui,
    imgproc,
    prelude::*,
    Result,
    videoio::CAP_ANY,
    videoio::VideoCapture,
};

fn main() -> Result<()> {
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

            highgui::imshow("frame", &gray)?;
        }
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }
    Ok(())
}