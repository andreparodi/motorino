use super::specs::System;
use super::specs::ReadStorage;
use super::components::PlayerFlag;
use super::specs::WriteStorage;
use super::components::Velocity;
use super::components::Transform;
use super::UpdateDeltaTime;
use super::specs::Read;
use super::input::KeyEvents;
use super::glfw::{Action, Key};
use std::f64::consts::PI;
use super::terrain::Terrain;
use super::components::GridPosition;
use super::components::TerrainPhysics;

#[derive(Default)]
pub struct PlayerController {
    pub is_in_air: bool
}

impl PlayerController {
    const RUN_SPEED: f32 = 50.0;
    const TURN_SPEED: f32 = 120.0;
    const GRAVITY: f32 = -80.0;
    const JUMP_POWER: f32 = 25.0;
}

impl<'a> System<'a> for PlayerController {
    type SystemData = (ReadStorage<'a, PlayerFlag>,
                       WriteStorage<'a, Velocity>,
                       WriteStorage<'a, Transform>,
                       ReadStorage<'a, GridPosition>,
                       ReadStorage<'a, TerrainPhysics>,
                       Read<'a, KeyEvents>,
                       Read<'a, UpdateDeltaTime>);

    fn run(&mut self, (_player_flag, mut velocity, mut transform, grid_position, terrain_physics, key_events, dt): <Self as System<'a>>::SystemData) {
        let dt = dt.0;
        use super::specs::Join;
        for (mut velocity, mut transform) in (&mut velocity, &mut transform).join() {
            for key_event in key_events.iter() {
                let key = &key_event.source;
                let action = &key_event.action;
                if key == &Key::W {
                    if action == &Action::Press || action == &Action::Repeat {
                        velocity.run_velocity = PlayerController::RUN_SPEED;
                     } else if action == &Action::Release {
                        velocity.run_velocity = 0.0;
                    }
                }
                if key == &Key::S {
                    if action == &Action::Press || action == &Action::Repeat {
                        velocity.run_velocity = -PlayerController::RUN_SPEED;
                    } else if action == &Action::Release {
                        velocity.run_velocity = 0.0;
                    }
                }
                if key == &Key::A {
                    if action == &Action::Press || action == &Action::Repeat {
                        velocity.turn_velocity = PlayerController::TURN_SPEED;
                    } else if action == &Action::Release {
                        velocity.turn_velocity = 0.0;
                    }
                }
                if key == &Key::D {
                    if action == &Action::Press || action == &Action::Repeat {
                        velocity.turn_velocity = -PlayerController::TURN_SPEED;
                    } else if action == &Action::Release {
                        velocity.turn_velocity = 0.0;
                    }
                }
                if key == &Key::Space {
                    if action == &Action::Press || action == &Action::Repeat {
                        if !self.is_in_air {
                            velocity.upwards_velocity =  PlayerController::JUMP_POWER;
                            self.is_in_air = true;
                        }
                    }
                }
            }

            transform.rotation.y = transform.rotation.y + velocity.turn_velocity * dt;
            let rotation_rad = PI as f32 / 180.0 * transform.rotation.y;
            let dz = velocity.run_velocity * dt * rotation_rad.sin();
            let dx = velocity.run_velocity * dt * rotation_rad.cos();
            transform.position.x = transform.position.x + dz;
            transform.position.z = transform.position.z + dx;

            let height = Terrain::get_height_for_position(&grid_position, &terrain_physics, transform.position.x, transform.position.z);

            velocity.upwards_velocity = velocity.upwards_velocity + PlayerController::GRAVITY * dt;
            transform.position.y = transform.position.y + velocity.upwards_velocity * dt;

            if transform.position.y < height {
                velocity.upwards_velocity = 0.0;
                self.is_in_air = false;
                transform.position.y = height;
            }
        }

    }
}

impl PlayerController {

}
