pub struct LinkedList {
    head: Link,
}

impl LinkedList {
    fn empty() -> LinkedList {
        LinkedList { head: None }
    }

    fn push(&mut self, element: u32) {
        let old_head = self.head.take();
        let new_head = Box::new(Node {
            elem: element,
            next: old_head,
        });

        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<u32> {
        let old_head = self.head.take();
        match old_head {
            None => None,
            Some(n) => {
                self.head = n.next;
                Some(n.elem)
            },
        }
    }
}

struct Node {
    elem: u32,
    next: Link,
}

type Link = Option<Box<Node>>;

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
