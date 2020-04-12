extern crate nom;
#[macro_use]
extern crate criterion;
extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use criterion::Criterion;
use nom::bytes::complete::tag;
use nom::multi;

static COUNT: usize = 1024;

fn parser(i: &str) -> nom::IResult<&str, Vec<&str>> {
  multi::count(tag("abc"), COUNT)(i)
}

fn count(c: &mut Criterion) {
  let mut data = String::new();

  for _ in 0..COUNT {
    data.extend("abc".chars());
  }

  parser(&data[..]).expect("should parse correctly");
  c.bench_function("count", move |b| {
    b.iter(|| parser(&data[..]).unwrap());
  });
}

criterion_group!(benches, count);
criterion_main!(benches);
