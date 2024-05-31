struct Node<'a, T> {
    prev: Option<&'a mut Node<'a, T>>,
    value: T,
}

impl<'a, T> Node<'a, T> {
    fn new(value: T) -> Self {
        Self {
            prev: None,
            value,
        }
    }

    fn append(&'a mut self, value: T) -> Self {
        Self {
            prev: Some(self),
            value,
        }
    }
}

impl<'a, T: std::fmt::Display> Node<'a, T> {
    fn display(&mut self) {
        print!("{}", self.value);
        match self.prev {
            None => println!(),
            Some(ref mut node) => {
                print!(" ");
                node.display();
            }
        }
    }
}

fn main() {
    let mut first = Node::new(1u32);
    let mut second = first.append(2);
    let mut third = second.append(3);
    third.display();
}
