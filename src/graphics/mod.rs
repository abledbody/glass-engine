//! Everything needed to render graphics in the Glass engine.

use std::fmt::Display;

use anyhow::Result;

use crate::data_types::Rect;

mod shading;
pub mod text;

/// An active window with a canvas.
pub struct Window {
	/// The title of the window.
	title: String,
	/// The canvas that the window will render.
	pub canvas: Texture2D<BitmapRGBA8>,
	/// The width of the inside of the window in pixels.
	width: u32,
	/// The height of the inside of the window in pixels.
	height: u32,
}

impl Window {
	/// Creates a new window on the desktop. Automatically closed when the window is dropped.
	pub fn new(title: &str, width: u32, height: u32) -> Window {
		Window {
			title: title.to_owned(),
			canvas: Texture2D::new(width, height),
			width,
			height,
		}
	}
	
	/// Resizes the window, along with its canvas.
	pub fn resize(&mut self, width: u32, height: u32) {
		self.width = width;
		self.height = height;
		self.canvas = self.canvas.resize(width, height);
	}
}

/// A bitmap texture which can be rendered.
pub struct Texture2D<T: Bitmap> {
	/// The bitmap data.
	bitmap: T,
	/// The width of the texture in pixels.
	width: u32,
	/// The height of the texture in pixels.
	height: u32,
}

impl<T: Bitmap> Texture2D<T> {
	/// Generates a new, blank Texture2D.
	pub fn new(width: u32, height: u32) -> Texture2D<T> {
		let bitmap = T::new(width as usize, height as usize);
		
		Texture2D {
			bitmap,
			width,
			height,
		}
	}
	
	/// Creates a new texture with the provided dimensions and tries to fit the existing texture into it.
	pub fn resize(&self, width: u32, height: u32) -> Texture2D<T> {
		let bitmap = T::new(width as usize, height as usize);
		
		Texture2D {
			bitmap,
			width,
			height,
		}
	}
	
	/// Blits one texture onto another.
	pub fn blit_to(&self, other: &mut Texture2D<T>, source_rect: Rect<isize>, target_rect: Rect<isize>) -> Result<()> {
		// Step 1:
		// 	Account for spill in source rects.
		// Step 2:
		//	Account for spill in target rects.
		// Step 3:
		//	Draw the dang thing.
		
		let source_spill_x = (-source_rect.x).max(0);
		let source_spill_y = (-source_rect.y).max(0);
		let source_spill_x2 = (source_rect.x2() - self.width as isize).max(0);
		let source_spill_y2 = (source_rect.y2() - self.height as isize).max(0);
		
		let clipped_source_rect = Rect {
			x: source_rect.x.max(0),
			y: source_rect.y.max(0),
			width: source_rect.width - source_spill_x - source_spill_x2,
			height: source_rect.height - source_spill_y - source_spill_y2,
		};
		
		let target_spill_x = (-target_rect.x).max(0);
		let target_spill_y = (-target_rect.y).max(0);
		let target_spill_x2 = (target_rect.x2() - other.width as isize).max(0);
		let target_spill_y2 = (target_rect.y2() - other.height as isize).max(0);
		
		let clipped_target_rect = Rect {
			x: target_rect.x.max(0),
			y: target_rect.y.max(0),
			width: target_rect.width - target_spill_x - target_spill_x2,
			height: target_rect.height - target_spill_y - target_spill_y2,
		};
		
		let drawn_width = clipped_source_rect.width.min(clipped_target_rect.width);
		let drawn_height = clipped_source_rect.height.min(clipped_target_rect.height);
		
		for y in 0..drawn_height {
			for x in 0..drawn_width {
				if let Some(source_texel) = self.bitmap.read((x + clipped_source_rect.x) as usize, (y + clipped_source_rect.y) as usize) {
					other.bitmap.write((x + clipped_target_rect.x) as usize, (y + clipped_target_rect.y) as usize, source_texel);
				}
			}
		}
		Ok(())
	}
}

impl<T: Bitmap> Display for Texture2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Texture2D of width {}, height {}, with format {}", self.width, self.height, self.bitmap)
    }
}

pub trait Bitmap: Display {
	type Inner;
	
	fn new(width: usize, height: usize) -> Self;
	fn pitch(&self) -> usize;
	fn read(&self, x: usize, y: usize) -> Option<Self::Inner>;
	fn write(&mut self, x: usize, y: usize, value: Self::Inner);
}

#[inline]
fn get_bitmap_index(pitch: usize, x: usize, y: usize) -> usize {
	x % pitch + y / pitch
}

/// An unsigned 8-bit value for each color with an alpha channel, written into an unsigned 32-bit number.
/// In binary, expressed from MSB to LSB as: ```Aa Bb Gg Rr```
#[allow(non_camel_case_types)]
pub struct BitmapRGBA8 {
	data: Vec<u32>,
	pitch: usize,
}

impl Bitmap for BitmapRGBA8 {
	type Inner = u32;
	
	fn new(width: usize, height: usize) -> Self {
		BitmapRGBA8 {
			data: vec![0xFF000000; width * height],
			pitch: width,
		}
	}
	
	fn pitch(&self) -> usize {
		self.pitch
	}
	
	fn read(&self, x: usize, y: usize) -> Option<u32> {
		self.data.get(get_bitmap_index(self.pitch, x, y)).copied()
	}

	fn write(&mut self, x: usize, y: usize, value: u32) {
		if let Some(x) = self.data.get_mut(get_bitmap_index(self.pitch, x, y)) {
			*x = value
		};
    }
}

impl Display for BitmapRGBA8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RGBA-8")
    }
}

#[derive(Debug)]
struct DroppedTextureError {
	
}

impl std::error::Error for DroppedTextureError {}

impl Display for DroppedTextureError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "A texture that once existed was dropped.")
	}
}