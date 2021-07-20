#![allow(dead_code)]
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {

    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r,
            g,
            b,
        }
    }

    pub fn to_u32(&self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }

}


//
//----------------| operators |---------------------
//

impl ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self { // add components

        Color::new(
            add_u8(self.r, other.r),
            add_u8(self.g, other.g),
            add_u8(self.b, other.b),
        )
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, other: Color) { // add components
        *self = *self + other;
    }
}


impl ops::Sub for Color {  
    type Output = Self;
    fn sub(self, other: Self) -> Self { // subtract each component
        Color::new(
            sub_u8(self.r, other.r),
            sub_u8(self.g, other.g),
            sub_u8(self.b, other.b),
        )
    }
}

impl ops::SubAssign for Color {
    fn sub_assign(&mut self, other: Self) { // subtract each component
        *self = *self - other;
    }
}


impl ops::Mul<f64> for Color { // Scalar multiplication

    type Output = Color;

    fn mul(self, other: f64) -> Color {   
        Color::new( // multiply and clamp components, rounding to integers
            (self.r as f64 * other).clamp(0.0, 255.0) as u8, 
            (self.g as f64 * other).clamp(0.0, 255.0) as u8, 
            (self.b as f64 * other).clamp(0.0, 255.0) as u8, 
        )
    }
}

impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}

//
// -----------------| Miscellanious |--------------
//

pub fn rgb(r: u8, g: u8, b: u8) -> Color{ // shorthand for creating colors
    Color::new(r,g,b)
}


//
//-----------------| private utilities |-----------------------------
//

fn add_u8(x: u8, y: u8) -> u8 {
    match x.checked_add(y) {
        Some(i) => i,
        None => u8::MAX,
    }
}

fn sub_u8(x: u8, y: u8) -> u8 {
    match x.checked_sub(y) {
        Some(i) => i,
        None => u8::MIN,
    }
}

fn mul_u8(x: u8, y: u8) -> u8 {
    match x.checked_mul(y) {
        Some(i) => i,
        None => u8::MAX,
    }
}