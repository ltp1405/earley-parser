use crate::{chart::EarleyChart, item::EarleyItem, state::EarleyState};

mod chart;
mod grammar;
mod item;
mod state;
mod sympol;

pub use grammar::{Grammar, GrammarRule};
pub use sympol::{number_range, Pattern, Sympol};

fn predict(grammar: &Grammar, chart: &mut EarleyChart, current: usize, rule_name: &str) {
    for rule in grammar.items.iter() {
        if rule.left == rule_name {
            chart[current].add_item(EarleyItem::new(rule.clone(), 0, current))
        }
    }
}

fn scan(
    item: &EarleyItem,
    chart: &mut EarleyChart,
    sympol: &Sympol,
    current: usize,
    input: &[&str],
) {
    match sympol {
        Sympol::Terminal(s) => {
            if current >= input.len() {
                return;
            }
            if s.matches(input[current]) {
                if chart.get(current + 1).is_none() {
                    chart.add_state(EarleyState::new());
                }
                chart[current + 1].add_item(EarleyItem {
                    dot: item.dot + 1,
                    ..item.clone()
                })
            }
        }
        _ => unreachable!(),
    }
}

fn complete(chart: &mut EarleyChart, current_state: usize, current_item: usize) {
    let item = chart[current_state][current_item].clone();
    let sympol = item.rule.left;
    for i in 0..chart[item.origin].len() {
        if chart[item.origin][i].next_symbol() == Some(Sympol::NonTerminal(sympol.clone())) {
            let current_dot = chart[item.origin][i].dot;
            let chosen_item = chart[item.origin][i].clone();
            chart[current_state].add_item(EarleyItem {
                dot: current_dot + 1,
                ..chosen_item
            })
        }
    }
}

pub fn build(grammar: &Grammar, input: &[&str]) -> EarleyChart {
    let g = grammar;
    let mut chart = EarleyChart::new();
    let mut state = EarleyState::new();
    state.add_item_unchecked(EarleyItem::new(g.items[0].clone(), 0, 0));
    for rule in g.items.iter() {
        if rule.left == g.start_rule {
            state.add_item(EarleyItem::new(rule.clone(), 0, 0));
        }
    }
    chart.add_state(state);

    let mut i = 0;
    while i < chart.len() {
        let mut j = 0;
        while j < chart[i].len() {
            let item = chart[i][j].clone();
            let next_symbol = item.next_symbol();
            match next_symbol {
                Some(Sympol::NonTerminal(s)) => predict(&g, &mut chart, i, &s),
                Some(sympol) => scan(&item, &mut chart, &sympol, i, &input),
                None => {
                    complete(&mut chart, i, j);
                }
            }
            j += 1;
        }
        i += 1;
    }
    chart
}
