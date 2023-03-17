use super::*;
use std::{fmt::Debug, cmp::Ordering};
use red_black_tree::{RBTreeNode as RB, NodeColor as NC, Direction as Dir, RBTChild as TRoot};

#[derive(Clone, Debug)]
pub struct RedBlackTree<T: Ord + Clone> {
    pub root: TRoot<T>,
    num_leaves: u128,
    height: u128,
    _is_num_leaves_available: bool,
    _is_height_available: bool
}

impl<T: Ord + Clone + Debug> RedBlackTree<T> {

    pub fn new() -> RedBlackTree<T> {
        RedBlackTree { 
            root: None, 
            num_leaves: 0, 
            height: 0,
            _is_num_leaves_available: true,
            _is_height_available: true
        }
    }

    pub fn is_empty(&self) -> bool {
        RB::get_root_nil(&self.root)
    }

    fn asc_print(root: &TRoot<T>) {
        let left = RB::get_left(root);
        if !RB::get_root_nil(&left) {
            Self::asc_print(&left);
        }
        print!("{:?} -> ", RB::get_root_key(root));
        let right = RB::get_right(root);
        if !RB::get_root_nil(&right) {
            Self::asc_print(&right);
        }
    }

    fn desc_print(root: &TRoot<T>) {
        let right = RB::get_right(root);
        if !RB::get_root_nil(&right) {
            Self::desc_print(&right);
        }
        print!("{:?} -> ", RB::get_root_key(root));
        let left = RB::get_left(root);
        if !RB::get_root_nil(&left) {
            Self::desc_print(&left);
        }
    }

    pub fn print_traverse(&self, order: Ordering) {
        match order {
            Ordering::Less => {
                Self::asc_print(&self.root);
                println!("done");
            },
            Ordering::Equal => println!("Choose Less or Greater please."),
            Ordering::Greater => {
                Self::desc_print(&self.root);
                println!("done");
            },
        }
    }

    pub fn print_tree(&self) {
        RB::print_tree(&self.root)
    }

    pub fn get_num_leaves(&mut self) -> u128 {
        if self._is_num_leaves_available {
            return self.num_leaves;
        }
        self.num_leaves = RB::count_leaves(&self.root);
        self._is_num_leaves_available = true;
        return self.num_leaves;
    }


    pub fn get_height(&mut self) -> u128 {
        if self._is_height_available {
            return self.height;
        }
        self.height = RB::get_height(&self.root);
        self._is_height_available = true;
        return self.height;
    }


    pub fn transplant(&mut self, u: &TRoot<T>, v: &TRoot<T>) {
        let u_node = u.clone();
        let v_node = v.clone();
        match RB::get_parent(&u_node) {
            Some(_) => {
                if RB::is_node_equal(&u_node, &RB::get_left(&RB::get_parent(&u_node))) {  // If the u_node is a left child.
                    RB::set_child(&RB::get_parent(&u_node), v_node.clone(), Dir::Left);  // Set the left child of u's parent
                    // to be v. In other words, removes u and replaces it with v.
                } else {  // u_node is a right child
                    RB::set_child(&RB::get_parent(&u_node), v_node.clone(), Dir::Right);  // Replaces u with v
                }
            },
            None => self.root = v_node.clone(),
        };
        RB::set_parent(&v_node, &RB::get_parent(&u_node));
    }

    pub fn get_minimum(&self) -> TRoot<T> {
        RB::get_minimum(&self.root)
    }

    fn delete_fixup(&mut self, fix_root: &TRoot<T>, fix_root_parent: Option<&TRoot<T>>) {
        //self.print();
        //RB::print_tree(&fix_root);
        let mut x = fix_root.clone();
        let mut parent = match fix_root_parent {
            Some(p) => p.clone(),
            None => RB::get_parent(&x),
        };
        //println!("{:?}", !RB::is_node_equal(&x, &self.root) && RB::get_root_color(&x) == NC::Black);
        while !RB::is_node_equal(&x, &self.root) && RB::get_root_color(&x) == NC::Black {
            if RB::is_node_equal(&x, &RB::get_left(&parent)) {
                let mut w = RB::get_right(&parent);
                // type 1
                if RB::get_root_color(&w) == NC::Red {
                    RB::set_root_color(&w, NC::Black);
                    RB::set_root_color(&parent, NC::Red);
                    self.root = RB::left_rotate(self.root.clone(), RB::get_root_key(&parent));
                    w = RB::get_right(&parent);
                }
                // type 2
                if RB::get_root_color(&RB::get_left(&w)) == NC::Black
                && RB::get_root_color(&RB::get_right(&w)) == NC::Black {
                    RB::set_root_color(&w, NC::Red);
                    x = parent.clone();
                    parent = RB::get_parent(&x);
                } else {
                    // type 3
                    if RB::get_root_color(&RB::get_right(&w)) == NC::Black {
                        RB::set_root_color(&RB::get_left(&w), NC::Black);
                        RB::set_root_color(&w, NC::Red);
                        self.root = RB::right_rotate(self.root.clone(), RB::get_root_key(&w));
                        w = RB::get_right(&parent);
                    }
                    // type 4
                    RB::set_root_color(&w, RB::get_root_color(&parent));
                    RB::set_root_color(&parent, NC::Black);
                    RB::set_root_color(&RB::get_right(&w), NC::Black);
                    self.root = RB::left_rotate(self.root.clone(), RB::get_root_key(&parent));
                    x = self.root.clone();
                    parent = RB::get_parent(&x);
                }
            } else {
                let mut w = RB::get_left(&parent);
                // type 1
                if RB::get_root_color(&w) == NC::Red {
                    RB::set_root_color(&w, NC::Black);
                    RB::set_root_color(&parent, NC::Red);
                    self.root = RB::right_rotate(self.root.clone(), RB::get_root_key(&parent));
                    w = RB::get_left(&parent);
                }
                // type 2
                if RB::get_root_color(&RB::get_right(&w)) == NC::Black
                && RB::get_root_color(&RB::get_left(&w)) == NC::Black {
                    RB::set_root_color(&w, NC::Red);
                    x = parent.clone();
                    parent = RB::get_parent(&x);
                } else {
                    // type 3
                    if RB::get_root_color(&RB::get_left(&w)) == NC::Black {
                        RB::set_root_color(&RB::get_right(&w), NC::Black);
                        RB::set_root_color(&w, NC::Red);
                        self.root = RB::left_rotate(self.root.clone(), RB::get_root_key(&w));
                        w = RB::get_left(&parent);
                    }
                    // type 4
                    RB::set_root_color(&w, RB::get_root_color(&parent));
                    RB::set_root_color(&parent, NC::Black);
                    RB::set_root_color(&RB::get_left(&w), NC::Black);
                    self.root = RB::right_rotate(self.root.clone(), RB::get_root_key(&parent));
                    x = self.root.clone();
                    parent = RB::get_parent(&x);
                }
            }
        }
        RB::set_root_color(&x, NC::Black);
    }

