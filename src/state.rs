use std::ops::{Deref, DerefMut};

use crate::item::EarleyItem;

#[derive(Clone, Debug)]
pub struct EarleyState {
    items: Vec<EarleyItem>,
}

impl EarleyState {
    pub fn new() -> EarleyState {
        EarleyState { items: Vec::new() }
    }

    pub fn items(&self) -> &Vec<EarleyItem> {
        &self.items
    }

    pub fn add_item_unchecked(&mut self, item: EarleyItem) {
        self.items.push(item);
    }

    pub fn add_item(&mut self, item: EarleyItem) {
        if !self.items.contains(&item) {
            self.add_item_unchecked(item);
        }
    }
}

impl Deref for EarleyState {
    type Target = Vec<EarleyItem>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for EarleyState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}
