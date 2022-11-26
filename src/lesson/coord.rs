pub struct Rect<T>{
    l: T,
    t: T,
    r: T,
    b: T
}

impl<T> Rect<T> {
    pub fn new(l: T, t: T,r: T, b: T) -> Rect<T> {
        Rect{l,t,r,b}
    }
}


pub struct Point<T>{
    x: T,
    y: T
}




impl Rect<i32> {

    pub fn centerX(&self)->i32 {
        let width = self.r - self.l;
        width / 2
    }

    pub fn centerY(&self)->i32 {
        let height = self.b - self.t;
        height / 2
    }

    // 把所有权交出去
    pub fn center(&self)-> Point<i32>{
        let point = Point{ x:self.centerX(), y:self.centerY()};
        return point;
    }
}