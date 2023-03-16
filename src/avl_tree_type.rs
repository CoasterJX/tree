use super::*;
use std::fmt::Debug;
use avl_tree::{AVLTreeNode as AVL, Direction as Dir, AVLChild as TRoot};

#[derive(Clone, Debug)]
pub struct AVLTree<T: Ord + Clone> {
    pub root: TRoot<T>,
    pub num_leaves: u128,
    pub height: u128,
}

impl<T: Ord + Clone + Debug> AVLTree<T> {

    pub fn new() -> AVLTree<T> {
        AVLTree { 
            root: None, 
            num_leaves: 0, 
            height: 0
        }
    }

    pub fn print(&self) {
        AVL::print_tree(&self.root)
    }

    pub fn insert(&mut self, key: &T) {
        match &self.root {
            Some(_) => AVL::insert_node(&self.root, key.clone()),
            None => self.root = AVL::new(key.clone()),
        };
        let mut z = AVL::find_node(&self.root, key.clone());

        // TODO: Implement AVL insertion
    }
}