use std::collections::HashMap;
use render::TileMapRenderObject;

struct Object {
	typeid: usize,
}

struct Layer {
	objects: Vec<Vec<Object>>,
}

impl Layer {
	pub fn get_object(&self, x: usize, y: usize) -> Result<&Object, String> {
		if x < self.objects.len() && y < self.objects[x].len() { // not sure if this is done in order, might cause issues
			Ok(&self.objects[x][y])
		} else {
			Err("object index out of bounds".to_string())
		}
	}
	pub fn get_object_mut(&mut self, x: usize, y: usize) -> Result<&mut Object, String> {
		if x < self.objects.len() && y < self.objects[x].len() { // not sure if this is done in order, might cause issues
			Ok(&mut self.objects[x][y])
		} else {
			Err("object index out of bounds".to_string())
		}
	}
}

struct Area {
	layers: Vec<Layer>,
	x: i32,
	y: i32,
	size_x: i32,
	size_y: i32,
}

impl Area {
	pub fn new(x: i32, y: i32, size_x: i32, size_y: i32) -> Self {
		Self {
			layers: Vec::new(),
			x,
			y,
			size_x,
			size_y,
		}
	}
	pub fn with_capacity(capacity: usize, x: i32, y: i32, size_x: i32, size_y: i32) -> Self {
		Self {
			layers: Vec::with_capacity(capacity),
			x,
			y,
			size_x,
			size_y,
		}
	}
	pub fn get(&self, layer: usize) -> Result<&Layer, String> {
		if x < self.layers.len() { // not sure if this is done in order, might cause issues
			Ok(&self.layers[layer])
		} else {
			Err("layer index out of bounds".to_string())
		}
	}
	pub fn get_mut(&mut self, layer: usize) -> Result<&mut Layer, String> {
		if x < self.layers.len() { // not sure if this is done in order, might cause issues
			Ok(&mut self.layers[layer])
		} else {
			Err("layer index out of bounds".to_string())
		}
	}
}

struct World {
	areas: HashMap<String, Area>,
	x: i32,
	y: i32,
	size_x: i32,
	size_y: i32,
}

impl World {
	pub fn new(x: i32, y: i32, size_x: i32, size_y: i32) -> Self {
		Self {
			areas: HashMap::new(),
			x,
			y,
			size_x,
			size_y,
		}
	}
	pub fn with_capacity(capacity: usize, x: i32, y: i32, size_x: i32, size_y: i32) -> Self {
		Self {
			areas: HashMap::with_capacity(capacity),
			x,
			y,
			size_x,
			size_y,
		}
	}
}

struct Universe {
	worlds: HashMap<String, World>,
}

impl Universe {
	pub fn new() -> Self {
		Self {
			worlds: HashMap::new(),
		}
	}
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			worlds: HashMap::with_capacity(capacity),
		}
	}
	/// Load everything in viewport from disk
	pub fn load(&mut Self, x, y, size_x, size_y) {
		// Filesystem stuff here
	}
	/// Unload everything not in viewport
	pub fn discard(&mut Self, x, y, size_x, size_y) {
		Self.worlds.retain(|world| {
			if bounding_box(x, y, size_x, size_y, world.x, world.y, world.size_x, world.size_y) {
				world.areas.retain(|area| {
					bounding_box(x, y, size_x, size_y, area.x, area.y, area.size_x, area.size_y)
				})
				true
			} else {
				false
			}
		})
	}
	/// Get everything in area, return drawables
	pub fn get_drawables(&Self, x, y, size_x, size_y) -> Vec<> {
		for world in Self.worlds {
			for area in world.areas {
				for layer in area.layers {
					for row in layer.objects {
						for object in row {
							// wew lad
						}
					}
				}
			}
		}
	}
	/// Update loaded objects and return drawables
	pub fn update(&mut Self, x, y, size_x, size_y) -> Vec<> {
		discard(Self, x, y, size_x, size_y);
		load(Self, x, y, size_x, size_y);
		get_drawables(Self, x, y, size_x, size_y)
	}
}

fn bounding_box (x1, y1, size_x1, size_y1, x2, y2, size_x2, size_y2) -> bool {
	x1 >= x2 && x1+size_x1 <= x2+size_x2 && y1 >= y2 && y1+size_y1 <= y2+size_y2
}
