use std::env::args;

mod demos;
mod ray;
mod vec3;

fn main() {
    let arg: Vec<String> = args().collect();
    if arg.len() != 2 {
        eprintln!("Error input\nExample: {} 01", arg[0]);
        return;
    }

    let demos = vec![demos::demo01::run];
    let idx: usize = arg[1].parse().unwrap();

    if idx > demos.len() || idx <= 0 {
        eprintln!("Number range: 1 ~ {}", demos.len());
        return;
    }

    demos[idx - 1]();
}
