struct Node {
    elem: u32,
    next: Link,
}

type Link = Option<Box<Node>>;

struct LinkedList {
    head: Link,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let link = Some(Box::new(Node {
            elem: 1,
            next: None,
        }));
        let list = LinkedList { head: link };
    }
}
