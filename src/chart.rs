use std::ops::{Deref, DerefMut};

use crate::state::EarleyState;
#[derive(Clone, Debug)]
pub struct EarleyChart {
    states: Vec<EarleyState>,
}

impl EarleyChart {
    pub fn new() -> Self {
        EarleyChart { states: Vec::new() }
    }

    pub fn add_state(&mut self, state: EarleyState) {
        self.states.push(state);
    }

    pub fn states(&self) -> &Vec<EarleyState> {
        &self.states
    }
}

impl Deref for EarleyChart {
    type Target = Vec<EarleyState>;

    fn deref(&self) -> &Self::Target {
        &self.states
    }
}

impl DerefMut for EarleyChart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.states
    }
}
