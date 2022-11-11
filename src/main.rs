fn main() {
    let r1: Rectangle = Rectangle {
        width: 22,
        height: 25,
    };
    dbg!(r1.area()); // as u1.uname has String (which does not implement Copy trait, u1 will become UNUSABLE. Other scalar types implement copy trait)
    dbg!(Rectangle::square(20)); 
}
// return the index of the first word ending

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//houses methods for Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
// contains associated functions as they do not have the first param as &self
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

