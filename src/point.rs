#[derive(Copy, Clone, PartialEq, Debug)]

pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn distance(&self, another_point: &Point) -> f64 {
        ((self.x - another_point.x).powf(2.) + (self.y - another_point.y).powf(2.)).sqrt()
    }

}