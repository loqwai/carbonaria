use std::time::Instant;

#[allow(dead_code)]
pub fn profile<F>(label: &str, f: F)
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    println!("{}: {:?}", label, start.elapsed());
}
