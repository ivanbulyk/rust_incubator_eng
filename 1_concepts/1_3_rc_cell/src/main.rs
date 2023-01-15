use step_1_3::stack::GlobalStack;

fn main() {
    println!("Implemented you!");

    let gl_stack: GlobalStack<isize> = GlobalStack::new();
    gl_stack.push(1);
    gl_stack.push(2);
    gl_stack.push(3);

    println!("{:#?}", gl_stack);

    gl_stack.pop();

    println!("{:#?}", gl_stack);

    let a = gl_stack.peek().unwrap();

    println!("{:#?}", gl_stack);

    println!("{:#?}", a);
}
