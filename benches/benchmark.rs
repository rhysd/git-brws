use criterion::{black_box, criterion_group, criterion_main, Criterion};
use git_brws::argv::Parsed;
use git_brws::url::build_url;
use std::ffi::OsStr;

struct DummyArgs<'a>(Vec<&'a OsStr>);

impl<'a> DummyArgs<'a> {
    fn new(args: &'a [&'a str]) -> DummyArgs<'a> {
        DummyArgs(args.iter().map(AsRef::as_ref).collect())
    }
}

impl<'a> IntoIterator for DummyArgs<'a> {
    type Item = &'a OsStr;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("no argument", |b| {
        b.iter(|| {
            let args = DummyArgs::new(&[]);
            if let Parsed::OpenPage(cfg) = Parsed::from_iter(args).unwrap() {
                build_url(black_box(&cfg)).unwrap();
            } else {
                assert!(false);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
