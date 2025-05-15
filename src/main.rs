use image::{RgbImage, Rgb};
use imageproc::drawing::draw_line_segment_mut;
use std::collections::HashMap;
mod lsystem;
use lsystem::*;

fn main() {
    let mut img = RgbImage::from_pixel(400, 400, Rgb([255, 255, 255]));

   
    let color = Rgb([1, 1, 1]);


  
    let mut segments = Vec::new();
    let mut bbox = BoundingBox::new();

    let lsys = LSystem {
        line_len: 10,
        max_iterations: 2,
        angle_change: 90.0,
        bouding_box: BoundingBox::new(),
        axiom: "F+F+F+F".to_string(),
        rules: HashMap::from([('F', "F+F-F-FF+F+F-F".to_string())])
    };

    lsys.draw(&lsys.axiom, Vec2(100.0, 100.0), 0.0, 0, &mut segments, &mut bbox);
    // for (start, end) in &segments {
    //     println!("Start: ({:.2}, {:.2}) -> End: ({:.2}, {:.2})", start.0, start.1, end.0, end.1);
    // }
    //let img_height = img.height() as f32;
     for (start, end) in &segments{
        let start_f = start.to_f32();
        let end_f = end.to_f32();

        let flipped_start = (start_f.0, img.height() as f32 - start_f.1);
       let flipped_end = (end_f.0, img.height() as f32 - end_f.1);
        draw_line_segment_mut(
            &mut img,
           flipped_start,
            flipped_end,
            color
        );
     }
    img.save("lsystem.png").unwrap();
}