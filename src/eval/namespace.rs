use anyhow::Context;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameSpace<T>(Vec<Option<T>>, Vec<usize>);

impl<T> NameSpace<T> {
    pub fn new() -> Self {
        Self(Vec::new(), vec![0])
    }

    fn get_pos(&self, depth: usize, id: usize) -> anyhow::Result<usize> {
        let offset = self.1.get(depth).copied().context("Use of undefined depth")?;
        let pos = offset + id;
        let next_offset = self.1.get(depth + 1).copied().unwrap_or_else(|| self.0.len());
        if pos >= next_offset {
            anyhow::bail!("Illegal id.");
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

    pub fn get(&self, depth: usize, id: usize) -> anyhow::Result<&T> {
        let pos = self.get_pos(depth, id)?;
        self.0[pos]
            .as_ref()
            .context("The value is borrowed.")
    }

    pub fn borrow(&mut self, depth: usize, id: usize) -> anyhow::Result<T> {
        let pos = self.get_pos(depth, id)?;
        let res = std::mem::replace(&mut self.0[pos], None);
        res.context("The value is borrowed.")
    }

    pub fn ret(&mut self, depth: usize, id: usize, item: T) -> anyhow::Result<()> {
        let pos = self.get_pos(depth, id)?;
        if self.0[pos].is_some() {
            anyhow::bail!("The value is not borrowed.");
        }
        self.0[pos] = Some(item);
        Ok(())
    }
}
