pub struct LinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> LinkedList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

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

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|n| &mut n.elem)
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

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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

        // Check peak mut
        linked_list.peek_mut().map(|value| *value = 6144);
        assert_eq!(linked_list.peek_mut(), Some(&mut 6144));

        // Check normal removal
        assert_eq!(linked_list.pop(), Some(6144));
        assert_eq!(linked_list.pop(), Some(4096));

        // Check exhaustion
        assert_eq!(linked_list.pop(), Some(1024));
        assert_eq!(linked_list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut linked_list = LinkedList::empty();

        linked_list.push(1024);
        linked_list.push(2048);
        linked_list.push(3072);

        let mut iter = linked_list.into_iter();

        assert_eq!(iter.next(), Some(3072));
        assert_eq!(iter.next(), Some(2048));
        assert_eq!(iter.next(), Some(1024));
        assert_eq!(iter.next(), None);
    }
}
