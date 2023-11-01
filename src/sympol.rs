#[derive(Clone, PartialEq)]
pub enum Sympol {
    NonTerminal(String),
    Terminal(Pattern),
}

#[derive(Clone, PartialEq)]
pub enum Pattern {
    Alternative(Vec<String>),
    Exact(String),
    Range(char, char),
}

impl Pattern {
    pub fn matches(&self, s: &str) -> bool {
        match self {
            Pattern::Alternative(v) => v.contains(&s.to_string()),
            Pattern::Exact(e) => e == s,
            Pattern::Range(start, end) => {
                let n = s.parse::<char>().unwrap();
                n >= *start && n <= *end
            }
        }
    }
}

impl std::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::Alternative(v) => {
                let mut result = String::new();
                for s in v {
                    result.push_str(&format!("{}", s));
                }
                write!(f, "[{}]", result)
            }
            Pattern::Exact(s) => write!(f, "{}", s),
            Pattern::Range(start, end) => write!(f, "[{}-{}]", start, end),
        }
    }
}

impl std::fmt::Debug for Sympol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sympol::NonTerminal(s) => write!(f, "{}", s),
            Sympol::Terminal(p) => write!(f, "{:?}", p),
        }
    }
}

pub fn number_range(start: char, end: char) -> Sympol {
    Sympol::Terminal(Pattern::Range(start, end))
}
