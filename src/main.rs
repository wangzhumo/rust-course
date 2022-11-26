use log::{debug, error, info};
mod lesson;

pub use lesson::coord::*;

fn main() {
    env_logger::init();
    debug!("Hello, world!");
    error!("Hello, world!");
    info!("Hello, world!");
   
    lesson::ownership::main_ownership();
    let rect:Rect<i32> = Rect::new(0,0,800,400);
    error!("Rect center is x = {}  y = {}",rect.centerX(),rect.centerY());
}
