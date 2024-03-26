use criterion::{ criterion_group, criterion_main, Criterion};
use ioddengine::{engine::Engine, parser::Parser};


fn parser_benchmark(c: &mut Criterion) {
    let content =  std::fs::read_to_string("../data/05D.xml").unwrap();        
    c.bench_function("parsertest", 
                    | b | b.iter(|| Parser::parse(&content)));
    
    
}

fn engine_benchmark(c : &mut Criterion){
    let content =  std::fs::read_to_string("../data/05D.xml").unwrap();   
    let iodevice = Parser::parse(&content);
    let e = Engine::new(&iodevice, "en");
    c.bench_function("enginebench",
                    | b  | b.iter(|| e.parse("01B1"))); 
}

criterion_group!(benches, parser_benchmark, engine_benchmark);
criterion_main!(benches);