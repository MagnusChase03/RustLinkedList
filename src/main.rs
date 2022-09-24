mod linked_list;

fn main() {
    let mut root: linked_list::Node = linked_list::Node {
        data: 0,
        next: linked_list::NextNode::Null,
    };
    root.push(1);
    root.push(2);

    root.print();

    root.pop();

    root.print();
}
