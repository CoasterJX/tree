use super::*;
use std::fmt::Debug;
use red_black_tree::{RBTreeNode as RB, NodeColor as NC, Direction as Dir, RBTChild as TRoot};

#[derive(Clone, Debug)]
pub struct RedBlackTree<T: Ord + Clone> {
    pub root: TRoot<T>,
    pub num_leaves: u128,
    pub height: u128,
}

impl<T: Ord + Clone + Debug> RedBlackTree<T> {

    pub fn new() -> RedBlackTree<T> {
        RedBlackTree { 
            root: None, 
            num_leaves: 0, 
            height: 0
        }
    }

    pub fn print(&self) {
        RB::print_tree(&self.root)
    }

    pub fn insert(&mut self, key: &T) {
        match &self.root {
            Some(_) => RB::insert_node(&self.root, key.clone()),
            None => self.root = RB::new(key.clone()),
        };
        let mut z = RB::find_node(&self.root, key.clone());

        while RB::get_root_color(&RB::get_parent(&z)) == NC::Red {
            if RB::is_node_equal(
                &RB::get_parent(&z),
                &RB::get_left(&RB::get_parent(&RB::get_parent(&z)))
            ) {
                let y = RB::get_right(&RB::get_parent(&RB::get_parent(&z)));
                if RB::get_root_color(&y) == NC::Red {
                    RB::set_root_color(&RB::get_parent(&z), NC::Black);
                    RB::set_root_color(&y, NC::Black);
                    RB::set_root_color(&RB::get_parent(&RB::get_parent(&z)), NC::Red);
                    z = RB::get_parent(&RB::get_parent(&z));
                } else {
                    if RB::is_node_equal(
                        &z,
                        &RB::get_right(&RB::get_parent(&z))
                    ) {
                        z = RB::get_parent(&z);
                        self.root = RB::left_rotate(self.root.clone(), RB::get_root_key(&z));
                    }
                    RB::set_root_color(&RB::get_parent(&z), NC::Black);
                    RB::set_root_color(&RB::get_parent(&RB::get_parent(&z)), NC::Red);
                    self.root = RB::right_rotate(self.root.clone(), RB::get_root_key(&RB::get_parent(&RB::get_parent(&z))));
                }
            } else {
                let y = RB::get_left(&RB::get_parent(&RB::get_parent(&z)));
                if RB::get_root_color(&y) == NC::Red {
                    RB::set_root_color(&RB::get_parent(&z), NC::Black);
                    RB::set_root_color(&y, NC::Black);
                    RB::set_root_color(&RB::get_parent(&RB::get_parent(&z)), NC::Red);
                    z = RB::get_parent(&RB::get_parent(&z));
                } else {
                    if RB::is_node_equal(
                        &z,
                        &RB::get_left(&RB::get_parent(&z))
                    ) {
                        z = RB::get_parent(&z);
                        self.root = RB::right_rotate(self.root.clone(), RB::get_root_key(&z));
                    }
                    RB::set_root_color(&RB::get_parent(&z), NC::Black);
                    RB::set_root_color(&RB::get_parent(&RB::get_parent(&z)), NC::Red);
                    self.root = RB::left_rotate(self.root.clone(), RB::get_root_key(&RB::get_parent(&RB::get_parent(&z))));
                }
            }
        }
        RB::set_root_color(&self.root, NC::Black);
    }
}