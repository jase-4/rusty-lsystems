use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct Vec2(pub f64, pub f64);

#[derive(Copy, Clone, Debug)]
pub struct Vec2f(pub f32, pub f32);

impl Vec2 {
    pub fn add(&self, other: Vec2) -> Vec2 {
        Vec2(self.0 + other.0, self.1 + other.1)
    }

    pub fn from_angle(length: f64, angle_deg: f64) -> Vec2 {
        let angle_rad = angle_deg.to_radians();
        Vec2(length * angle_rad.cos(), length * angle_rad.sin())
    }

    pub fn to_f32(&self) -> Vec2f {
        Vec2f(self.0 as f32, self.1 as f32)
    }
}

#[derive(Debug)]
pub struct BoundingBox {
    pub min: Vec2,
    pub max: Vec2,
}

impl BoundingBox {
    pub fn new() -> Self {
        Self {
            min: Vec2(f64::INFINITY, f64::INFINITY),
            max: Vec2(f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }

    pub fn update(&mut self, point: Vec2) {
        self.min.0 = self.min.0.min(point.0);
        self.min.1 = self.min.1.min(point.1);
        self.max.0 = self.max.0.max(point.0);
        self.max.1 = self.max.1.max(point.1);
    }

    pub fn center(&self) -> Vec2 {
        Vec2(
            (self.min.0 + self.max.0) / 2.0,
            (self.min.1 + self.max.1) / 2.0,
        )
    }

    pub fn size(&self) -> Vec2 {
        Vec2(
            (self.max.0 - self.min.0).abs(),
            (self.max.1 - self.min.1).abs(),
        )
    }
}

pub struct LSystem {
    pub line_len: i32,
    pub max_iterations: i32,
    pub angle_change: f64,
    pub bouding_box: BoundingBox,
    pub axiom: String,
    pub rules: HashMap<char, String>,
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

                            bbox.update(pos);
                            bbox.update(next_pos);
                            
                            // segments.push((pos, next_pos));
                            if let Some(last_segment) = segments.last() {
                                self.draw(
                                    value,
                                    last_segment.1,
                                    angle,
                                    cur_iterations + 1,
                                    segments,
                                    bbox,
                                );
                            } else {
                                self.draw(
                                    value,
                                    pos,
                                    angle,
                                    cur_iterations + 1,
                                    segments,
                                    bbox,
                                );
                            }

                            // pos = next_pos;
                           
                        }
                    } else {
                        let dir = Vec2::from_angle(self.line_len as f64, angle);
                        let next_pos = pos.add(dir);

                        bbox.update(pos);
                        bbox.update(next_pos);
                        segments.push((pos, next_pos));

                        pos = next_pos;
                    }
                }
                '+' => angle += self.angle_change,
                '-' => angle -= self.angle_change,
                _ => {}
            }
        }
    }
}

pub struct LSystemBuilder {
    line_len: i32,
    max_iterations: i32,
    angle_change: f64,
    axiom: String,
    rules: HashMap<char, String>,
}

impl LSystemBuilder {
    pub fn new(axiom: String) -> Self {
        Self {
            line_len: 1,
            max_iterations: 1,
            angle_change: 0.0,
            axiom,
            rules: HashMap::new(),
        }
    }

    pub fn line_len(mut self, len: i32) -> Self {
        self.line_len = len;
        self
    }

    pub fn max_iterations(mut self, iters: i32) -> Self {
        self.max_iterations = iters;
        self
    }
    pub fn axiom(mut self, axiom: &str) -> Self {
        self.axiom = axiom.to_string();
        self
    }

    pub fn angle_change(mut self, angle: f64) -> Self {
        self.angle_change = angle;
        self
    }

    pub fn rule(mut self, key: char, value: &str) -> Self {
        self.rules.insert(key, value.to_string());
        self
    }

    pub fn build(self) -> LSystem {
        LSystem {
            line_len: self.line_len,
            max_iterations: self.max_iterations,
            angle_change: self.angle_change,
            axiom: self.axiom,
            rules: self.rules,
            bouding_box: BoundingBox::new(),
        }
    }
}
