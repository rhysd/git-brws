use criterion::{black_box, criterion_group, criterion_main, Criterion};
use git_brws::argv::Parsed;
use git_brws::config::{Config, EnvConfig};
use git_brws::url::build_url;
use std::env::current_dir;

fn empty_config() -> Config {
    let env = EnvConfig {
        git_command: "git".to_string(),
        ghe_ssh_port: None,
        ghe_url_host: None,
        gitlab_ssh_port: None,
        github_token: None,
        ghe_token: None,
        https_proxy: None,
        browse_command: None,
    };
    Config {
        repo_url: "https://github.com/rhysd/git-brws.git".to_string(),
        branch: None,
        cwd: current_dir().unwrap(),
        args: vec![],
        stdout: true,
        pull_request: false,
        website: false,
        blame: false,
        remote: None,
        env,
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let cfg = empty_config();
    c.bench_function("no argument", |b| {
        b.iter(|| build_url(black_box(&cfg.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
