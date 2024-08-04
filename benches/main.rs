pub mod parsing_json;

criterion::criterion_main! {
    parsing_json::benches,
}
