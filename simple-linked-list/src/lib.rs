use std::mem;

#[derive(Clone)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>> // points to the last item inserted on the list
}

#[derive(Clone)]
struct Node<T> {
    data: T,
    prev: Option<Box<Node<T>>>,
}

// LIFO linked list
impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut ptr = &self.head;
        loop {
            match ptr {
                None => return len,
                Some(node) => {
                    len += 1;
                    ptr = &node.prev;
                }
            }
        }
    }

    //  push the node at the end
    pub fn push(&mut self, _element: T) {
        let new_node  = Node {
            data: _element,
            prev: mem::replace(&mut self.head, None)
        };
        let boxed = Some(Box::new(new_node));
        self.head = boxed;
    }

    // remove the last node and return it
    pub fn pop(&mut self) -> Option<T>  {
        match mem::replace(&mut self.head, None) {
            None => return None,
            Some(head) => {
                self.head = head.prev;
                return Some(head.data);
            }
        }
    }

    // return a ref to the head
    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => return None,
            Some(head) => return Some(&head.data),
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut rev = SimpleLinkedList {
            head: None
        };
        let mut clone = (*self).clone();
        loop {
            match clone.pop() {
                None => return rev,
                Some(node) => rev.push(node),
            }
        }
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut list = SimpleLinkedList {
            head: None
        };
        for i in _item {
            list.push((*i).clone());
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut v = Vec::new();
        loop {
            match self.pop() {
                None => return v,
                Some(node) => v.insert(0, node),
            }
        }
    }
}
