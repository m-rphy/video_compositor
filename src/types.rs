use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Rules {
    pub size: [u32; 2],
    pub rects: Vec<RectRule>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RectRule {
    pub src: [u32; 4],   // x, y, width, height
    pub dest: [u32; 2],  // x, y
    pub alpha: f32,
    pub z: i32,
}

