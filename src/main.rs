use log::{debug, error, info};
mod lesson;

pub use lesson::coord::*;

fn main() {
    // 初始化env_logger
    env_logger::init();

    //lesson::ownership::main_ownership();
    //let rect:Rect<i32> = Rect::new(0,0,800,400);
    //error!("Rect center is {}",rect.ToString());

    let x = String::from("wangzhumo");
    let y = String::from("wzm");
    let result;
    {
        
        result =  lesson::liferecycle::longest(x.as_str(), y.as_str());
    }
    println!("result = {result}");
}
