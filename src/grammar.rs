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
    fn new(left: &str, right: Vec<Sympol>) -> GrammarRule {
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

pub fn grammar() -> Grammar {
    Grammar {
        start_rule: "Sum".to_string(),
        items: vec![
            GrammarRule::new(
                "Sum",
                vec![
                    Sympol::NonTerminal("Sum".to_string()),
                    Sympol::Terminal(Pattern::Alternative(vec!["+".to_string(), "-".to_string()])),
                    Sympol::NonTerminal("Product".to_string()),
                ],
            ),
            GrammarRule::new("Sum", vec![Sympol::NonTerminal("Product".to_string())]),
            GrammarRule::new(
                "Product",
                vec![
                    Sympol::NonTerminal("Product".to_string()),
                    Sympol::Terminal(Pattern::Alternative(vec!["*".to_string(), "/".to_string()])),
                    Sympol::NonTerminal("Factor".to_string()),
                ],
            ),
            GrammarRule::new("Product", vec![Sympol::NonTerminal("Factor".to_string())]),
            GrammarRule::new(
                "Factor",
                vec![
                    Sympol::Terminal(Pattern::Exact("(".to_string())),
                    Sympol::NonTerminal("Sum".to_string()),
                    Sympol::Terminal(Pattern::Exact(")".to_string())),
                ],
            ),
            GrammarRule::new("Factor", vec![Sympol::NonTerminal("Number".to_string())]),
            GrammarRule::new(
                "Number",
                vec![
                    number_range('1', '9'),
                    Sympol::NonTerminal("Number".to_string()),
                ],
            ),
            GrammarRule::new("Number", vec![number_range('1', '9')]),
        ],
    }
}
