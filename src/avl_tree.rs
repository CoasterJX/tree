use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::cmp::{Ordering, max};

pub enum Direction {
    Left,
    Right,
}

type TChild<T> = Rc<RefCell<AVLTreeNode<T>>>;
type TParent<T> = Weak<RefCell<AVLTreeNode<T>>>;
pub type AVLChild<T> = Option<TChild<T>>;
type AVLParent<T> = Option<TParent<T>>;

#[derive(Debug)]
pub struct AVLTreeNode<T: Ord + Clone> {
    pub key: T,
    pub parent: AVLParent<T>,
    left_child: AVLChild<T>,
    right_child: AVLChild<T>,
    pub _ptr_self: AVLParent<T>,
    pub is_nil: bool,
    pub height: u128,
}

impl<T: Ord + Clone + Debug> AVLTreeNode<T> {

    pub fn empty() -> AVLChild<T> {
        None
    }


    pub fn new(key: T) -> AVLChild<T> {
        AVLTreeNode::_new(key, None, false)
    }

    pub fn count_leaves(root: &AVLChild<T>) -> u128 {
        if AVLTreeNode::is_leaf(root) { return 1; }
        if AVLTreeNode::get_root_nil(root) { return 0; }
        //if RBTreeNode::is_leaf(root) {return 1;}
        AVLTreeNode::count_leaves(&AVLTreeNode::get_left(root)) + AVLTreeNode::count_leaves(&AVLTreeNode::get_right(root))
    }


    fn _new(key: T, parent: AVLParent<T>, is_nil: bool) -> AVLChild<T> {
        let node = Rc::new(RefCell::new(Self { 
            key, 
            parent, 
            left_child: None, 
            right_child: None,
            _ptr_self: None,
            is_nil,
            height: 1
        }));

        let weak_ptr = Rc::downgrade(&node);
        {
            let mut node_ref = node.borrow_mut();
            node_ref._ptr_self = Some(weak_ptr);
        }

        Some(node)
    }

    pub fn get_height(root: &AVLChild<T>) -> u128 {
        if AVLTreeNode::get_root_nil(root) {return 0;}
        //1 + max(
            //AVLTreeNode::get_height(&AVLTreeNode::get_left(root)),
            //AVLTreeNode::get_height(&AVLTreeNode::get_right(root))
        //)
        match root {
            Some(root_ptr) => root_ptr.borrow().height,
            None => todo!("should never happen"),
        } 
    }

    pub fn set_height(root: &AVLChild<T>, height: u128) {
        match root {
            Some(root_ptr) => {
                let mut node_ref = root_ptr.borrow_mut();
                //match parent {
                    //Some(parent_ptr) => node_ref.parent = parent_ptr.borrow()._ptr_self.clone(),
                    //None => node_ref.parent = None,
                //}
                node_ref.height = height.clone();
            },
            None => {},
        }
    }

    pub fn update_height(root: &AVLChild<T>) {
        AVLTreeNode::set_height(root, 1 + max(
            AVLTreeNode::get_height(&AVLTreeNode::get_left(root)),
            AVLTreeNode::get_height(&AVLTreeNode::get_right(root))
        ))
    }

    pub fn get_balance_factor(root: &AVLChild<T>) -> i64 {
        if AVLTreeNode::get_root_nil(root) {return 0;}
        let lh: i64 = AVLTreeNode::get_height(&AVLTreeNode::get_left(root)) as i64;
        let rh: i64 = AVLTreeNode::get_height(&AVLTreeNode::get_right(root)) as i64;
        let balance_factor: i64 = rh - lh;
        balance_factor
    }


    fn to_string_nil(direction: &Direction, extra: &str) {
        let direction_str = match direction {
            Direction::Left => "<──",
            Direction::Right => "──>",
        };
        println!("{}{}NIL", extra, direction_str);
    }


    fn to_string(direction: &Direction, parent: &AVLChild<T>, key: &T, extra: &str) {
        let direction_str = match direction {
            Direction::Left => "<──",
            Direction::Right => "──>",
        };
        let parent_key = if let Some(parent_ptr) = parent {
            parent_ptr.borrow().key.clone()
        } else {key.clone()};

        println!("{}", extra);
        println!(
            "{}{}(key {:?}, parent {:?})", 
            extra, direction_str, key, parent_key);
    }


