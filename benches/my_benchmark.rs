use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let filename = String::from("./src/data/3.csv");
    let test_name = String::from("bench_naive");
    let query = String::from("count(*) where dob = 1990");
    c.bench_function(test_name.as_str(), |b| {
        b.iter(|| {
            csvseeker::query_data(&filename, &query);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);