use std::fmt::Display;

pub struct Rect<T:Display>{
    l: T,
    t: T,
    r: T,
    b: T
}

pub trait Printable{
    fn ToString(&self) -> String;
}

impl<T:Display> Printable for Rect<T> {
    fn ToString(&self) -> String {
        format!("Rect(left: {}, top: {}, right: {}, bottom: {})",self.l,self.t,self.r,self.b)
    }
}

impl<T:Display> Rect<T> {
    pub fn new(l: T, t: T,r: T, b: T) -> Rect<T> {
        Rect{l,t,r,b}
    }
}



impl Rect<i32> {

    pub fn CenterX(&self)->i32 {
        let width = self.r - self.l;
        width / 2
    }

    pub fn CenterY(&self)->i32 {
        let height = self.b - self.t;
        height / 2
    }

    // 把所有权交出去
    pub fn Center(&self)-> Point<i32>{
        let point = Point{ x:self.CenterX(), y:self.CenterY()};
        return point;
    }
}

pub struct Point<T:Display>{
    x: T,
    y: T
}




