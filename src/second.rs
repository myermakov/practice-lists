//use std::mem;

struct Node<T> {
   elem: T, 
   next: Link<T>,   
}


pub struct List<T> {
    head: Link<T>, 
}


type Link<T> = Option<Box<Node<T>>>;


impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None } 
    }

    pub fn push(&mut self, elem: T)  {
        let new_node = Box::new(Node {
                elem: elem, 
                next: self.head.take()
            });
        self.head = Some(new_node);
    }


    pub fn pop(&mut self) -> Option<T>  {
      self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.elem
        })
    }
    //as_ref()
    //as_mut()
    pub fn peek(&self) -> Option<&T>  {
    self.head.as_ref().map(|node| {
        &node.elem
    })
    }





}

impl<T> Drop for List<T> {

fn drop(&mut self) {
    let mut cur_link = self.head.take();
    while let Some(mut boxed_node) = cur_link {
       cur_link = boxed_node.next.take(); 
    }
  }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
pub fn into_iter(self) -> IntoIter<T> {
  IntoIter(self)
  }
}

impl<T> Iterator for IntoIter<T> {
type Item = T;
fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

#[cfg(test)]
mod test {
use super::List;

#[test]
fn iterations() {
    let mut list = List::new();
    list.push("lol");
    list.push("wat");
    list.push("stahp");
    let mut it = list.into_iter();
    assert_eq!(it.next(),Some("stahp"));
    assert_eq!(it.next(),Some("wat"));
    assert_eq!(it.next(),Some("lol"));
    assert_eq!(it.next(),None);
    assert_eq!(it.next(),None);
}


#[test]
fn basics() {
  let mut list = List::new();
  assert_eq!(list.peek(), None);
  assert_eq!(list.pop(), None);
  assert_eq!(list.peek(), None);
  list.push(1);
  list.push(2);
  list.push(3);
  assert_eq!(list.pop(), Some(3));
  assert_eq!(list.pop(), Some(2));

  list.push(4);
  list.push(5);
  assert_eq!(list.peek(), Some(&5));
  assert_eq!(list.pop(), Some(5));
  assert_eq!(list.pop(), Some(4));

  assert_eq!(list.peek(), Some(&1));
  assert_eq!(list.pop(), Some(1));

  assert_eq!(list.pop(), None);
  assert_eq!(list.pop(), None);
}
}
