#[derive(Clone, Debug, PartialEq)]
pub enum Sympol {
    NonTerminal(String),
    Alt(Vec<Sympol>),
    Terminal(String),
}

fn number_range(start: u8, end: u8) -> Sympol {
    let mut result = Vec::new();
    for i in start..end {
        result.push(Sympol::Terminal(format!("{}", i as char)));
    }
    Sympol::Alt(result)
}

#[derive(Clone, Debug, PartialEq)]
pub struct GrammarRule {
    left: String,
    right: Vec<Sympol>,
}

impl GrammarRule {
    fn new(left: &str, right: Vec<Sympol>) -> GrammarRule {
        GrammarRule {
            left: left.to_string(),
            right,
        }
    }
}

pub struct Grammar {
    start_rule: String,
    items: Vec<GrammarRule>,
}

pub fn grammar() -> Grammar {
    Grammar {
        start_rule: "Sum".to_string(),
        items: vec![
            GrammarRule::new(
                "Sum",
                vec![
                    Sympol::NonTerminal("Sum".to_string()),
                    Sympol::Alt(vec![
                        Sympol::Terminal("+".to_string()),
                        Sympol::Terminal("-".to_string()),
                    ]),
                    Sympol::NonTerminal("Product".to_string()),
                ],
            ),
            GrammarRule::new("Sum", vec![Sympol::NonTerminal("Product".to_string())]),
            GrammarRule::new(
                "Product",
                vec![
                    Sympol::NonTerminal("Product".to_string()),
                    Sympol::Alt(vec![
                        Sympol::Terminal("*".to_string()),
                        Sympol::Terminal("/".to_string()),
                    ]),
                    Sympol::NonTerminal("Factor".to_string()),
                ],
            ),
            GrammarRule::new("Product", vec![Sympol::NonTerminal("Factor".to_string())]),
            GrammarRule::new(
                "Factor",
                vec![
                    Sympol::Terminal("(".to_string()),
                    Sympol::NonTerminal("Sum".to_string()),
                    Sympol::Terminal(")".to_string()),
                ],
            ),
            GrammarRule::new("Factor", vec![Sympol::NonTerminal("Number".to_string())]),
            GrammarRule::new(
                "Number",
                vec![
                    number_range(0, 9),
                    Sympol::NonTerminal("Number".to_string()),
                ],
            ),
            GrammarRule::new("Number", vec![number_range(0, 9)]),
        ],
    }
}

#[derive(Clone, Debug, PartialEq)]
struct EarleyItem {
    rule: GrammarRule,
    dot: usize,
    start: usize,
    end: usize,
}

impl EarleyItem {
    fn new(rule: GrammarRule, dot: usize, start: usize, end: usize) -> EarleyItem {
        EarleyItem {
            rule,
            dot,
            start,
            end,
        }
    }

    fn next_symbol(&self) -> Option<Sympol> {
        if self.dot < self.rule.right.len() {
            Some(self.rule.right[self.dot].clone())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct EarleyState {
    items: Vec<EarleyItem>,
}

impl EarleyState {
    fn new() -> EarleyState {
        EarleyState { items: Vec::new() }
    }

    fn add_item_unchecked(&mut self, item: EarleyItem) {
        self.items.push(item);
    }

    fn add_item(&mut self, item: EarleyItem) {
        if !self.items.contains(&item) {
            self.items.push(item);
        }
    }
}

#[derive(Clone, Debug)]
struct EarleyChart {
    states: Vec<EarleyState>,
}

fn predict(state: &mut EarleyState) {}

fn scan() {}

fn complete() {}

pub fn build() {
    let g = grammar();
    let mut chart = EarleyChart { states: Vec::new() };
    let mut state = EarleyState::new();
    state
        .items
        .push(EarleyItem::new(g.items[0].clone(), 0, 0, 0));
    for rule in g.items.iter() {
        if rule.left == g.start_rule {
            state.items.push(EarleyItem::new(rule.clone(), 0, 0, 0));
        }
    }
    chart.states.push(state);
    println!("{:#?}", chart);

    let mut i = 0;
    while i < chart.states.len() {
        let state = &chart.states[i];
        let mut j = 0;
        while j < state.items.len() {
            let item = &state.items[j].clone();
            let next_symbol = item.next_symbol();
            match next_symbol {
                Some(Sympol::NonTerminal(s)) => {
                    predict()
                }
                Some(Sympol::Terminal(s)) => {
                    scan()
                }
                Some(Sympol::Alt(v)) => {
                    scan()
                }
                None => {
                    complete()
                }
            }
            j += 1;
        }
        i += 1;
    }
}
