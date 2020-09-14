pub struct Resolution {
    pub x: u16,
    pub y: u16
}

impl Resolution {
    pub fn new(x:u16, y:u16) -> Resolution { Resolution{ x, y } }
    pub fn area(&self) -> u32 {
        self.x as u32*self.y as u32
    }

    pub fn y(&self) -> u16 { self.y }
    pub fn x(&self) -> u16 { self.x }

    pub fn height(&self) -> u16 { self.y }
    pub fn width(&self) -> u16 { self.x }

}