    fn _print_tree(root: &AVLChild<T>, direction: Direction, extra: &str) {
        match root {
            Some(tree_ptr) => {
                let node_ref = tree_ptr.borrow();
                AVLTreeNode::to_string(
                    &direction, 
                    &AVLTreeNode::get_parent(&root),
                    &node_ref.key,
                    extra);
                
                let (left_child, right_child) = (&node_ref.left_child, &node_ref.right_child);
                match left_child {
                    Some(_) => AVLTreeNode::_print_tree(left_child, Direction::Left, &(extra.to_owned()+"|\t")),
                    None => AVLTreeNode::<T>::to_string_nil(&Direction::Left, &(extra.to_owned()+"|\t")),
                };
                match right_child {
                    Some(_) => AVLTreeNode::_print_tree(right_child, Direction::Right, &(extra.to_owned()+"|\t")),
                    None => AVLTreeNode::<T>::to_string_nil(&Direction::Right, &(extra.to_owned()+"|\t")),
                };
            },
            None => AVLTreeNode::<T>::to_string_nil(&Direction::Left, ""),
        }
    }


    pub fn print_tree(root: &AVLChild<T>) {
        AVLTreeNode::_print_tree(root, Direction::Left, "");
    }


    pub fn left_rotate(root: AVLChild<T>, key: T) -> AVLChild<T> {
        let root_copy = root.clone();
        let x = AVLTreeNode::find_node(&root, key.clone());
        let parent = AVLTreeNode::get_parent(&x);
        match &parent {
            Some(_) => {
                if AVLTreeNode::is_node_equal(&x, &AVLTreeNode::get_left(&parent)) {
                    AVLTreeNode::set_child(
                        &parent, 
                        AVLTreeNode::_left_rotate(&root, key), 
                        Direction::Left
                    );
                } else {
                    AVLTreeNode::set_child(
                        &parent, 
                        AVLTreeNode::_left_rotate(&root, key), 
                        Direction::Right
                    );
                }
                return root_copy;
            },
            None => {
                return AVLTreeNode::_left_rotate(&root, key)
            },
        }
    }


    fn _left_rotate(root: &AVLChild<T>, key: T) -> AVLChild<T> {
        match root {
            Some(_) => {
                let x = AVLTreeNode::find_node(root, key);
                let y = AVLTreeNode::get_right(&x);
                match y {
                    None => return root.clone(),
                    _ => (),
                }
                
                AVLTreeNode::set_child(&x, AVLTreeNode::get_left(&y), Direction::Right);

                match AVLTreeNode::get_left(&y) {
                    Some(_) => AVLTreeNode::set_parent(&AVLTreeNode::get_left(&y), &x),
                    None => (),
                };
                AVLTreeNode::set_parent(&y, &AVLTreeNode::get_parent(&x));

                AVLTreeNode::set_parent(&x, &y);
                let z = x.clone();
                AVLTreeNode::set_child(&y, z, Direction::Left);

                AVLTreeNode::update_height(&x);
                AVLTreeNode::update_height(&y);

                return y; // this y must be used to set the parent's left or right

            },
            None => todo!("Does not support empty root now"),
        }
    }

    pub fn right_rotate(root: AVLChild<T>, key: T) -> AVLChild<T> {
        let root_copy = root.clone();
        let x = AVLTreeNode::find_node(&root, key.clone());
        let parent = AVLTreeNode::get_parent(&x);
        match &parent {
            Some(_) => {
                if AVLTreeNode::is_node_equal(&x, &AVLTreeNode::get_right(&parent)) {
                    AVLTreeNode::set_child(
                        &parent, 
                        AVLTreeNode::_right_rotate(&root, key), 
                        Direction::Right
                    );
                } else {
                    AVLTreeNode::set_child(
                        &parent, 
                        AVLTreeNode::_right_rotate(&root, key), 
                        Direction::Left
                    );
                }
                return root_copy;
            },
            None => {
                return AVLTreeNode::_right_rotate(&root, key)
            },
        }
    }

