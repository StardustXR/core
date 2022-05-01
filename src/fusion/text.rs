struct Text {
	text: &str,
	font_path: &str,
	character_height: f32,
	bounds: Vec2,
	bounds_align: Vec2,
	enabled: bool,
	node_id: i32,
	path: &str,
	color: Color,
}

impl Colored for Text {
	fn get_color(&self) -> Color {
	return self.color;
	}
	fn set_color(&self, color: Color) {
	self.color = color;
	}
}
impl Boundable for Text {
	fn get_bounds(&self) -> Vec2 {
	return self.bounds;
	}
	fn get_bounds_align(&self) -> Vec2 {
	return self.bounds_align;
	}
	fn set_bounds(&self, bounds: Vec2) {
	self.bounds = bounds;
	}
	fn set_bounds_align(&self, bounds: Vec2) {
	self.bounds_align = bounds;
	}
}
impl Textable for Text {
	fn get_character_height(&self) -> f32 {
	return self.character_height;
	}
	fn set_character_height(&self, character_height: f32) {
	self.character_height = character_height;
	}
	fn get_font_path(&self) -> &str {
	return self.font_path;
	}
	fn set_font_path(&self, font_path: &str) {
	self.font_path = font_path;
	}
	fn get_text(&self) -> &str {
	return self.text;
	}
	fn set_text(&self, text: &str) {
	self.text = text;
	}
}
impl Spatial for Text {
	fn rotatable(&self) -> bool {
	return true;
	}
	fn scalable(&self) -> bool {
	return true;
	}
	fn translatable(&self) -> bool {
	return true;
	}
	fn zoneable(&self) -> bool {
	return true;
	}
}
impl Node for Text {
	fn create(&self) {
	//Create the text
	}
	fn destroy(&self) {
	//Destroy the text
	}
}
