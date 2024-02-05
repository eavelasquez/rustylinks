pub struct LinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    fn empty() -> Self {
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

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current_head = self.head.take();
        while let Some(mut n) = current_head {
            current_head = n.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut linked_list = LinkedList::empty();

        // Check empty linked list behaves right
        assert_eq!(linked_list.pop(), None);

        // Populate linked list
        linked_list.push(1024);
        linked_list.push(2048);
        linked_list.push(3072);

        // Check normal removal
        assert_eq!(linked_list.pop(), Some(3072));
        assert_eq!(linked_list.pop(), Some(2048));

        // Push some more just to make sure nothing's corrupted
        linked_list.push(4096);
        linked_list.push(5120);

        // Check peak
        assert_eq!(linked_list.peak(), Some(&5120));

        // Check normal removal
        assert_eq!(linked_list.pop(), Some(5120));
        assert_eq!(linked_list.pop(), Some(4096));

        // Check exhaustion
        assert_eq!(linked_list.pop(), Some(1024));
        assert_eq!(linked_list.pop(), None);
    }
}
