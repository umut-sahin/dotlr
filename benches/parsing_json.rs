use {
    criterion::{
        BatchSize,
        Criterion,
        Throughput,
        criterion_group,
    },
    dotlr::{
        Grammar,
        Parser,
    },
};

fn benchmark_parsing_json(criterion: &mut Criterion) {
    let grammars = [
        ("Simple", include_str!("../assets/grammars/correct/json.lr")),
        ("Optimized", include_str!("../assets/grammars/correct/json.optimized.lr")),
    ];

    let mut group = criterion.benchmark_group("Parsing JSON");

    let input = include_str!("../assets/data/large.json");
    group.throughput(Throughput::Bytes(input.len() as u64));

    for (name, definition) in grammars {
        let grammar = Grammar::parse(definition).unwrap();
        for lalr in [false, true] {
            let parser = if lalr {
                Parser::lalr(grammar.clone()).unwrap()
            } else {
                Parser::lr(grammar.clone()).unwrap()
            };
            let tokens = parser.tokenize(input).unwrap();
            group.bench_function(
                format!("{} {}(1)", name, if lalr { "LALR" } else { "LR" }),
                |b| {
                    b.iter_batched(
                        || tokens.clone(),
                        |tokens| {
                            criterion::black_box(parser.parse(tokens)).unwrap();
                        },
                        BatchSize::PerIteration,
                    );
                },
            );
        }
    }
}

criterion_group! {
    name =
        benches;

    config =
        Criterion::default()
            .sample_size(30)
            .confidence_level(0.95)
            .with_plots();

    targets =
        benchmark_parsing_json,
}
