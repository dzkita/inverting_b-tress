use std::fmt::{Debug, Display};

type NodeRef<T> = Option<Box<Node<T>>>;

#[derive(Default, Debug, Clone)]
struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

#[derive(Debug)]
enum Action<T, U> {
    Call(T),
    Handle(U),
}

fn generate_tree_nonrec(level: usize) -> NodeRef<i32> {
    let mut counter = 1;
    let mut args_stack = Vec::<Action<usize, i32>>::new();
    let mut ret_stack = Vec::<NodeRef<i32>>::new();
    use Action::*;
    args_stack.push(Call(level));

    while let Some(action) = args_stack.pop() {
        match action {
            Call(level) => {
                if level > 0 {
                    args_stack.push(Handle(counter));
                    counter += 1;
                    args_stack.push(Call(level - 1));
                    args_stack.push(Call(level - 1));
                } else {
                    ret_stack.push(None);
                }
            }
            Handle(value) => {
                let left = ret_stack.pop().unwrap();
                let right = ret_stack.pop().unwrap();
                ret_stack.push(Some(Box::new(Node { value, left, right })));
            }
        }
    }
    ret_stack.pop().unwrap()
}

#[allow(dead_code)]
fn generate_tree(level: usize, counter: &mut i32) -> NodeRef<i32> {
    if level == 0 {
        return None;
    } else {
        let mut node = Node {
            value: *counter,
            left: None,
            right: None,
        };
        *counter += 1;
        node.left = generate_tree(level - 1, counter);
        node.right = generate_tree(level - 1, counter);
        Some(Box::new(node))
    }
}

fn invert_tree_nonrec<T: Clone + Debug>(root: &NodeRef<T>) -> NodeRef<T> {
    let mut args_stack = Vec::<Action<&NodeRef<T>, &T>>::new();

    let mut ret_stack = Vec::<NodeRef<T>>::new();
    use Action::*;

    args_stack.push(Call(root));
    while let Some(action) = args_stack.pop() {
        // println!("Action: {:?} ",action);
        match action {
            Call(root) => {
                if let Some(node) = root {
                    args_stack.push(Handle(&node.value));
                    args_stack.push(Call(&node.right));
                    args_stack.push(Call(&node.left));
                } else {
                    ret_stack.push(None)
                }
            }
            Handle(value) => {
                let left = ret_stack.pop().unwrap();
                let right = ret_stack.pop().unwrap();
                ret_stack.push(Some(Box::new(Node {
                    value: value.clone(),
                    left,
                    right,
                })));
            }
        }
        // println!("ret_stack :{:?}",ret_stack);
        // println!("----------------------");
    }
    ret_stack.pop().unwrap()
}

fn print_tree_nonrec<T: Display>(root: &NodeRef<T>) {
    let mut stack = Vec::<Action<(&NodeRef<T>, usize), (&T, usize)>>::new();

    use Action::*;
    stack.push(Call((root, 0)));
    while let Some(action) = stack.pop() {
        match action {
            Call((root, level)) => {
                if let Some(node) = root {
                    stack.push(Call((&node.left, level + 1)));
                    stack.push(Handle((&node.value, level)));
                    stack.push(Call((&node.right, level + 1)));
                }
            }
            Handle((value, level)) => {
                for _ in 0..level {
                    print!("  ");
                }
                println!("{}", value);
            }
        }
    }
}

fn main() {
    // let mut counter = 1;
    let tree = generate_tree_nonrec(3);
    // print_tree(&tree, 0);
    print_tree_nonrec(&tree);
    println!("------------------------");
    // visit_nodes(&tree);
    print_tree_nonrec(&invert_tree_nonrec(&tree));
}
