pub struct Game {
    state: bool
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: true
        }
    }
    pub fn get_state(&self) -> bool {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(Game::new().get_state());
    }
}
