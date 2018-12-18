use super::Entity;

pub struct EntityAllocator {
	free: Vec<Entity>,
	len: usize
}
impl EntityAllocator {
	pub fn new() -> EntityAllocator {
		EntityAllocator {
			free: Vec::new(),
			len: 0,
		}
	}
	pub fn allocate(&mut self) -> Option<Entity> {
		let result = self.free.pop();
		if result == None {
			self.len += 1;
		}
		result
	}
	pub fn deallocate(&mut self, id: Entity) {
		self.free.push(id);
	}
	pub fn is_free(&self, id: Entity) -> bool {
		self.free.contains(&id)
	}
	pub fn exists(&self, id: Entity) -> bool {
		!self.is_free(id) && id < self.len
	}
	pub fn len(&self) -> usize {
		self.len
	}
}
