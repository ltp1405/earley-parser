use crate::{grammar::GrammarRule, sympol::Sympol};
#[derive(Clone, PartialEq)]
pub struct EarleyItem {
    pub rule: GrammarRule,
    pub dot: usize,
    pub origin: usize,
}

impl std::fmt::Debug for EarleyItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut right = String::new();
        for s in self.rule.right.iter().take(self.dot) {
            right.push_str(&format!("{:?} ", s));
        }
        right.push_str("・");
        for s in self.rule.right.iter().skip(self.dot) {
            right.push_str(&format!("{:?} ", s));
        }
        write!(f, "{} -> {} ({})", self.rule.left, right, self.origin)
    }
}

impl EarleyItem {
    pub fn new(rule: GrammarRule, dot: usize, origin: usize) -> EarleyItem {
        EarleyItem {
            rule,
            dot,
            origin
        }
    }

    pub fn next_symbol(&self) -> Option<Sympol> {
        if self.dot < self.rule.right.len() {
            Some(self.rule.right[self.dot].clone())
        } else {
            None
        }
    }
}

