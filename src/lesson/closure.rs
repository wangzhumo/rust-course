//! 模拟工作的调用，演示闭包
use std::thread;
use std::time::Duration;



/**
 * 如果workhard函数更换，需要改很多地方，所以这里使用闭包优化
 */
pub fn workout(intensity:u32,random_number:u32){
    if intensity < 25 {
        println!(
            "今天活力满满，先工作 {} 个小时!",
            workhard(intensity)
        );
    } else if random_number == 3 {
        println!("昨天工作过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天工作，今天干干有氧，跑步 {} 分钟!",
            workhard(intensity)
        );
    }
}


/**
 * workout 的闭包版本，现在如果要替换，只需要改动`action`的调用即可
 */
pub fn workoutWithClosure(intensity:u32,random_number:u32){
    let action = || {
        workhard(intensity)
    };

    if intensity < 25 {
        println!(
            "今天活力满满，先工作 {} 个小时!",
            action()
        );
    } else if random_number == 3 {
        println!("昨天工作过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天工作，今天干干有氧，跑步 {} 分钟!",
            action()
        );
    }
}



fn workhard(intensity:u32) -> u32 {
    println!("workhard: {}",intensity);
    thread::sleep(Duration::from_secs(2));
    intensity
}