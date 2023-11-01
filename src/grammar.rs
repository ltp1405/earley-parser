use crate::sympol::{number_range, Pattern, Sympol};

#[derive(Clone, PartialEq)]
pub struct GrammarRule {
    pub left: String,
    pub right: Vec<Sympol>,
}

impl std::fmt::Debug for GrammarRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut right = String::new();
        for s in self.right.iter() {
            right.push_str(&format!("{:?} ", s));
        }
        write!(f, "{} -> {}", self.left, right)
    }
}

impl GrammarRule {
    pub fn new(left: &str, right: Vec<Sympol>) -> GrammarRule {
        GrammarRule {
            left: left.to_string(),
            right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grammar {
    pub start_rule: String,
    pub items: Vec<GrammarRule>,
}

