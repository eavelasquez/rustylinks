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

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|n| &mut n.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
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

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // note: we can use self.0 here because IntoIter is a tuple struct
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n| {
            self.next = n.next.as_deref();
            &n.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|n| {
            self.next = n.next.as_deref_mut();
            &mut n.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn basics() {
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

        // Check normal removal
        assert_eq!(linked_list.pop(), Some(5120));
        assert_eq!(linked_list.pop(), Some(4096));

        // Check exhaustion
        assert_eq!(linked_list.pop(), Some(1024));
        assert_eq!(linked_list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut linked_list = LinkedList::empty();

        // Check empty linked list behaves right
        assert_eq!(linked_list.peak(), None);
        assert_eq!(linked_list.peek_mut(), None);

        // Populate linked list
        linked_list.push(1024);
        linked_list.push(2048);
        linked_list.push(3072);

        // Check peak
        assert_eq!(linked_list.peak(), Some(&3072));

        // Check peak mut
        assert_eq!(linked_list.peek_mut(), Some(&mut 3072));
        linked_list.peek_mut().map(|value| *value = 4096);
        assert_eq!(linked_list.peek_mut(), Some(&mut 4096));

        // Check normal removal
        assert_eq!(linked_list.pop(), Some(4096));
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

    #[test]
    fn iter() {
        let mut linked_list = LinkedList::empty();

        linked_list.push(1024);
        linked_list.push(2048);
        linked_list.push(3072);

        let mut iter = linked_list.iter();

        assert_eq!(iter.next(), Some(&3072));
        assert_eq!(iter.next(), Some(&2048));
        assert_eq!(iter.next(), Some(&1024));
    }

    #[test]
    fn iter_mut() {
        let mut linked_list = LinkedList::empty();

        linked_list.push(1024);
        linked_list.push(2048);
        linked_list.push(3072);

        let mut iter = linked_list.iter_mut();

        assert_eq!(iter.next(), Some(&mut 3072));
        assert_eq!(iter.next(), Some(&mut 2048));
        assert_eq!(iter.next(), Some(&mut 1024));
    }
}
