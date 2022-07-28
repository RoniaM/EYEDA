use opencv::{
    Result,
    prelude::*,
    objdetect,
    highgui,
    imgproc,
    core,
    types,
    videoio,
    highgui::wait_key,
};
use opencv::imgproc::hough_circles;
use opencv::imgproc::HoughModes::HOUGH_GRADIENT;


fn main()->Result<()>{

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

    let mut eye_cascade = objdetect::CascadeClassifier::new("righteye_2splits.xml")?;
    let mut img = Mat::default();

    loop{
        cam.read(&mut img)?;
        let mut gray = Mat::default();
        imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
        let mut eyes = types::VectorOfRect::new();
        eye_cascade.detect_multi_scale(
            &gray,
            &mut eyes,
            1.1,
            10,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size::new(10, 10),
            core::Size::new(0, 0)
        )?;
        println!("{:?}", eyes);
        if eyes.len() > 0{
            for eye in eyes.iter(){

                imgproc::rectangle(
                    &mut img,
                    eye,
                    core::Scalar::new(0f64, 255f64, 0f64, 0f64),
                    2,
                    imgproc::LINE_8,
                    0
                )?;

                let eye_gray = &gray[eye_y: eye.y + eye.height, eye_x: eye.x + eye.width];
                
                let circles = &mut ();

                let hough_circles = hough_circles(
                    eye_gray, 
                    circles,
                    1,
                    0.0,
                    0.0,
                    200.0,
                    1.0,
                    0,
                    0
                );
            }

        }
        highgui::imshow("img", &img)?;
        let pressed_key = wait_key(30)?;
        if pressed_key == 27 /* Escape key */ {
            break
        }
    }

    Ok(())
}