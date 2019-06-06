mod utils;
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pixel {
    Out = 0,
    In = 1,
}

struct RowCol {
    width: u32,
    height: u32,
    pub row: u32,
    pub col: u32,
}


impl RowCol {

    pub fn from_index(index: u32, width: &u32, height: &u32) -> RowCol {
        RowCol {
            width: *width,
            height: *height,
            row: index / height,
            col: index % width,
        }
    }

    pub fn to_complex(&self, r_range: &CoordRange, i_range: &CoordRange) -> Complex {
        let r_portion: f64 = (self.col as f64) / (self.width as f64);
        let i_portion: f64 = (self.row as f64) / (self.height as f64);
        let r_position = r_range.get_position_by_portion(r_portion);
        let i_position = i_range.get_position_by_portion(i_portion);
        Complex {
            r: r_position,
            i: i_position,
        }
    }
}

struct CoordRange {
    min: f64,
    max: f64,
}

impl CoordRange {
    pub fn new(min: f64, max: f64) -> CoordRange {
        assert!(min <= max);
        CoordRange { min: min, max: max }
    }

    pub fn get_position_by_portion(&self, portion: f64) -> f64 {
        portion * self.size() + self.min
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }
}

#[derive(Debug, PartialEq)]
struct Complex {
    r: f64,
    i: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.r, self.i)
    }
}

impl Complex {
    pub fn new(r: f64, i: f64) -> Complex {
        Complex { r: r, i: i }
    }

    // pub fn manhattan_dist(&self) -> f64 {
    //     self.r + self.i
    // }

    pub fn dist_squared(&self) -> f64 {
        self.r * self.r + self.i * self.i
    }

    pub fn square(&self) -> Complex {
        Complex {
            r: self.r * self.r - self.i * self.i,
            i: 2.0 * self.r * self.i,
        }
    }

    pub fn plus(&self, c: &Complex) -> Complex {
        Complex {
            r: self.r + c.r,
            i: self.i + c.i,
        }
    }

    pub fn in_mand(&self, iters: &u32) -> bool {
        let mut iter: u32 = 0;
        let mut z = Complex::new(0.0, 0.0);

        loop {
            z = z.square().plus(self);
            if z.dist_squared() > 4.0 {
                break false;
            }
            if iter > *iters {
                break true;
            }
            iter += 1;
        }

    }

}

#[wasm_bindgen]
pub struct Mand {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

#[wasm_bindgen]
impl Mand {

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }


    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Mand {
        let width = 800;
        let height = 800;
        let x_range = CoordRange::new(x_min, x_max);
        let y_range = CoordRange::new(y_min, y_max);
        let iters = 200;
        let pixels = (0..width * height)
            .map(|p| {
                if RowCol::from_index(p, &width, &height)
                    .to_complex(&x_range, &y_range)
                    .in_mand(&iters)
                {
                    Pixel::In
                } else {
                    Pixel::Out
                }
            })
            .collect();


        Mand {
            width,
            height,
            pixels,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rowcol() {
        let width: u32 = 512;
        let height: u32 = 512;

        let r1 = RowCol::from_index(0, &width, &height);
        assert!(r1.row == 0);
        assert!(r1.col == 0);

        let r2 = RowCol::from_index(153_800, &width, &height);
        assert!(r2.row == 300);
        assert!(r2.col == 200);

        let range_r = CoordRange::new(-2.0, 1.0);
        let range_i = CoordRange::new(-1.5, 1.5);

        let c = r1.to_complex(&range_r, &range_i);
        let ac = Complex::new(-2.0, -1.5);
        assert_eq!(c, ac);
    }

    #[test]
    fn test_coord() {
        let a = CoordRange::new(-2.0, 3.0);
        assert_eq!(a.size(), 5.0);
        assert_eq!(a.get_position_by_portion(0.0), -2.0);
        assert_eq!(a.get_position_by_portion(0.5), 0.5);
        assert_eq!(a.get_position_by_portion(1.0), 3.0);
    }

    #[test]
    fn test_mand(){
        let a = Complex::new(0.0,0.0);
        assert!(a.in_mand(&100)==true);
        let a = Complex::new(0.5,0.0);
        assert!(a.in_mand(&100)==false);
        let a = Complex::new(-1.5,1.0);
        assert!(a.in_mand(&100)==false);
        let a = Complex::new(-0.5,0.0);
        assert!(a.in_mand(&100)==true);
        let a = Complex::new(-0.2,-0.2);
        assert!(a.in_mand(&100)==true);
    }
}