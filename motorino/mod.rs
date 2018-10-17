#![allow(non_upper_case_globals)]

extern crate cgmath;
extern crate image;
extern crate gl;
extern crate glfw;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate rand;
extern crate specs;
extern crate tobj;

use self::camera::Camera;
use self::camera::CameraController;
use self::cgmath::Vector3;
use self::components::{GridPosition, RawModel, PlayerFlag, SimpleTexture, Transform, Velocity};
use self::debugui::{DebugUi, DebugUiBuilder, RenderSettingsController};
use self::display::Display;
use self::terrain::Terrain;
use self::environment::{Fog, Light};
use self::glfw::Action;
use self::input::CursorPosition;
use self::input::KeyEvents;
use self::input::MouseEvents;
use self::input::MouseState;
use self::input::WindowEventHandler;
use self::models::Loader;
use self::player::PlayerController;
use self::rand::prelude::*;
use self::renderers::{ClearScreenRenderer, EntityRenderer, RenderSettings, TerrainRenderer};
use self::resources::ResourceLoader;
use self::specs::prelude::*;
use self::specs::World;
use self::glfw::WindowEvent;
use std::path::Path;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use self::components::TerrainPhysics;
use self::components::TerrainTexturePack;
use self::debugui::DebugInfo;
use self::debugui::DebugInfoResetter;

#[macro_use]
pub mod macros;

pub mod renderers;
pub mod shaders;
pub mod resources;
pub mod models;
pub mod terrain;
pub mod camera;
pub mod environment;
pub mod components;
pub mod debugui;
pub mod player;
pub mod input;
pub mod display;
pub mod ringbuffer;

#[derive(Clone, Copy, Debug, Default)]
pub struct UpdateDeltaTime(f32);

#[derive(Clone, Copy, Debug)]
pub struct WindowSize {
    width: u32,
    height: u32
}

impl Default for WindowSize {
    fn default() -> Self {
        WindowSize {width: Motorino::WIDTH, height: Motorino::HEIGHT}
    }
}

pub struct Motorino {
    resource_loader: Rc<ResourceLoader>
}

impl Motorino {

    pub const WIDTH: u32 = 800;
    pub const HEIGHT: u32 = 800;

    pub fn new() -> Motorino {
        Motorino {resource_loader: Rc::new(ResourceLoader::from_relative_path(Path::new("res-output")).unwrap()) }
    }

    fn create_display(&self) -> (Display, Receiver<(f64, WindowEvent)>) {
        Display::create(Motorino::WIDTH, Motorino::HEIGHT)
    }

    fn create_world(&self, mut loader: &mut Loader, debug_ui: DebugUi) -> World {
        let mut world = World::new();
        world.register::<Velocity>();
        world.register::<Transform>();
        world.register::<TerrainTexturePack>();
        world.register::<TerrainPhysics>();
        world.register::<GridPosition>();
        world.register::<RawModel>();
        world.register::<SimpleTexture>();
        world.register::<PlayerFlag>();

        world.add_resource(UpdateDeltaTime::default());
        world.add_resource(DebugInfo::default());
        world.add_resource(Light::default());
        world.add_resource(Fog::default());
        world.add_resource(Camera::default());
        world.add_resource(WindowSize::default());
        world.add_resource(CursorPosition::default());
        world.add_resource(KeyEvents::default());
        world.add_resource(MouseEvents::default());
        world.add_resource(MouseState::default());
        world.add_resource(RenderSettings::default());
        world.add_resource(debug_ui);

        let mut rng = thread_rng();
        Motorino::create_terrain(&mut loader, &self.resource_loader, &mut world);

        Motorino::create_multiple_entities(&mut world, &mut rng, &mut loader, "models/tree1.obj", "textures/tree1.jpg", 50);
        Motorino::create_multiple_entities(&mut world, &mut rng, &mut loader, "models/tree2.obj", "textures/tree2.jpg", 100);
        Motorino::create_multiple_entities(&mut world, &mut rng, &mut loader, "models/tree3.obj", "textures/tree3.jpg", 300);

        Motorino::create_player(&mut loader, &mut world);

        world
    }

    fn create_player(loader: &mut Loader, world: &mut World) {
        world.create_entity()
            .with(Transform { position: Vector3 { x: 0.0, y: 0.0, z: 0.0 }, ..Transform::default() })
            .with(loader.load_from_obj("models/lego-man.obj"))
            .with(loader.load_simple_texture("textures/lego-man.jpg", 0.0, 20.0).unwrap())
            .with(Velocity::default())
            .with(PlayerFlag {})
            .build();
    }

