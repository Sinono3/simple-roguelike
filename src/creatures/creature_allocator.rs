use super::CreatureId;

pub struct CreatureAllocator {
	free: Vec<CreatureId>,
	len: usize
}
impl CreatureAllocator {
	pub fn new() -> CreatureAllocator {
		CreatureAllocator {
			free: Vec::new(),
			len: 0,
		}
	}
	pub fn allocate(&mut self) -> Option<CreatureId> {
		let result = self.free.pop();
		if result == None {
			self.len += 1;
		}
		result
	}
	pub fn deallocate(&mut self, id: CreatureId) {
		self.free.push(id);
	}
	pub fn is_free(&self, id: CreatureId) -> bool {
		self.free.contains(&id)
	}
	pub fn exists(&self, id: CreatureId) -> bool {
		!self.is_free(id) && id < self.len
	}
	pub fn len(&self) -> usize {
		self.len
	}
}
