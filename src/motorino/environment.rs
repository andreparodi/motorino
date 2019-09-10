use super::cgmath::Vector3;

pub struct Light {
    pub position: Vector3<f32>,
    pub colour: Vector3<f32>
}

impl Default for Light {
    fn default() -> Light {
        Light {
            position: Vector3::new(0.0, 600.0, -20.0),
            colour: Vector3::new(1.0, 1.0, 1.0)
        }
    }
}

pub struct Fog {
    pub day_colour: Vector3<f32>,
    pub night_colour: Vector3<f32>,
    pub density: f32,
    pub gradient: f32
}

impl Default for Fog {
    fn default() -> Fog {
        Fog {
            day_colour: Vector3 {x: 0.78, y: 0.86, z:0.86},
            night_colour: Vector3 {x: 0.275, y: 0.275, z:0.275},
            density: 0.007,
            gradient: 1.5
        }
    }
}
