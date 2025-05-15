use std::collections::HashMap;
#[derive(Copy, Clone, Debug)]
pub struct Vec2(pub f64, pub f64);
pub struct Vec2f(pub f32, pub f32);

impl Vec2 {
    fn add(&self, other: Vec2) -> Vec2 {
        Vec2(self.0 + other.0, self.1 + other.1)
    }

    fn from_angle(length: f64, angle_deg: f64) -> Vec2 {
        let angle_rad = angle_deg.to_radians();
        Vec2(length * angle_rad.cos(), length * angle_rad.sin())
    }
    pub fn to_f32(&self) -> Vec2f {
        Vec2f(self.0 as f32, self.1 as f32)
    }
}

#[derive(Debug)]
pub struct BoundingBox {
    min: Vec2,
    max: Vec2,
}

 impl BoundingBox {
    pub fn new() -> Self {
        BoundingBox {
            min: Vec2(f64::INFINITY, f64::INFINITY),
            max: Vec2(f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }

    fn update(&mut self, point: Vec2) {
        self.min.0 = self.min.0.min(point.0);
        self.min.1 = self.min.1.min(point.1);
        self.max.0 = self.max.0.max(point.0);
        self.max.1 = self.max.1.max(point.1);
    }

    fn center(&self) -> Vec2 {
        Vec2(
            (self.min.0 + self.max.0) / 2.0,
            (self.min.1 + self.max.1) / 2.0,
        )
    }

    fn size(&self) -> Vec2 {
        Vec2(
            (self.max.0 - self.min.0).abs(),
            (self.max.1 - self.min.1).abs(),
        )
    }
}

pub struct LSystem{
    pub line_len: i32,
    pub  max_iterations: i32,
    pub angle_change: f64,
    pub bouding_box: BoundingBox,
    pub axiom: String,
    pub rules: HashMap<char,String>
}


impl LSystem {
    pub fn draw(
        &self,
        instructions: &str,
        start_pos: Vec2,
        start_angle: f64,
        cur_iterations: i32,
        segments: &mut Vec<(Vec2, Vec2)>,
        bbox: &mut BoundingBox,
    ) {
        let mut pos = start_pos;
        let mut angle = start_angle;

        for ch in instructions.chars() {
            match ch {
                'F' => {
                    if cur_iterations < self.max_iterations {
                        if let Some(value) = self.rules.get(&ch) {
                            let dir = Vec2::from_angle(self.line_len as f64, angle);
                            let next_pos = pos.add(dir);
    
                            // store result
                            bbox.update(pos);
                            bbox.update(next_pos);
                            segments.push((pos, next_pos));
    
                            pos = next_pos;
                            self.draw(
                                value,
                                pos,
                                angle,
                                cur_iterations + 1,
                                segments,
                                bbox,
                            );
                        }
                    } else {
                        let dir = Vec2::from_angle(self.line_len as f64, angle);
                        let next_pos = pos.add(dir);

                        // store result
                        bbox.update(pos);
                        bbox.update(next_pos);
                        segments.push((pos, next_pos));

                        pos = next_pos;
                    }
                }
                '+' => {
                    angle += self.angle_change;
                }
                '-' => {
                    angle -= self.angle_change;
                }
                // You can add '[' and ']' for recursion state saving here
                _ => {}
            }
        }
    }
}