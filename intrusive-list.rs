struct Node<'a, T> {
    prev: Option<&'a Node<'a, T>>,
    value: T,
}

impl<'a, T> Node<'a, T> {
    fn new(value: T) -> Self {
        Self {
            prev: None,
            value,
        }
    }

    fn append(&'a self, value: T) -> Self {
        Self {
            prev: Some(self),
            value,
        }
    }
}

impl<'a, T: std::fmt::Display> Node<'a, T> {
    fn display(&self) {
        print!("{}", self.value);
        match self.prev {
            None => println!(),
            Some(node) => {
                print!(" ");
                node.display();
            }
        }
    }
}

fn main() {
    let first = Node::new(1u32);
    let second = first.append(2);
    let third = second.append(3);
    third.display();
}
