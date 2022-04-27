//! Everything needed to render graphics in the Glass engine.

/// An active window with a canvas.
pub struct Window {
	/// The canvas that the window will render
	canvas: Texture2D,
	/// The width of the inside of the window in pixels
	width: u32,
	/// The height of the inside of the window in pixels
	height: u32,
}

/// A bitmap texture which can be rendered.
pub struct Texture2D {
	
}

impl Texture2D {
	/// Generates a new, blank Texture2D.
	pub fn new() -> Texture2D {
		Texture2D {}
	}
}