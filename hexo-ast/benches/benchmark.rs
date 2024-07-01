use std::io::Read;
use criterion::{Criterion, criterion_group, criterion_main, black_box};

use hexo_ast::AstParser;

fn java_file(c: &mut Criterion) {
    let mut source_file = std::fs::File::open("benches/java_file.hexo").unwrap();
    let mut source_buffer = String::new();
    source_file.read_to_string(&mut source_buffer).unwrap();
    let source_buffer_str_x1 = source_buffer.as_str();

    let parser = AstParser::default();

    c.bench_function("java-file", |b| b.iter(|| {
        black_box(parser.parse(source_buffer_str_x1).unwrap());
    }));

    let mut source_buffer_str_x1000 = String::new();
    for _ in 0..1000 {
        source_buffer_str_x1000.push_str(source_buffer_str_x1);
    }

    c.bench_function("java-file-x1000", |b| b.iter(|| {
        black_box(parser.parse(source_buffer_str_x1000.as_str()).unwrap());
    }));
}

criterion_group!(benches, java_file);
criterion_main!(benches);