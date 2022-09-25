mod linked_list;
use linked_list::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push(0);
    list.push(1);
    list.print();

    list.pop();
    list.print();
}
