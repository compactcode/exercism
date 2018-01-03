struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn len(&self) -> usize {
        self.next.as_ref().map_or(1, |node| 1 + node.len())
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        self.head.as_ref().map_or(0, |node| node.len())
    }

    pub fn push(&mut self, data: T) {
        self.head = Some(Box::new(Node {
            data: data,
            next: self.head.take()
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.next.take();
            old_head.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}


impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_: &[T]) -> Self {
        unimplemented!()
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}