    pub fn delete(&mut self, key: &T) {
        self._is_num_leaves_available = false;
        self._is_height_available = false;
        let z = RB::find_node(&self.root, key.clone());
        match z {
            None => return,
            _ => (),
        };
        if RB::is_node_equal(&self.root, &z)
        && RB::get_root_nil(&RB::get_left(&self.root))
        && RB::get_root_nil(&RB::get_right(&self.root)) {
            self.root = None;
            return;
        }

        //RB::print_tree(&z);
        RB::solidify_all_nil(&self.root);
        let mut x: TRoot<T> = None;
        let mut y = z.clone();
        let mut y_orig_color = RB::get_root_color(&y);

        if RB::get_root_nil(&RB::get_left(&z)) {
            x = RB::get_right(&z);
            self.transplant(&z, &RB::get_right(&z));
        } else if RB::get_root_nil(&RB::get_right(&z)) {
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
            NC::Black => self.delete_fixup(&x, None),
        };
        RB::virtualize_all_nil(&self.root);
    }

    pub fn insert(&mut self, key: &T) {
        self._is_num_leaves_available = false;
        self._is_height_available = false;
        match &self.root {
            Some(_) => RB::insert_node(&self.root, key.clone()),
            None => self.root = RB::new(key.clone()),
        };
        let mut z = RB::find_node(&self.root, key.clone());

        while RB::get_root_color(&RB::get_parent(&z)) == NC::Red {  // Keep looping if the current node is red
            // The if block checks if the parent of z is a left node
            if RB::is_node_equal(
                &RB::get_parent(&z),
                &RB::get_left(&RB::get_parent(&RB::get_parent(&z)))
            ) {
                /*
                    Here we know that the parent of z is a left node. Get the uncle of z (sibling of z's parent)
                */
                let y = RB::get_right(&RB::get_parent(&RB::get_parent(&z)));
                if RB::get_root_color(&y) == NC::Red {  // If z's uncle is red
                    /*
                        This entire if block just performs node recoloring
                    */
                    // The layer composed of z's parent and uncle becomes black.
                    RB::set_root_color(&RB::get_parent(&z), NC::Black);  // Set z's parent's color to black
                    RB::set_root_color(&y, NC::Black);  // Set z's uncle's color to black
                    // Set z's grandparent to be re
                    RB::set_root_color(&RB::get_parent(&RB::get_parent(&z)), NC::Red);
                    // Set z to be the grandparent
                    z = RB::get_parent(&RB::get_parent(&z));
                } else {  // If z's uncle is black. Here we know that z's parent is a left node.
                    // The if condition checks if z is a right node
                    if RB::is_node_equal(
                        &z,
                        &RB::get_right(&RB::get_parent(&z))
                    ) {
                        z = RB::get_parent(&z);  // Set z to be the current node's parent
                        self.root = RB::left_rotate(self.root.clone(), RB::get_root_key(&z));  // Perform a left rotation at z
                    }
                    RB::set_root_color(&RB::get_parent(&z), NC::Black);  // Set z's parent's color to be black
                    RB::set_root_color(&RB::get_parent(&RB::get_parent(&z)), NC::Red);  // Set z's grandparent's color to be red
                    self.root = RB::right_rotate(self.root.clone(), RB::get_root_key(&RB::get_parent(&RB::get_parent(&z))));
                    // Perform a right rotation at z's grandparent
                }
            } else {
                /*
                    Here we know that the parent of z is a right node. Get the uncle of z (sibling of z's parent)
                */
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