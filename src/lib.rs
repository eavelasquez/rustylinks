pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    fn empty() -> LinkedList<T> {
        LinkedList { head: None }
    }

    fn push(&mut self, element: T) {
        let new_head = Box::new(Node {
            elem: element,
            next: self.head.take(),
        });

        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.elem
        })
    }

    fn peak(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.elem)
    }
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut linked_list = LinkedList::empty();
        linked_list.push(1024);
        linked_list.push(2048);
    }
}
