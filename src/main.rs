enum Node<T> {
    Link(Box<LinkedList<T>>),
    Nil
}

struct LinkedList<T> {
    first: T,
    rest: Node<T>,
}

impl<T> LinkedList<T> {
    pub fn new(item: T) -> Self {
        LinkedList {
            first: item,
            rest: Node::<T>::Nil,
        }
    }

    fn add_first(self: &mut Self, item: T) {
        let mut tail: LinkedList<T> = LinkedList::<T>::new(self.first);
        tail.rest = self.rest;
        self.first = item;
        self.rest = Node::<T>::Link(Box::new(tail));
    }
}


fn main() {
    println!("Hello, world!");

    let mut lst: LinkedList<i32> = LinkedList::<i32>::new(1);
    println!("({})", lst.first);
}
