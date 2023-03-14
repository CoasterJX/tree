use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::cmp::Ordering;

type TChild<T> = Rc<RefCell<AVLreeNode<T>>>;
type TParent<T> = Weak<RefCell<AVLreeNode<T>>>;
pub type AVLChild<T> = Option<TChild<T>>;
type AVLParent<T> = Option<TParent<T>>;

#[derive(Debug)]
pub struct AVLTreeNode<T: Ord + Clone> {\
    pub key: T,
    pub parent: AVLParent<T>,
    left_child: AVLChild<T>,
    right_child: AVLChild<T>,
    pub _ptr_self: AVLParent<T>
}

impl<T: Ord + Clone + Debug> RBTreeNode<T> {

    pub fn empty() -> RBTChild<T> {
        None
    }


    pub fn new(key: T) -> RBTChild<T> {
    }


    fn _new(key: T, color: NodeColor, parent: RBTParent<T>) -> RBTChild<T> {
        
    }


    fn to_string_nil(direction: &Direction, extra: &str) {
        
    }


    fn to_string(color: &NodeColor, direction: &Direction, parent: &RBTChild<T>, key: &T, extra: &str) {
        
    }


    fn _print_tree(root: &RBTChild<T>, direction: Direction, extra: &str) {

    }


    pub fn print_tree(root: &RBTChild<T>) {
        
    }


    pub fn left_rotate(root: RBTChild<T>, key: T) -> RBTChild<T> {
        
    }


    fn _left_rotate(root: &RBTChild<T>, key: T) -> RBTChild<T> {

    }

    pub fn right_rotate(root: RBTChild<T>, key: T) -> RBTChild<T> {

    }

    pub fn _right_rotate(root: &RBTChild<T>, key: T) -> RBTChild<T> {

    }


    pub fn is_node_equal(root1: &RBTChild<T>, root2: &RBTChild<T>) -> bool {
        
    }


    pub fn get_color(root: &RBTChild<T>, key: T) -> NodeColor {
        
    }


    pub fn get_root_color(root: &RBTChild<T>) -> NodeColor {
        
    }


    pub fn set_root_color(root: &RBTChild<T>, color: NodeColor) {
        
    }


    pub fn get_root_key(root: &RBTChild<T>) -> T {
        
    }


    pub fn get_parent(root: &RBTChild<T>) -> RBTChild<T> {
        
    }


    pub fn get_parent_by_key(root: &RBTChild<T>, key: T) -> RBTChild<T> {
        
    }


    pub fn set_parent(root: &RBTChild<T>, parent: &RBTChild<T>) {
        
    }


    pub fn set_child(root: &RBTChild<T>, child: RBTChild<T>, direction: Direction) {
        
    }


    pub fn get_left(root: &RBTChild<T>) -> RBTChild<T> {
        
    }


    pub fn get_right(root: &RBTChild<T>) -> RBTChild<T> {
        
    }


    pub fn insert_node(root: &RBTChild<T>, key: T) {
        
    }


    pub fn find_node(root: &RBTChild<T>, key: T) -> RBTChild<T> {
        
    }


    fn _recurse_node(root: &RBTChild<T>, key: T, insert: bool) -> RBTChild<T> {

    }
}