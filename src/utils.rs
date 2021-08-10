use anyhow::Context;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataBase<T>(Vec<T>, Vec<usize>);

impl<T> DataBase<T> {
    pub fn new() -> Self {
        Self(Vec::new(), Vec::new())
    }

    fn get_offset(&self, depth: usize) -> anyhow::Result<usize> {
        self.1
            .get(depth)
            .copied()
            .context("Use of undefined depth")
    }

    fn get_last_offset(&self) -> usize {
        self.1.last().copied().unwrap_or_default()
    }

    pub fn chunk(&mut self) -> usize {
        self.1.push(self.0.len());
        self.1.len() - 1
    }

    pub fn back(&mut self) {
        let offset = self.1.pop().unwrap_or_default();
        for _ in 0..(self.0.len() - offset) {
            self.0.pop();
        }
    }

    pub fn register<I: Into<T>>(&mut self, item: I) -> usize {
        self.0.push(item.into());
        self.0.len() - self.get_last_offset() - 1
    }

    pub fn get(&self, depth: usize, id: usize) -> anyhow::Result<&T> {
        let offset = self.get_offset(depth)?;
        self.0
            .get(offset + id)
            .context("Use of undefined id")
    }

    pub fn get_mut(&mut self, depth: usize, id: usize) -> anyhow::Result<&mut T> {
        let offset = self.get_offset(depth)?;
        self.0
            .get_mut(offset + id)
            .context("Use of undefined id")
    }
}
