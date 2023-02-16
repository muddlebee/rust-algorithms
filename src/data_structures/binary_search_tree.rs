use std::cmp::Ordering;
use std::ops::Deref;
use bencher::Bencher;

/// This struct implements as Binary Search Tree (BST), which is a
/// simple data structure for storing sorted data
/// where clause docs https://doc.rust-lang.org/rust-by-example/generics/where.html
#[derive(Debug)]
#[derive(Clone)]
pub struct BinarySearchTree<T>
    where
        T: Ord + Clone,
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> Default for BinarySearchTree<T>
    where
        T: Ord + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

///TODO:implement delete and modify methods

impl<T> BinarySearchTree<T>
    where
        T: Ord + Clone,
{
    /// Find a value in this tree. Returns True iff value is in this
    /// tree, and false otherwise
    pub fn search(&self, value: &T) -> bool {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Equal => {
                        // key == value
                        true
                    }
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => node.search(value),
                            None => false,
                        }
                    }
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => node.search(value),
                            None => false,
                        }
                    }
                }
            }
            None => false,
        }
    }

    /// Create a new, empty BST
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    /// Returns a new iterator which iterates over this tree in order
    pub fn iter(&self) -> impl Iterator<Item=&T> {
        BinarySearchTreeIter::new(self)
    }

    /// Insert a value into the appropriate location in this tree.
    pub fn insert(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
        } else {
            match &self.value {
                None => (),
                Some(key) => {
                    let target_node = if value < *key {
                        &mut self.left
                    } else {
                        &mut self.right
                    };
                    match target_node {
                        Some(node) => {
                            node.insert(value);
                        }
                        None => {
                            let mut node = BinarySearchTree::new();
                            node.insert(value);
                            *target_node = Some(Box::new(node));
                        }
                    }
                }
            }
        }
    }

    //insert a value into the appropriate location in this tree.
    pub fn insert_node(&mut self, value: &T) {
        if self.value.is_none() {
            self.value = Some(value.clone());
        } else {
            match &self.value {
                Some(key) => {
                    match key.cmp(value) {
                        Ordering::Equal => {}
                        Ordering::Greater => {
                            // key > value
                            match &mut self.left {
                                Some(node) => node.insert_node(value),
                                None => {
                                    //insert a new node and return the reference to the new node
                                    let mut node = BinarySearchTree::new();
                                    node.value = Some(value.clone());
                                    self.left = Some(Box::new(node));

                                },
                            };
                        }
                        Ordering::Less => {
                            // key < value
                            match &mut self.right {
                                Some(node) => node.insert_node(value),
                                None => {
                                    let mut node = BinarySearchTree::new();
                                    node.value = Some(value.clone());
                                    self.right = Some(Box::new(node));

                                },
                            };
                        }
                    }
                }
                None => {

                }
            }
        }
    }


    /// Returns the smallest value in this tree
    pub fn minimum(&self) -> Option<&T> {
        match &self.left {
            Some(node) => node.minimum(),
            None => self.value.as_ref(),
        }
    }

    /// Returns the largest value in this tree
    pub fn maximum(&self) -> Option<&T> {
        match &self.right {
            Some(node) => node.maximum(),
            None => self.value.as_ref(),
        }
    }

    /// Returns the largest value in this tree smaller than value
    pub fn floor(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => node.floor(value),
                            None => None,
                        }
                    }
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => {
                                let val = node.floor(value);
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            }
                            None => Some(key),
                        }
                    }
                    Ordering::Equal => Some(key),
                }
            }
            None => None,
        }
    }

    /// Returns the smallest value in this tree larger than value
    pub fn ceil(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => node.ceil(value),
                            None => None,
                        }
                    }
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => {
                                let val = node.ceil(value);
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            }
                            None => Some(key),
                        }
                    }
                    Ordering::Equal => {
                        // key == value
                        Some(key)
                    }
                }
            }
            None => None,
        }
    }

    pub fn delete(&mut self, value: &T) {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Equal => {
                        // key == value
                        if self.left.is_none() && self.right.is_none() {
                            self.value = None;
                        } else if self.left.is_none() {
                            match &mut self.right.take() {
                                Some(node) => {
                                    self.value = node.value.take();
                                    self.left = node.left.take();
                                    self.right = node.right.take();
                                }
                                None => (),
                            }
                        } else if self.right.is_none() {
                            match &mut self.left.take() {
                                Some(node) => {
                                    self.value = node.value.take();
                                    self.left = node.left.take();
                                    self.right = node.right.take();
                                }
                                None => (),
                            }
                        } else {
                            let mut node = self.right.as_mut().unwrap();
                            while node.left.is_some() {
                                node = node.left.as_mut().unwrap();
                            }
                            self.value = node.value.take();
                            node.delete(&value);
                        }
                    }
                    Ordering::Greater => {
                        // key > value
                        match &mut self.left {
                            Some(node) => node.delete(value),
                            None => (),
                        }
                    }
                    Ordering::Less => {
                        // key < value
                        match &mut self.right {
                            Some(node) => node.delete(value),
                            None => (),
                        }
                    }
                }
            }
            None => (),
        }
    }

    fn inorder_successor(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Equal => {
                        // key == value
                        match &self.right {
                            Some(node) => node.minimum(),
                            None => None,
                        }
                    }
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => {
                                let val = node.inorder_successor(value);
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            }
                            None => Some(key),
                        }
                    }
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => node.inorder_successor(value),
                            None => None,
                        }
                    }
                }
            }
            None => None,
        }
    }

    fn inorder_predecessor(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Equal => {
                        // key == value
                        match &self.left {
                            Some(node) => node.maximum(),
                            None => None,
                        }
                    }
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => {
                                let val = node.inorder_predecessor(value);
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            }
                            None => Some(key),
                        }
                    }
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => node.inorder_predecessor(value),
                            None => None,
                        }
                    }
                }
            }
            None => None,
        }
    }

    fn delete_node(&mut self, value: &T) {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Equal => {
                        // key == value
                        if self.left.is_none() && self.right.is_none() {
                            self.value = None;
                        } else if self.left.is_none() {
                            match &mut self.right.take() {
                                Some(node) => {
                                    self.value = node.value.take();
                                    self.left = node.left.take();
                                    self.right = node.right.take();
                                }
                                None => (),
                            }
                        } else if self.right.is_none() {
                            match &mut self.left.take() {
                                Some(node) => {
                                    self.value = node.value.take();
                                    self.left = node.left.take();
                                    self.right = node.right.take();
                                }
                                None => (),
                            }
                        } else {
                            let mut node = self.right.as_mut().unwrap();
                            while node.left.is_some() {
                                node = node.left.as_mut().unwrap();
                            }
                            self.value = node.value.take();
                            node.delete(&value);
                        }
                    }
                    Ordering::Greater => {
                        // key > value
                        match &mut self.left {
                            Some(node) => node.delete(value),
                            None => (),
                        }
                    }
                    Ordering::Less => {
                        // key < value
                        match &mut self.right {
                            Some(node) => node.delete(value),
                            None => (),
                        }
                    }
                }
            }
            None => (),
        }
    }
}

