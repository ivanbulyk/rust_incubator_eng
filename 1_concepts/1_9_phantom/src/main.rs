use step_1_9::Fact;

fn main() {
    println!("Implementing you!");

    let f: Fact<i32> = Fact::new();
    // let v: Fact<i32> = Fact::new();
    println!("Fact about i32: {:#?}", f.fact());
    println!("Fact about i32: {:#?}", f.fact());
}
