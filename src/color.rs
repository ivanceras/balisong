//color.rs
use std::fmt;

pub struct Color{
	pub r:u8,
	pub g:u8,
	pub b:u8,
	pub a:u8
}

impl Color {

	pub fn new(r:u8, g:u8, b:u8, a:u8)->Color{
		Color{r:r, g:g, b:b, a:a}
	}
	
	pub fn white()->Color{
		Color{r:255, g:255, b:255, a:255}
	}
	pub fn black()->Color{
		Color{r:0, g:0, b:0,a:255}
	}
	pub fn purple()->Color{
	    Color{r:203, g:0, b:245, a:255}
	}
	pub fn red()->Color{
	    Color{r:255, g:0, b:0, a:255}
	}
	pub fn green()->Color{
	    Color{r:0, g:255, b:0, a:255}
	}
	pub fn blue()->Color{
	    Color{r:0, g:0, b:155, a:255}
	}
}

impl Clone for Color {
    fn clone(&self) -> Color { Color{r:self.r, g:self.g, b:self.b, a:self.a} }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}
