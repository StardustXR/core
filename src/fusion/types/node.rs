trait Node {
	fn create(&self) {

	}
	fn destroy(&self);
	fn get_node_id(&self) -> i32;
	fn set_enabled(&self);
	fn set_disabled(&self);
	fn get_path(&self) -> &str;
	fn set_path(&self, path: &str);
}
trait Spatial {
	fn translatable(&self) -> bool;
	fn rotatable(&self) -> bool;
	fn scalable(&self) -> bool;
	fn zoneable(&self) -> bool;
}
trait Colored {
	fn get_color(&self) -> Color;
	fn set_color(&self, color: Color);
}
trait Boundable {
	fn get_bounds(&self) -> Vec2;
	fn set_bounds(&self, bounds: Vec2);
	fn get_bounds_align(&self) -> Vec2;
	fn set_bounds_align(&self, bounds: Vec2);
}
trait Textable {
	fn get_text(&self) -> &str;
	fn set_text(&self, text: &str);
	fn get_font_path(&self) -> &str;
	fn set_font_path(&self, font_path: &str);
	fn get_character_height(&self) -> f32;
	fn set_character_height(&self, character_height: f32);
}
