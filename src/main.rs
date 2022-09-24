enum NextNode {
    Null,
    NextNode(Box<Node>),
}

struct Node {
    data: i32,
    next: NextNode,
}

impl Node {
    fn push(&mut self, data: i32) {
        match self.next {
            NextNode::Null => {
                let new_node: Node = Node {
                    data: data,
                    next: NextNode::Null,
                };
                self.next = NextNode::NextNode(Box::new(new_node));
            },
            NextNode::NextNode(ref mut next_node) => {
                next_node.push(data);
            }
        }
    }

    fn pop(&mut self) {

        match self.next {
            NextNode::Null => {},
            NextNode::NextNode(ref mut next_node) => {
                
                match next_node.next {

                    NextNode::Null => {

                        self.next = NextNode::Null;

                    },
                    NextNode::NextNode(ref _another_node) => {

                        next_node.pop();

                    }

                }

            }
        }

    }

    fn print(&self) {
        println!("{} ", self.data);
        match self.next {
            NextNode::Null => {},
            NextNode::NextNode(ref next_node) => {
                next_node.print();
            }
        }
    }
}

fn main() {
    let mut root: Node = Node {
        data: 0,
        next: NextNode::Null,
    };
    root.push(1);
    root.push(2);

    root.print();

    root.pop();

    root.print();
}
