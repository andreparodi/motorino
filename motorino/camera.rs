
use super::cgmath::{Point3, Vector3, Matrix4, vec3};
use super::cgmath::prelude::*;
use std::fmt;
use super::specs::System;
use super::specs::ReadStorage;
use super::specs::Write;
use super::components::PlayerFlag;
use super::components::Transform;


// Default camera values
const YAW: f32 = -90.0;
const PITCH: f32 = -10.0;

type Point3f = Point3<f32>;
type Vector3f = Vector3<f32>;
type Matrix4f = Matrix4<f32>;

pub struct Camera {
    pub follow_player: bool,
    // Camera Attributes
    pub position: Point3f,
    pub front: Vector3f,
    pub up: Vector3f,
    pub right: Vector3f,
    pub world_up: Vector3f,
    // Euler Angles
    pub yaw: f32,
    pub pitch: f32
}

impl Default for Camera {
    fn default() -> Camera {
        let mut camera = Camera {
            follow_player: true,
            position: Point3::new(4.0, 50.0, 120.0),
            front: vec3(0.0, 0.0, -1.0),
            up: Vector3::zero(), // initialized later
            right: Vector3::zero(), // initialized later
            world_up: Vector3::unit_y(),
            yaw: YAW,
            pitch: PITCH
        };
        camera.update_camera_vectors();
        camera
    }
}

impl Camera {

    pub fn get_view_matrix(&self) -> Matrix4f {
        Matrix4::look_at(self.position, self.position + self.front, self.up)
    }

    fn update_camera_vectors(&mut self) {
        // Calculate the new Front vector
        let front = Vector3 {
            x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            y: self.pitch.to_radians().sin(),
            z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        };
        self.front = front.normalize();
        // Also re-calculate the Right and Up vector
        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
        self.update_camera_vectors();
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
        self.update_camera_vectors();
    }
}

impl fmt::Display for Camera {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Camera: [{:.1}, {:.1}, {:.1}] [{:.1}, {:.1}]", self.position.x, self.position.y, self.position.z, self.yaw, self.pitch)
    }
}

pub struct CameraController;

impl<'a> System<'a> for CameraController {
    type SystemData = (Write<'a, Camera>,
                       ReadStorage<'a, Transform>,
                       ReadStorage<'a, PlayerFlag>);

    fn run(&mut self, (mut camera, transform, player_flag): <Self as System<'a>>::SystemData) {
        if camera.follow_player {
            use super::specs::Join;
            for (transform, _player_flag) in (&transform, &player_flag).join() {
                camera.position.x = transform.position.x - (transform.rotation.y.to_radians().sin() * 20.0);
                camera.position.y = transform.position.y + 10.0;
                camera.position.z = transform.position.z - (transform.rotation.y.to_radians().cos() * 20.0);
                camera.yaw = -transform.rotation.y + 90.0;
                camera.update_camera_vectors();
            }
        }
    }
}
