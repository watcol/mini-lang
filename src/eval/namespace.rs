use crate::{MiniError, MiniResult};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameSpace<T>(Vec<Option<T>>, Vec<usize>);

impl<T> NameSpace<T> {
    pub fn new() -> Self {
        Self(Vec::new(), vec![0])
    }

    fn get_pos(&self, depth: usize, id: usize) -> MiniResult<usize> {
        let offset = self.1.get(depth).copied().ok_or("Use of undefined depth")?;
        let pos = offset + id;
        let next_offset = self
            .1
            .get(depth + 1)
            .copied()
            .unwrap_or_else(|| self.0.len());
        if pos >= next_offset {
            return Err(MiniError::from("Illegal id"));
        }
        Ok(pos)
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

    pub fn register(&mut self, item: T) -> usize {
        self.0.push(Some(item));
        self.0.len() - self.get_last_offset() - 1
    }

    pub fn get(&self, depth: usize, id: usize) -> MiniResult<&T> {
        let pos = self.get_pos(depth, id)?;
        Ok(self.0[pos].as_ref().ok_or("The value is borrowed.")?)
    }

    pub fn borrow(&mut self, depth: usize, id: usize) -> MiniResult<T> {
        let pos = self.get_pos(depth, id)?;
        let res = std::mem::replace(&mut self.0[pos], None);
        Ok(res.ok_or("The value is borrowed.")?)
    }

    pub fn ret(&mut self, depth: usize, id: usize, item: T) -> MiniResult<()> {
        let pos = self.get_pos(depth, id)?;
        if self.0[pos].is_some() {
            return Err(MiniError::from("Illegal id"));
        }
        self.0[pos] = Some(item);
        Ok(())
    }
}
