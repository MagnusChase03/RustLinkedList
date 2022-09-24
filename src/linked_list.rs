pub enum NextNode {
    Null,
    NextNode(Box<Node>),
}

pub struct Node {
    pub data: i32,
    pub next: NextNode,
}

impl Node {
    pub fn push(&mut self, data: i32) {
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

    pub fn pop(&mut self) {

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

    pub fn print(&self) {
        println!("{} ", self.data);
        match self.next {
            NextNode::Null => {},
            NextNode::NextNode(ref next_node) => {
                next_node.print();
            }
        }
    }
}