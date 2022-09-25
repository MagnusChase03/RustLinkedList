type NextNode<T> = Option<Box<Node<T>>>;
struct Node<T> {
    data: T,
    next: NextNode<T>,
}

impl<T: std::fmt::Debug> Node<T> {
    fn print(&self) {
        println!("{:?} ", self.data);

        match &self.next {
            None => {}
            Some(node) => {
                node.print();
            }
        }
    }
}

pub struct LinkedList<T> {
    head: NextNode<T>,
}

impl<T: std::fmt::Debug> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: NextNode::None,
        }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data: data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        self.head.take().map(|node| {
            self.head = node.next;
        });
    }

    pub fn print(&self) {
        match &self.head {
            None => {}
            Some(node) => {
                node.print();
            }
        }
    }
}