struct BinarySearchTreeIter<'a, T>
    where
        T: Ord + Clone,
{
    stack: Vec<&'a BinarySearchTree<T>>,
}

impl<'a, T> BinarySearchTreeIter<'a, T>
    where
        T: Ord + Clone,
{
    pub fn new(tree: &BinarySearchTree<T>) -> BinarySearchTreeIter<T> {
        let mut iter = BinarySearchTreeIter { stack: vec![tree] };
        iter.stack_push_left();
        iter
    }

    fn stack_push_left(&mut self) {
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
        }
    }
}

impl<'a, T> Iterator for BinarySearchTreeIter<'a, T>
    where
        T: Ord + Clone
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.stack.is_empty() {
            None
        } else {
            let node = self.stack.pop().unwrap();
            if node.right.is_some() {
                self.stack.push(node.right.as_ref().unwrap().deref());
                self.stack_push_left();
            }
            node.value.as_ref()
        }
    }
}

#[cfg(test)]
mod test {
    use bencher::Bencher;
    use crate::data_structures::binary_search_tree::test;
    use super::BinarySearchTree;

    fn prequel_memes_tree() -> BinarySearchTree<&'static str> {
        let mut tree = BinarySearchTree::new();
        tree.insert("hello there");
        tree.insert("general kenobi");
        tree.insert("you are a bold one");
        tree.insert("kill him");
        tree.insert("back away...I will deal with this jedi slime myself");
        tree.insert("your move");
        tree.insert("you fool");
        print!("{:#?}", tree);
        tree
    }


    #[test]
    fn test_search() {
        let tree = prequel_memes_tree();
        assert!(tree.search(&"hello there"));
        assert!(tree.search(&"you are a bold one"));
        assert!(tree.search(&"general kenobi"));
        assert!(tree.search(&"you fool"));
        assert!(tree.search(&"kill him"));
        assert!(
            !tree.search(&"but i was going to tosche station to pick up some power converters")
        );
        assert!(!tree.search(&"only a sith deals in absolutes"));
        assert!(!tree.search(&"you underestimate my power"));
    }

    #[test]
    fn test_maximum_and_minimum() {
        assert_eq!(BinarySearchTree::<i32>::new().maximum(), None);
        let tree = prequel_memes_tree();
        assert_eq!(*tree.maximum().unwrap(), "your move");
        assert_eq!(
            *tree.minimum().unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        let mut tree2: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree2.maximum().is_none());
        assert!(tree2.minimum().is_none());
        tree2.insert(0);
        assert_eq!(*tree2.minimum().unwrap(), 0);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(-5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 5);
    }

    #[test]
    fn test_floor_and_ceil() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.floor(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .floor(&"these are not the droids you're looking for")
                .unwrap(),
            "kill him"
        );
        assert!(tree.floor(&"another death star").is_none());
        assert_eq!(*tree.floor(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.floor(&"but i was going to tasche station").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(
            *tree.floor(&"you underestimate my power").unwrap(),
            "you fool"
        );
        assert_eq!(*tree.floor(&"your new empire").unwrap(), "your move");
        assert_eq!(*tree.ceil(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .ceil(&"these are not the droids you're looking for")
                .unwrap(),
            "you are a bold one"
        );
        assert_eq!(
            *tree.ceil(&"another death star").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(*tree.ceil(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.ceil(&"but i was going to tasche station").unwrap(),
            "general kenobi"
        );
        assert_eq!(
            *tree.ceil(&"you underestimate my power").unwrap(),
            "your move"
        );
        assert!(tree.ceil(&"your new empire").is_none());
    }

    #[test]
    fn test_iterator() {
        let tree = prequel_memes_tree();
        let mut iter = tree.iter();
        assert_eq!(
            iter.next().unwrap(),
            &"back away...I will deal with this jedi slime myself"
        );
        assert_eq!(iter.next().unwrap(), &"general kenobi");
        assert_eq!(iter.next().unwrap(), &"hello there");
        assert_eq!(iter.next().unwrap(), &"kill him");
        assert_eq!(iter.next().unwrap(), &"you are a bold one");
        assert_eq!(iter.next().unwrap(), &"you fool");
        assert_eq!(iter.next().unwrap(), &"your move");
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }


    //generate test cases for delete method
    #[test]
    fn test_delete() {
        let mut tree = prequel_memes_tree();
        tree.delete(&"hello there");
        assert_eq!(tree.search(&"hello there"), false);
        assert_eq!(tree.search(&"general kenobi"), true);
        assert_eq!(tree.search(&"you are a bold one"), true);
        assert_eq!(tree.search(&"kill him"), true);
        assert_eq!(tree.search(&"back away...I will deal with this jedi slime myself"), true);
        tree.delete(&"general kenobi");
        assert_eq!(tree.search(&"hello there"), false);
        assert_eq!(tree.search(&"general kenobi"), false);
    }

    //add tests for inorder succesor
    #[test]
    fn test_inorder_succesor() {
        let mut tree = prequel_memes_tree();
        assert_eq!(tree.inorder_successor(&"hello there"), Some(&"kill him"));
        assert_eq!(tree.inorder_successor(&"general kenobi"), Some(&"hello there"));
        assert_eq!(tree.inorder_successor(&"you are a bold one"), Some(&"you fool"));
        assert_eq!(tree.inorder_successor(&"kill him"), Some(&"you are a bold one"));
        assert_eq!(tree.inorder_successor(&"back away...I will deal with this jedi slime myself"), Some(&"general kenobi"));
        assert_eq!(tree.inorder_successor(&"your move"), None);
        assert_eq!(tree.inorder_successor(&"you fool"), Some(&"your move"));
    }

    //add tests for inorder predecessor
    #[test]
    fn test_inorder_predecessor() {
        let mut tree = prequel_memes_tree();
        assert_eq!(tree.inorder_predecessor(&"hello there"), Some(&"general kenobi"));
        assert_eq!(tree.inorder_predecessor(&"general kenobi"), Some(&"back away...I will deal with this jedi slime myself"));
        assert_eq!(tree.inorder_predecessor(&"you are a bold one"), Some(&"kill him"));
    }

    //add tests for insert_node
    #[test]
    fn test_insert_node() {
        let mut tree2: BinarySearchTree<i32> = BinarySearchTree::new();
        tree2.insert_node(&1);
        assert_eq!(tree2.search(&1), true);
        tree2.insert_node(&2);
        print!("{:?}", tree2);
        assert_eq!(tree2.search(&2), true);
        tree2.insert_node(&3);
        tree2.insert_node(&4);
        tree2.insert_node(&5);
        assert_eq!(tree2.search(&5), true);
    }
}
