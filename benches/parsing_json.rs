use {
    criterion::{
        criterion_group,
        BatchSize,
        Criterion,
        Throughput,
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
        let parser = Parser::new(grammar).unwrap();
        let tokens = parser.tokenize(input).unwrap();
        group.bench_function(name, |b| {
            b.iter_batched(
                || tokens.clone(),
                |tokens| {
                    criterion::black_box(parser.parse(tokens).unwrap());
                },
                BatchSize::PerIteration,
            );
        });
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
