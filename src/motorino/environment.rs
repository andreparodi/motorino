use super::cgmath::Vector3;

pub struct Light {
    pub position: Vector3<f32>,
    pub colour: Vector3<f32>
}

impl Default for Light {
    fn default() -> Light {
        Light {
            position: Vector3::new(0.0, 50.0, -20.0),
            colour: Vector3::new(1.0, 1.0, 1.0)
        }
    }
}

pub struct Fog {
    pub colour: Vector3<f32>,
    pub density: f32,
    pub gradient: f32
}

impl Default for Fog {
    fn default() -> Fog {
        Fog {
            colour: Vector3 {x: 1.0, y: 1.0, z:1.0},
            density: 0.007,
            gradient: 1.5
        }
    }
}
