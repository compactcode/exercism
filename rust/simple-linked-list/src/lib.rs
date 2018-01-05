pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

pub struct NodeIter<'a, T:'a> {
    next: Option<&'a Node<T>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        self.iter().count()
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
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn iter(&self) -> NodeIter<T> {
        NodeIter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.data
        })
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut result = SimpleLinkedList::new();

        for data in self.iter() {
            result.push(data.clone());
        }

        result
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(slice: &[T]) -> Self {
        let mut result = SimpleLinkedList::new();

        for data in slice {
            result.push(data.clone());
        }

        result
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut result = vec![];

        while let Some(node) = self.pop() {
            result.push(node);
        }

        result.reverse();
        result
    }
}
