pub mod stack;

use stack::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gl_stack_pop_push() {
        let gl_stack: GlobalStack<isize> = GlobalStack::new();
        gl_stack.push(1);
        let item = gl_stack.pop();
        assert_eq!(item.unwrap(), 1);
        assert_eq!(gl_stack.is_empty(), true);
    }

    #[test]
    fn gl_stack_peek() {
        let gl_stack: GlobalStack<isize> = GlobalStack::new();
        gl_stack.push(1);
        assert_eq!(gl_stack.peek().unwrap(), 1);
        assert_eq!(gl_stack.is_empty(), false);
    }
}
