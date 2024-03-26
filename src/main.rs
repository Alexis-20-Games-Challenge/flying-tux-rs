//! This is a very copyright friendly game that is in
//! no way a knockoff of flappy bird.

use std::collections::VecDeque;

use glam::u32::UVec2;
use sdl2::{
    image::{InitFlag, LoadTexture, Sdl2ImageContext},
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
    Sdl, VideoSubsystem,
};

// Asset paths
const BG: &str = "./src/assets/bg.png";
const TUX: &str = "./src/assets/tux.png";
const BSOD: &str = "./src/assets/bsod.png";

// Dimensional Constants
const WIDTH: u32 = 512;
const HEIGHT: u32 = 1024;

/// A struct that contains the top left positions for each obstacle pair.
struct ObstaclePair {
    h1: UVec2,
    h2: UVec2,
}

///  Game state struct
struct State {
    ctx: Sdl,
    vid: VideoSubsystem,
    img: Sdl2ImageContext,
    canvas: Canvas<Window>,
    tex: TextureCreator<WindowContext>,
    player_pos: UVec2,
    player_vspeed: u32, // We will use this plus constant downwards acc gravity.
    obstacles: VecDeque<ObstaclePair>,
    points: u32,
    highscore: u32,
}

impl State {
    pub fn new() -> Self {
        let ctx = sdl2::init().unwrap();
        let vid = ctx.video().unwrap();
        let img = sdl2::image::init(InitFlag::PNG).unwrap();
        let win = vid
            .window("Flightless Bird Vs Windows 2024 Edition", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = win.into_canvas().software().build().unwrap();
        let tex = canvas.texture_creator();

        Self {
            ctx,
            vid,
            img,
            canvas,
            tex,
            player_pos: UVec2 {
                x: 128,
                y: HEIGHT / 2,
            },
            player_vspeed: 0,
            obstacles: VecDeque::with_capacity(3),
            points: 0,
            highscore: 0,
        }
    }
}

/// Returns a Result<(), String> because SDL has bad typing
fn main() -> Result<(), String> {
    println!("Welcome to flightless bird flying game!");

    let mut state = State::new();

    Ok(())
}