    fn create_terrain(mut loader: &mut Loader, resource_loader: &ResourceLoader, world: &mut World) {
        let height_map = resource_loader.load_image("textures/heightmap.png").unwrap();
        let terrain_texture_pack = TerrainTexturePack::new(loader, "textures/grass.jpg", "textures/mud.jpg", "textures/grass-flowers.jpg", "textures/path.jpg", "textures/blend-map.jpg");
        let terrain = Terrain::new(&mut loader, 0, 0, &height_map);
        let terrain2 = Terrain::new(&mut loader, 1, 0, &height_map);
        world.create_entity()
            .with(GridPosition { x: 0, z: 0 })
            .with(terrain_texture_pack)
            .with(terrain.raw_model)
            .with(TerrainPhysics { heights:terrain.heights, x: terrain.x, z: terrain.z})
            .build();
        world.create_entity()
            .with(GridPosition { x: 1, z: 0 })
            .with(terrain_texture_pack)
            .with(terrain2.raw_model)
            .with(TerrainPhysics { heights:terrain2.heights, x: terrain2.x, z: terrain2.z})
            .build();
    }

    fn create_multiple_entities(world: &mut World, rng: &mut ThreadRng, loader: &mut Loader, model: &str, texture: &str, count: i32) {
        let model = loader.load_from_obj(&model);
        let tex = loader.load_simple_texture(&texture, 0.0, 20.0).unwrap();

        for _i in 0..count {
            let x: f32 = rng.gen::<f32>() * Terrain::SIZE as f32;
            let z: f32 = rng.gen::<f32>() * Terrain::SIZE as f32;

            let height = {
                let terrain_physics: ReadStorage<TerrainPhysics> = world.read_storage();
                let grid_position: ReadStorage<GridPosition> = world.read_storage();

                Terrain::get_height_for_position(&grid_position, &terrain_physics, x, z)
            };

            world.create_entity()
                .with(Transform{position: Vector3{ x: x, y: height, z: z}, scale: Vector3{x:2.5, y:2.5, z:2.5},..Transform::default()})
                .with(model.clone())
                .with( tex.clone())
                .build();
        }
    }


    fn create_dispatcher(&self, event_receiver: Receiver<(f64, WindowEvent)>) -> Dispatcher {
        let dispatcher = DispatcherBuilder::new()
            .with(CameraController, "camera-controller", &[])
            .with(PlayerController::default(), "player-controller", &[])
            .with(RenderSettingsController, "render-settings-controller", &[])
            .with_thread_local(DebugInfoResetter)
            .with_thread_local(ClearScreenRenderer)
            .with_thread_local(TerrainRenderer::new(&self.resource_loader))
            .with_thread_local(EntityRenderer::new(&self.resource_loader))
            .with_thread_local(DebugUiBuilder)
            .with_thread_local(WindowEventHandler::new(event_receiver))
            .build();
        dispatcher
    }

    pub fn run(&mut self) {
        let (mut display, event_receiver) = self.create_display();
        let mut loader = Loader::new(self.resource_loader.clone());

        let debug_ui = DebugUi::new(|s| display.window.get_proc_address(s) as _);
        let mut world = self.create_world(&mut loader, debug_ui);

        let mut update_dispatcher = self.create_dispatcher(event_receiver);

        #[allow(unused_assignments)]
        let mut delta_time: f32 = 0.0;
        let mut last_frame: f32 = 0.0;

        while !display.should_close {
            let current_frame = display.get_time() as f32;
            delta_time = current_frame - last_frame;
            last_frame = current_frame;

//            let () = {
//                let mut frame_times = world.write_resource::<RingBuffer<f32>>();
//                frame_times.push(delta_time);
//            };

            let () = {
                let mut delta = world.write_resource::<UpdateDeltaTime>();
                *delta = UpdateDeltaTime(delta_time);
            };


            let () = {
                let mut mouse_state = world.write_resource::<MouseState>();
                mouse_state.button1 = display.mouse_action(glfw::MouseButtonLeft) == Action::Press;
                mouse_state.button2 = display.mouse_action(glfw::MouseButtonMiddle) == Action::Press;
                mouse_state.button3 = display.mouse_action(glfw::MouseButtonRight) == Action::Press;
            };

            let () = {
                let mut cursor_position = world.write_resource::<CursorPosition>();
                let (x, y) = display.get_cursor_position();
                cursor_position.x = x as f32;
                cursor_position.y = y as f32;
            };

            update_dispatcher.dispatch(&mut world.res);
            world.maintain();

            display.swap_buffers();
            display.poll_events();
        }
    }
}


