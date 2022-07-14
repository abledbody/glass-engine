const CHAR_PITCH: usize = 16;

use std::{fmt::Display, rc::Weak};

use anyhow::Result;

use crate::data_types::Rect;

use super::{Texture2D, BitmapRGBA8, DroppedTextureError};

pub struct MonospaceFont {
	texture: Weak<Texture2D<BitmapRGBA8>>,
	char_width: u16,
	char_height: u16,
}

impl MonospaceFont {
	pub fn new(texture_ref: Weak<Texture2D<BitmapRGBA8>>, char_width: u16, char_height: u16) -> Result<Self> {
		let texture = unwrap_or_err!(texture_ref.upgrade(), Err(DroppedTextureError{}.into()) );
		
		let font_required_width = char_width as usize * CHAR_PITCH;
		if font_required_width > texture.width as usize {
			return Err(FontTextureMisfitError {
				contents: font_required_width,
				bounds: texture.width as usize
			}.into());
		}
		
		let font = MonospaceFont {
			texture: texture_ref,
			char_width,
			char_height,
		};
		
		Ok(font)
	}
	
	

	pub fn write_char(&self, char: char, texture: &mut Texture2D<BitmapRGBA8>, x: isize, y: isize) -> Result<()> {
		let font_texture = unwrap_or_err!(self.texture.upgrade(), Err(DroppedTextureError{}.into()));
		
		let char_rect = self.get_char_texture_position(char);
		let target_rect = Rect::<isize>::new(x, y, self.char_width as isize, self.char_height as isize);
		
		let char_rect = Rect {
			x: char_rect.x as isize,
			y: char_rect.y as isize,
			width: char_rect.width as isize,
			height: char_rect.height as isize
		};
		
		font_texture.blit_to(texture, char_rect, target_rect)?;
		Ok(())
	}
	
	pub fn write_string(&self, string: &str, texture: &mut Texture2D<BitmapRGBA8>, x: isize, y: isize) -> Result<()> {
		for (i, char) in string.chars().enumerate() {
			self.write_char(char, texture, x + i as isize * self.char_width as isize, y)?;
		}
		Ok(())
	}

	fn get_char_texture_position(&self, char: char) -> Rect<u32> {
		let x = (char as usize % 16) * self.char_width as usize;
		let y = char as usize / 16;
		
		Rect::new(x as u32, y as u32, self.char_width as u32, self.char_height as u32)
	}
}

#[derive(Debug, Clone)]
pub struct FontTextureMisfitError {
	contents: usize,
	bounds: usize,
}

impl std::error::Error for FontTextureMisfitError {}

impl Display for FontTextureMisfitError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "font requires {} texels horizontally, but texture is only {} texels wide", self.contents, self.bounds)
	}
}