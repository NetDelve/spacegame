use std::collections::VecDeque;

struct RenderQueue<T> {
	temporary: bool, // remove items after rendering
	queue: VecDeque<T>,
}

impl<T> RenderQueue<T> {
	/// Creates an empty 'RenderQueue'.
	fn new(temporary: bool, capacity: usize) -> Self {
		Self {
			temporary,
			queue: VecDeque::with_capacity(capacity),
		}
	}
	/// Renders all objects in queue
	fn render(&mut self) -> Result<()> {
		for object in self.queue {
			object.render()?;
		}
		Ok(())
	}
}

struct RenderImage {
	
}
