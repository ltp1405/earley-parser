use earley_parser::{build, number_range, Grammar, GrammarRule, Pattern, Sympol};

fn grammar1() -> Grammar {
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

fn grammar2() -> Grammar {
    let d = Pattern::Alternative(vec![
        "the".to_string(),
    ]);
    let n = Pattern::Alternative(vec![
        "student".to_string(),
        "cat".to_string(),
        "dog".to_string(),
        "mouse".to_string(),
        "table".to_string(),
        "class".to_string(),
    ]);
    let a = Pattern::Alternative(vec![
        "big".to_string(),
        "small".to_string(),
        "young".to_string(),
    ]);
    let v = Pattern::Alternative(vec![
        "chased".to_string(),
        "saw".to_string(),
        "ate".to_string(),
        "sat".to_string(),
    ]);
    let p = Pattern::Alternative(vec![
        "in".to_string(),
        "on".to_string(),
        "under".to_string(),
        "with".to_string(),
    ]);
    let grammar = Grammar {
        start_rule: "S".to_string(),
        items: vec![
            GrammarRule::new(
                "S",
                vec![
                    Sympol::NonTerminal("NP".to_string()),
                    Sympol::NonTerminal("VP".to_string()),
                ],
            ),
            GrammarRule::new(
                "S",
                vec![
                    Sympol::NonTerminal("NP".to_string()),
                    Sympol::NonTerminal("VP".to_string()),
                    Sympol::NonTerminal("PP".to_string()),
                ],
            ),
            GrammarRule::new(
                "NP",
                vec![
                    Sympol::Terminal(d.clone()),
                    Sympol::NonTerminal("NP3".to_string()),
                ],
            ),
            GrammarRule::new(
                "NP3",
                vec![
                    Sympol::Terminal(a),
                    Sympol::NonTerminal("NP3".to_string()),
                ],
            ),
            GrammarRule::new(
                "NP3",
                vec![Sympol::Terminal(n.clone())],
            ),
            GrammarRule::new(
                "NP3",
                vec![
                    Sympol::Terminal(n.clone()),
                    Sympol::NonTerminal("PP".to_string()),
                ],
            ),
            GrammarRule::new(
                "PP",
                vec![
                    Sympol::Terminal(p),
                    Sympol::NonTerminal("NP2".to_string()),
                ],
            ),
            GrammarRule::new(
                "NP2",
                vec![
                    Sympol::Terminal(d),
                    Sympol::NonTerminal("NP3".to_string()),
                ],
            ),
            GrammarRule::new(
                "VP",
                vec![Sympol::Terminal(v)],
            ),
        ],
    };
    grammar
}

fn main() {
    let input = "2+(3*4-5)"
        .split("")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    let grammar = grammar1();
    let chart = build(&grammar, &input);
    println!("{:#?}", chart);
    println!("========================================");
    let input = "the young student sat in the class"
        .split_whitespace()
        .collect::<Vec<&str>>();
    println!("{:?}", input);
    let grammar = grammar2();
    let chart = build(&grammar, &input);
    println!("{:#?}", chart);
}