    pub fn _right_rotate(root: &AVLChild<T>, key: T) -> AVLChild<T> {
        match root {
            Some(_) => {
                let x = AVLTreeNode::find_node(root, key);
                let y = AVLTreeNode::get_left(&x);
                match y {
                    None => return root.clone(),
                    _ => (),
                }
                
                AVLTreeNode::set_child(&x, AVLTreeNode::get_right(&y), Direction::Left);

                match AVLTreeNode::get_right(&y) {
                    Some(_) => AVLTreeNode::set_parent(&AVLTreeNode::get_right(&y), &x),
                    None => (),
                };
                AVLTreeNode::set_parent(&y, &AVLTreeNode::get_parent(&x));

                AVLTreeNode::set_parent(&x, &y);
                let z = x.clone();
                AVLTreeNode::set_child(&y, z, Direction::Right);

                AVLTreeNode::update_height(&x);
                AVLTreeNode::update_height(&y);

                return y; // this y must be used to set the parent's left or right

            },
            None => todo!("Does not support empty root now"),
        }
    }


    pub fn is_node_equal(root1: &AVLChild<T>, root2: &AVLChild<T>) -> bool {
        match (root1, root2) {
            (None, None) => true,
            (Some(ptr1), Some(ptr2)) => {
                match ptr1.borrow().key.cmp(&ptr2.borrow().key) {
                    Ordering::Equal => true,
                    _ => false,
                }
            },
            _ => false
        }
    }


    pub fn get_root_key(root: &AVLChild<T>) -> T {
        match root {
            Some(root_ptr) => root_ptr.borrow().key.clone(),
            None => todo!("should never happen"),
        }
    }

    pub fn set_root_key(root: &AVLChild<T>, key: T) {
        match root {
            Some(root_ptr) => {
                let mut node_ref = root_ptr.borrow_mut();
                node_ref.key = key;
            },
            None => todo!("should never happen"),
        }
    }

    pub fn get_root_nil(root: &AVLChild<T>) -> bool {
        match root {
            Some(target_ptr) => target_ptr.borrow().is_nil.clone(),
            None => true,
        }
    }


    pub fn get_parent(root: &AVLChild<T>) -> AVLChild<T> {
        match root {
            Some(tree_ptr) => {
                let node_ref = tree_ptr.borrow();
                if let Some(parent_ptr) = node_ref.parent.clone() {
                    return parent_ptr.upgrade();
                } else {return None;}
            }
            None => None,
        }
    }

    pub fn is_leaf(root: &AVLChild<T>) -> bool {
        match root {
            Some(tree_ptr) => {
                let node_ref = tree_ptr.borrow();
                match (&node_ref.left_child, &node_ref.right_child) {
                    (None, None) => return true,
                    _ => return false,
                };
            },
            None => false,
        }
    }

    pub fn get_parent_by_key(root: &AVLChild<T>, key: T) -> AVLChild<T> {
        AVLTreeNode::get_parent(&AVLTreeNode::find_node(root, key))
    }


    pub fn set_parent(root: &AVLChild<T>, parent: &AVLChild<T>) {
        match root {
            Some(tree_ptr) => {
                let mut node_ref = tree_ptr.borrow_mut();
                match parent {
                    Some(parent_ptr) => node_ref.parent = parent_ptr.borrow()._ptr_self.clone(),
                    None => node_ref.parent = None,
                }
            },
            None => todo!("not supported"),
        }
    }


    pub fn set_child(root: &AVLChild<T>, child: AVLChild<T>, direction: Direction) {
        match root {
            Some(tree_ptr) => {
                let mut node_ref = tree_ptr.borrow_mut();
                match direction {
                    Direction::Left => node_ref.left_child = child,
                    Direction::Right => node_ref.right_child = child,
                }
            },
            None => todo!("not supported"),
        }
    }

    pub fn set_child_nil(root: &AVLChild<T>, direction: Direction) {
        match root {
            Some(_) => {
                let nil_node = AVLTreeNode::_new(AVLTreeNode::get_root_key(root).clone(), None, true);
                match direction {
                    Direction::Left => {
                        AVLTreeNode::set_child(root, nil_node, direction);
                        AVLTreeNode::set_parent(&AVLTreeNode::get_left(root), root);
                    },
                    Direction::Right => {
                        AVLTreeNode::set_child(root, nil_node, direction);
                        AVLTreeNode::set_parent(&AVLTreeNode::get_right(root), root);
                    },
                }
            },
            None => todo!("not supported"),
        }
    }

