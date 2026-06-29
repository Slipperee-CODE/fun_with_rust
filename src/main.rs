enum Node<T> {
    Link(Box<LinkedList<T>>),
    Nil
}

struct LinkedList<T> {
    first: T,
    rest: Node<T>,
}

impl<T> LinkedList<T> {
    pub fn new(item: T, rest: Node<T>) -> Self {
        LinkedList {
            first: item,
            rest: Node::<T>::Nil,
        }
    }

    fn add_first(self: Self, item: T) {
        let curr: Self = self; // when declaring a new variable in rust, always use let (and mut if you want the variable to be mutable) 
        self = LinkedList::new(item, rest);
    }
}


fn main() {
    println!("Hello, world!");

    let mut lst: LinkedList<i32> = LinkedList::<i32>::new(1, Node::<i32>::Nil);
    lst.add_first(2);
    println!("({})", lst.first);
}
