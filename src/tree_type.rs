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

    pub fn transplant(&mut self, u: &TRoot<T>, v: &TRoot<T>) {
        let u_node = u.clone();
        let v_node = v.clone();
        match RB::get_parent(&u_node) {
            Some(_) => {
                if RB::is_node_equal(&u_node, &RB::get_left(&RB::get_parent(&u_node))) {
                    RB::set_child(&RB::get_parent(&u_node), v_node.clone(), Dir::Left);
                } else {
                    RB::set_child(&RB::get_parent(&u_node), v_node.clone(), Dir::Right);
                }
            },
            None => self.root = v_node.clone(),
        };
        RB::set_parent(&v_node, &RB::get_parent(&u_node));
    }

    pub fn get_minimum(&self) -> TRoot<T> {
        RB::get_minimum(&self.root)
    }

    fn delete_fixup(&mut self, x: &TRoot<T>) {
        
    }

    pub fn delete(&mut self, key: &T) {
        let z = RB::find_node(&self.root, key.clone());
        match z {
            None => return,
            _ => (),
        };

        let mut x: TRoot<T> = None;
        let mut y = z.clone();
        let mut y_orig_color = RB::get_root_color(&y);

        if let None = RB::get_left(&z) {
            x = RB::get_right(&z);
            self.transplant(&z, &RB::get_right(&z));
        } else if let None = RB::get_right(&z) {
            x = RB::get_left(&z);
            self.transplant(&z, &RB::get_left(&z));
        } else {
            y = RB::get_minimum(&RB::get_right(&z));
            y_orig_color = RB::get_root_color(&y);
            x = RB::get_right(&y);

            if RB::is_node_equal(&RB::get_parent(&y), &z) {
                RB::set_parent(&x, &y);
            } else {
                self.transplant(&y, &RB::get_right(&y));
                RB::set_child(&y, RB::get_right(&z), Dir::Right);
                RB::set_parent(&RB::get_right(&y), &y);
            }

            self.transplant(&z, &y);
            RB::set_child(&y, RB::get_left(&z), Dir::Left);
            RB::set_parent(&RB::get_left(&y), &y);
            RB::set_root_color(&y, RB::get_root_color(&z));
        }

        match y_orig_color {
            NC::Red => (),
            NC::Black => self.delete_fixup(&x),
        };
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