use std::collections::HashMap;

struct Object {
	typeid: usize,
}

struct Layer {
	objects: Vec<Vec<Object>>,
}

impl Layer {
	pub fn get(&self, x: usize, y: usize) -> Result<&Object, String> {
		if x < self.objects.len() && y < self.objects[x].len() { // not sure if this is done in order, might cause issues
			Ok(&self.objects[x][y])
		} else {
			Err("object index out of bounds".to_string())
		}
	}
	pub fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut Object, String> {
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
}
