use configuard::{
    common::{read_all_entries, render_all_entries, render_server_config_head},
    new_decoy,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};


const ENTRIES_DIR: &str = "./tests/entries";


fn criterion_read_all_entries(c: &mut Criterion) {
    c.bench_function("+Read all", |b| {
        b.iter(|| read_all_entries(black_box(ENTRIES_DIR)))
    });
}

fn criterion_render_all_entries(c: &mut Criterion) {
    c.bench_function("+Render all", |b| {
        b.iter(|| render_all_entries(black_box(ENTRIES_DIR)))
    });
}

fn criterion_decoy(c: &mut Criterion) {
    c.bench_function("+Decoy", |b| b.iter(new_decoy));
}

fn criterion_render_server_config_head(c: &mut Criterion) {
    c.bench_function("+Server config render", |b| {
        b.iter(|| render_server_config_head(black_box(12345)))
    });
}


criterion_group!(
    benches,
    criterion_decoy,
    criterion_read_all_entries,
    criterion_render_all_entries,
    criterion_render_server_config_head,
);
criterion_main!(benches);
