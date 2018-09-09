extern crate ggez;
extern crate specs;

use ggez::conf;
/*use ggez::event;*/
use ggez::graphics;
use ggez::{Context, GameResult,};
use ggez::event::{self, Axis, Button, Keycode, Mod, MouseButton, MouseState};
use ggez::timer;
/*use ggez::filesystem;*/
use std::env;
use std::path;

struct MainState {
    text: graphics::Text,
    //canvas: graphics::Canvas,
    image: graphics::Image,
    point: graphics::Point2,
    frames: usize,
    //draw_with_canvas: bool,
    //spritebatch: graphics::spritebatch::SpriteBatch,
}

impl MainState {

    /* Creates a new main state from a given context - look into return type*/
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        /* Figure out what is going on here with the contexts */
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;
        let text = graphics::Text::new(ctx, "Hello World!", &font)?;
        //let canvas = graphics::Canvas::with_window_size(ctx)?;
       let image = graphics::Image::new(ctx,"/tile.png").unwrap();
       let point = graphics::Point2::new(50.0,50.0);
       //let batch = graphics::spritebatch::SpriteBatch::new(image);

        let s = MainState {
            text,
            //canvas,
            image, 
            point,
            /*draw_with_canvas : false,*/
            frames: 0,
            //spritebatch: batch,
        };
        Ok(s)   /* what does this do? */
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult <()> {
        let dest_point = graphics::Point2::new(10.0, 10.0);
        graphics::set_canvas(ctx, None);
        graphics::set_background_color(ctx, graphics::Color::from((64,64,0,0)));
        graphics::clear(ctx);
        graphics::draw_ex(
            ctx,
            &self.text,
            graphics::DrawParam{
                dest: dest_point,
                color: Some(graphics::Color::from((0,0,0,255))),
                ..Default::default()
            },
        )?;

        graphics::circle(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Point2::new(200.0,300.0),
            100.0,
            0.1,
        )?;
        graphics::draw_ex(
            ctx,
            &self.image,
            graphics::DrawParam{
                dest: self.point,
                ..Default::default()
                
            },
        )?;

        graphics::present(ctx); 
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }
/* switches canvas mode on keypress */
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool){
        println!(
            "Key Pressed: {:?}, modifer {:?}, repeat: {}",
            keycode, keymod, repeat
        );
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        //self.mouse_down = true;
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
        self.point = graphics::Point2::new(x as f32,y as f32);
    }
}
pub fn main() {
    println!("Hello World! Starting Main!");
    let c = conf::Conf { /*create a new Conf - we can later load a config*/
        window_setup: conf::WindowSetup {
            samples: conf::NumSamples::Two,
            ..Default::default()
        },
        ..Default::default()
    };
/*create a new context*/
   let ctx = &mut Context::load_from_conf("rusty-rpg","varneryo",c).unwrap();
   /*println!("Default path: {:#?}", ctx.filesystem);*/
   /*Adds resources folder in project dir to filesystem roots*/
   if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    ctx.filesystem.mount(&path,true);
    /*println!("Default path: {:#?}", ctx.filesystem);*/
   }

   let state = &mut MainState::new(ctx).unwrap();
   if let Err(e) = event::run(ctx,state){
    println!("Error encountered:{}",e);
   } else {
    println!("Game exited cleanly.");
   }
}