    pub fn solidify_all_nil(root: &AVLChild<T>) {
        if let None = AVLTreeNode::get_left(root) {
            AVLTreeNode::set_child_nil(root, Direction::Left);
        } else {
            AVLTreeNode::solidify_all_nil(&AVLTreeNode::get_left(root));
        };
        if let None = AVLTreeNode::get_right(root) {
            AVLTreeNode::set_child_nil(root, Direction::Right);
        } else {
            AVLTreeNode::solidify_all_nil(&AVLTreeNode::get_right(root));
        };
    }

    pub fn virtualize_all_nil(root: &AVLChild<T>) {
        if AVLTreeNode::get_root_nil(&AVLTreeNode::get_left(root)) {
            AVLTreeNode::set_child(root, None, Direction::Left);
        } else {
            AVLTreeNode::virtualize_all_nil(&AVLTreeNode::get_left(root));
        };
        if AVLTreeNode::get_root_nil(&AVLTreeNode::get_right(root)) {
            AVLTreeNode::set_child(root, None, Direction::Right);
        } else {
            AVLTreeNode::virtualize_all_nil(&AVLTreeNode::get_right(root));
        };
    }

    pub fn get_left(root: &AVLChild<T>) -> AVLChild<T> {
        match root {
            Some(tree_ptr) => {
                let node_ref = tree_ptr.borrow();
                if let Some(left) = &node_ref.left_child {
                    return Some(left.clone());
                } else {return None;}
            },
            None => None,
        }
    }

    pub fn get_minimum(root: &AVLChild<T>) -> AVLChild<T> {
        match root {
            Some(tree_ptr) => {
                let node_ref = tree_ptr.borrow();
                match &node_ref.left_child {
                    Some(_) => {
                        if AVLTreeNode::get_root_nil(&node_ref.left_child) {
                            return root.clone();
                        }
                        return AVLTreeNode::get_minimum(&node_ref.left_child)
                    },
                    None => return root.clone(),
                }
            },
            None => todo!("not supported"),
        }
    }

    pub fn get_right(root: &AVLChild<T>) -> AVLChild<T> {
        match root {
            Some(tree_ptr) => {
                let node_ref = tree_ptr.borrow();
                if let Some(left) = &node_ref.right_child {
                    return Some(left.clone());
                } else {return None;}
            },
            None => None,
        }
    }


    pub fn insert_node(root: &AVLChild<T>, key: T) {
        let _ = AVLTreeNode::_recurse_node(root, key, true);
    }


    pub fn find_node(root: &AVLChild<T>, key: T) -> AVLChild<T> {
        AVLTreeNode::_recurse_node(root, key, false)
    }


    fn _recurse_node(root: &AVLChild<T>, key: T, insert: bool) -> AVLChild<T> {
        match root {
            Some(tree_ptr) => {
                
                let mut node_ref = tree_ptr.borrow_mut();

                match key.cmp(&node_ref.key) {

                    Ordering::Less => {
                        match node_ref.left_child {
                            Some(_) => AVLTreeNode::_recurse_node(&node_ref.left_child, key, insert).clone(),
                            None => {
                                if insert {
                                    node_ref.left_child = AVLTreeNode::_new(key, node_ref._ptr_self.clone(), false);
                                    if let Some(insert_ptr) = &node_ref.left_child {
                                        return Some(insert_ptr.clone());
                                    } else {todo!("should never reach here")}

                                } else {return None;}
                            },
                        }
                    },
                    Ordering::Equal => return Some(tree_ptr.clone()),
                    Ordering::Greater => {
                        match node_ref.right_child {
                            Some(_) => AVLTreeNode::_recurse_node(&node_ref.right_child, key, insert).clone(),
                            None => {
                                if insert {
                                    node_ref.right_child = AVLTreeNode::_new(key, node_ref._ptr_self.clone(), false);
                                    if let Some(insert_ptr) = &node_ref.right_child {
                                        return Some(insert_ptr.clone());
                                    } else {todo!("should never reach here")}
                                } else {return None;}
                            },
                        }
                    },
                }
            },
            None => {return None;},
        }
    }
}