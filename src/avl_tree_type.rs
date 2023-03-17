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

    pub fn print_tree(&self) {
        AVL::print_tree(&self.root)
    }

    pub fn insert(&mut self, key: &T) {
        match &self.root {
            Some(_) => AVL::insert_node(&self.root, key.clone()),
            None => self.root = AVL::new(key.clone()),
        };
        let mut z = AVL::find_node(&self.root, key.clone());
        
        loop {
            z = AVL::get_parent(&z);

            // TODO: Implement AVL insertion
            let bf: i64 = AVL::get_balance_factor(&z);

            if bf == -2 {
                if AVL::get_balance_factor(&AVL::get_left(&z)) <= 0 {
                    // Left-Left case
                    self.root = AVL::right_rotate(self.root.clone(), AVL::get_root_key(&z));
                    z = AVL::get_parent(&z);  // After the rotation z would be moved down one layer. This set's node z
                    // to be the node that took it's place.
                } else {
                    // Left-Right case
                    self.root = AVL::left_rotate(self.root.clone(), AVL::get_root_key(&AVL::get_left(&z)));
                    self.root = AVL::right_rotate(self.root.clone(), AVL::get_root_key(&z));
                    z = AVL::get_parent(&z);
                }
            } else if bf == 2 {
                if AVL::get_balance_factor(&AVL::get_right(&z)) >= 0 {
                    // Right-Right case
                    self.root = AVL::left_rotate(self.root.clone(), AVL::get_root_key(&z));
                    z = AVL::get_parent(&z);
                } else {
                    // Right-Left case
                    self.root = AVL::right_rotate(self.root.clone(), AVL::get_root_key(&AVL::get_right(&z)));
                    self.root = AVL::left_rotate(self.root.clone(), AVL::get_root_key(&z));
                    z = AVL::get_parent(&z);
                }
            }
            if AVL::is_node_equal(&None, &z) {
                break;
            }
        }
    }
}