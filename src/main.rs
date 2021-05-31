use processor::Processor;

mod processor;

fn main() {
    let cpu = Processor::build();
    println!("Hello, world!");
}
