use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
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

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        self.iter().map(|elem| elem.clone()).collect()
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(slice: &[T]) -> Self {
        slice.iter().map(|elem| elem.clone()).collect()
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut result = self.into_iter().collect::<Vec<T>>();
        result.reverse();
        result
    }
}

// An iterator over borrowed values.
pub struct NodeIter<'a, T:'a> {
    next: Option<&'a Node<T>>
}

// An iterator over borrowed values.
impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.data
        })
    }
}

// An iterator over owned values.
pub struct NodeIntoIter<T> {
    list: SimpleLinkedList<T>
}

// An iterator over owned values.
impl<T> Iterator for NodeIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

// Allow the list to be used in a for loop.
impl<T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;
    type IntoIter = NodeIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIntoIter { list: self }
    }
}

// An iterator over owned values.
impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();

        for data in iter {
            list.push(data)
        }

        list
    }
}

// Implement drop to prevent stack overflow during deallocation.
// http://cglab.ca/~abeinges/blah/too-many-lists/book/first-drop.html
impl<T> Drop for SimpleLinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut node) = cur_link {
            cur_link = node.next.take();
        }
    }
}
