struct Node {
    elem: u32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct List {
    head: Link,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let link = Link::More(Box::new(Node {
            elem: 1,
            next: Link::Empty,
        }));
        let list = List { head: link };
    }
}
