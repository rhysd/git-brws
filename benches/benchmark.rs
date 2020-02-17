use criterion::{criterion_group, criterion_main, Criterion};
use git_brws::argv::Parsed;
use git_brws::url::build_url;

fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! bench_function {
        ($bench_name:expr, $args:expr) => {
            c.bench_function($bench_name, |b| {
                b.iter(|| {
                    let parsed = Parsed::parse_iter($args.iter()).unwrap();
                    if let Parsed::OpenPage(cfg) = parsed {
                        build_url(&cfg).unwrap();
                    } else {
                        assert!(false);
                    }
                })
            });
        };
    }

    bench_function!("no argument", ["git-brws"]);
    bench_function!("commit", ["git-brws", "c6c470c"]);
    bench_function!("diff", ["git-brws", "25d500f..c6c470c"]);
    bench_function!("file", ["git-brws", "README.md"]);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
