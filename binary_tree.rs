use std::fmt::Debug;
use std::fmt::Display;

type RefNode<T> = Option<Box<Node<T>>>;

#[derive(Default, Debug, Clone)]
struct Node<T> {
    v: T,
    left: RefNode<T>,
    right: RefNode<T>,
}

#[derive(Debug)]
enum Action<T, U> {
    Call(T),
    Handle(U),
}

impl<T: Display + Clone> Node<T> {
    #[allow(dead_code)]
    fn new(v: T) -> Self {
        Self {
            v,
            left: None,
            right: None,
        }
    }

    fn ref_node(v: T, left: RefNode<T>, right: RefNode<T>) -> RefNode<T> {
        Some(Box::new(Self { v, left, right }))
    }

    #[allow(dead_code)]
    fn print_tree(&self, level: usize) {
        if let Some(right) = &self.right {
            right.print_tree(level + 1);
        }

        for _ in 0..level {
            print!("  ");
        }
        println!("{}", self.v);

        if let Some(left) = &self.left {
            left.print_tree(level + 1);
        }
    }

    #[allow(dead_code)]
    fn visit_node(&self) {
        print!("{}, ", self.v);

        if let Some(n) = &self.left {
            n.visit_node();
        }

        if let Some(n) = &self.right {
            n.visit_node();
        }
    }
}

#[allow(dead_code)]
fn invert_tree<T: Clone>(root: RefNode<T>) -> RefNode<T> {
    match root {
        Some(node) => Some(Box::new(Node {
            v: node.v.clone(),

            left: invert_tree(node.right),
            right: invert_tree(node.left),
        })),
        None => None,
    }
}

fn invert_tree_stack<T: Clone + Debug>(root: &RefNode<T>) -> RefNode<T> {
    let mut ret_stack = Vec::<RefNode<T>>::new();
    let mut call_stack = Vec::<Action<&RefNode<T>, _>>::new();

    call_stack.push(Action::Call(&root));

    while let Some(action) = call_stack.pop() {
        match action {
            Action::Call(Some(n)) => {
                call_stack.push(Action::Handle(&n.v));
                call_stack.push(Action::Call(&n.right));
                call_stack.push(Action::Call(&n.left));
            }

            Action::Call(None) => {
                ret_stack.push(None);
            }

            Action::Handle(v) => {
                let l = ret_stack.pop().unwrap();
                let r = ret_stack.pop().unwrap();

                ret_stack.push(Some(Box::new(Node {
                    v: v.clone(),
                    left: l.clone(),
                    right: r.clone(),
                })));
            }
        }
    }

    ret_stack.pop().unwrap()
}

fn print_tree_st<T: Display>(root: &RefNode<T>) {
    let mut stack = Vec::<Action<(&RefNode<T>, usize), (&T, usize)>>::new();

    stack.push(Action::Call((root, 0)));

    while let Some(action) = stack.pop() {
        match action {
            Action::Call((Some(n), level)) => {
                stack.push(Action::Call((&n.left, level + 1)));
                stack.push(Action::Handle((&n.v, level)));
                stack.push(Action::Call((&n.right, level + 1)));
            }
            Action::Handle((v, level)) => {
                for _ in 0..level {
                    print!("  ");
                }
                println!("{v}");
            }

            _ => {}
        }
    }
}

#[allow(dead_code)]
fn generate_tree(depth: usize, counter: &mut usize) -> RefNode<usize> {
    if depth == 0 {
        return None;
    }

    *counter += 1;
    let mut node = Node::new(*counter);

    node.left = generate_tree(depth - 1, counter);

    node.right = generate_tree(depth - 1, counter);

    Some(Box::new(node))
}

fn generate_tree_st(depth: usize) -> RefNode<usize> {
    let mut call_st = Vec::<Action<(RefNode<usize>, usize), usize>>::new();
    let mut ret_st = Vec::<RefNode<usize>>::new();

    call_st.push(Action::Call((Node::ref_node(0, None, None), depth)));

    let mut counter = 0;

    while let Some(action) = call_st.pop() {
        match action {
            Action::Call((Some(_), d)) => {
                counter += 1;
                call_st.push(Action::Handle(counter));

                if d > 1 {
                    let l = Node::ref_node(0, None, None);
                    let r = Node::ref_node(0, None, None);

                    call_st.push(Action::Call((r, d - 1)));
                    call_st.push(Action::Call((l, d - 1)));
                } else {
                    call_st.push(Action::Call((None, 0)));
                    call_st.push(Action::Call((None, 0)));
                }
            }

            Action::Call((None, _)) => {
                ret_st.push(None);
            }

            Action::Handle(c) => {
                let r = ret_st.pop().unwrap();
                let l = ret_st.pop().unwrap();

                let ref_node = Node::ref_node(c.clone(), l, r);

                ret_st.push(ref_node);
            }
        }
    }

    ret_st.pop().unwrap()
}

fn main() {
    println!("Recursive calls");
    let root = generate_tree(3, &mut 0);
    root.as_ref().unwrap().print_tree(0);

    println!("");
    println!("-------------- After invert ------------ ");
    println!("");
    let t = invert_tree(root);
    t.as_ref().unwrap().print_tree(0);

    println!("");

    println!(" ---------------------------- ");

    println!("");

    println!("Non-Recursive calls");

    let root = generate_tree_st(3);
    print_tree_st(&root);

    println!("");
    println!("-------------- After invert ------------ ");
    println!("");

    let t = invert_tree_stack(&root);
    print_tree_st(&t);
}
