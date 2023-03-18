use super::*;
use red_black_tree::{RBTreeNode as RB, NodeColor};
use tree_type::RedBlackTree as RBT;
use avl_tree::{AVLTreeNode as AVL};
use avl_tree_type::AVLTree as AVLT;
use rand::Rng;
use std::{fmt::Debug, cmp::Ordering};

#[test]
fn test_rb_rotate() {
    let root = RB::new(5);
    for i in [5, 2, 10, 8, 6, 9, 12] {
        RB::insert_node(&root, i);
    }
    RB::print_tree(&root);
    
    // perform right rotate at 10
    let root = RB::left_rotate(root, 12);
    RB::print_tree(&root);
}

#[test]
fn test_avl_left_rotate() {
    let root = AVL::new(5);
    for i in [5, 2, 10, 8, 6, 9, 12] {
        AVL::insert_node(&root, i);
    }
    AVL::print_tree(&root);

    // perform right rotate at 10
    let root = AVL::left_rotate(root, 10);
    AVL::print_tree(&root);
}

#[test]
fn test_avl_right_rotate() {
    let root = AVL::new(5);
    for i in [5, 2, 10, 8, 6, 9, 12] {
        AVL::insert_node(&root, i);
    }
    AVL::print_tree(&root);
}

#[test]
fn test_avl_set_root_key() {
    let root = AVL::new(15);
    for i in [12, 19, 8, 23] {
        AVL::insert_node(&root, i);
    }
    AVL::print_tree(&root);
    AVL::set_root_key(&AVL::find_node(&root, 15), 16);
    AVL::print_tree(&root);
}

#[test]
fn test_avl_node_get_height() {
    let root = AVL::new(15);
    assert_eq!(1, AVL::get_height(&root));
}

#[test]
fn test_avl_node_set_height() {
    let root = AVL::new(15);
    AVL::set_height(&root, 3);
    assert_eq!(3, AVL::get_height(&root));
}

#[test]
fn test_avl_node_update_height() {
    let root = AVL::new(15);
    assert_eq!(1, AVL::get_height(&root));
}


#[test]
fn test_rb_insert() {
    let mut rbt = RBT::<u64>::new();
    for i in [5, 2, 10, 8, 6, 9, 12, 13] {
        rbt.insert(&i);
    }
    rbt.print_tree();
}

#[test]
fn test_avl_insert() {
    let mut avl = AVLT::<u64>::new();
    for i in [5, 2, 10, 8, 6, 9, 12, 13] {
        avl.insert(&i);
    }
    avl.print_tree();
}

#[test]
fn test_rb_delete1() {
    let mut rbt = RBT::<u64>::new();
    println!("{:?}", rbt.is_empty());
    for i in [12, 8, 15, 5, 9, 13, 19, 10, 23] {
        rbt.insert(&i);
    }
    //RB::solidify_all_nil(&rbt.root);
    //RB::virtualize_all_nil(&rbt.root2;
    //rbt.delete(&12);
    //rbt.delete(&10);
    // rbt.print_tree();
    // println!("{:?}", rbt.is_empty());
    rbt.print_traverse(std::cmp::Ordering::Equal);
}

#[test]
fn test_pressure() {
    let mut rbt = RBT::<u64>::new();
    let mut rng = rand::thread_rng();
    let mut inserted = vec![];
    for _ in 0..10000 {
        let a = rng.gen_range(0..100000);
        rbt.insert(&a);
        inserted.append(&mut vec![a]);
    }
    for i in inserted {
        rbt.delete(&i);
    }
    rbt.print_tree();
}

#[test]
fn test_rb_delete2() {
    let mut rbt = RBT::<u64>::new();
    for i in [12, 8, 15, 5, 9, 13, 23, 1, 10] {
        rbt.insert(&i);
    }
    rbt.delete(&5);
    rbt.print_tree();
}

#[test]
fn test_rb_delete3() {
    let mut rbt = RBT::<u64>::new();
    for i in [12, 8, 15, 1, 9, 13, 23, 10] {
        rbt.insert(&i);
    }
    rbt.delete(&12);
    rbt.print_tree();
}

#[test]
fn test_rb_transplant1() {
    let mut rbt = RBT::<u64>::new();
    for i in [15, 12, 19, 9, 13, 23] {
        rbt.insert(&i);
    }
    rbt.transplant(
        &RB::find_node(&rbt.root, 15),
        &RB::find_node(&rbt.root, 19)
    );
    rbt.print_tree();
    //RB::print_tree(&rbt.get_minimum());
}

#[test]
fn test_rb_transplant2() {
    let mut rbt = RBT::<u64>::new();
    for i in [15, 12, 19, 13, 23] {
        rbt.insert(&i);
    }
    rbt.transplant(
        &RB::find_node(&rbt.root, 12),
        &RB::find_node(&rbt.root, 13)
    );
    rbt.print_tree();
}

#[test]
fn test_rb_transplant3() {
    let mut rbt = RBT::<u64>::new();
    for i in [15, 12, 19, 8, 23] {
        rbt.insert(&i);
    }
    rbt.transplant(
        &RB::find_node(&rbt.root, 19),
        &RB::find_node(&rbt.root, 23)
    );
    rbt.print_tree();
}

#[test]
fn test_rb_transplant4() {
    let mut rbt = RBT::<u64>::new();
    for i in [15, 12, 19, 8, 23] {
        rbt.insert(&i);
    }
    rbt.transplant(
        &RB::find_node(&rbt.root, 23),
        &None
    );
    rbt.print_tree();
}

#[test]
fn test_avl_delete_case1() {
    /*
        Case 1 is when the node being deleted is a leaf node.
    */
    let mut avl = AVLT::<u64>::new();
    for i in [11, 31, 18, 2, 1, 33, 28, 4, 3, 7, 5, 25, 20, 19, 15, 12, 14, 29] {
        avl.insert(&i);
    }
    avl.print_tree();
    avl.delete(&19);
    avl.print_tree();
}

#[test]
fn test_avl_delete_case2() {
    /*
        Case 2 is when the node being deleted has a right subtree but no left subtree.
    */
    let mut avl = AVLT::<u64>::new();
    for i in [11, 31, 18, 2, 1, 33, 28, 4, 3, 7, 5, 25, 20, 19, 15, 12, 14, 29] {
        avl.insert(&i);
    }
    avl.print_tree();
    avl.delete(&28);
    avl.print_tree();
}

#[test]
fn test_avl_delete_case3() {
    /*
        Case 3 is when the node being deleted has a left subtree but no right subtree.
    */
    let mut avl = AVLT::<u64>::new();
    for i in [11, 31, 18, 2, 1, 33, 28, 4, 3, 7, 5, 25, 20, 19, 15, 12, 14, 29] {
        avl.insert(&i);
    }
    avl.print_tree();
    avl.delete(&20);
    avl.print_tree();
}

#[test]
fn test_avl_delete_case4_min_is_leaf() {
    /*
        Case 4 is when the node to be deleted has both a left and right subtree. The way the delete implementation
        works is that it replaces the current node with the smallest node in the right subtree. The smallest node in the
        right subtree will be deleted instead. To delete the smallest node in the right subtree there are two cases
        to consider. This test case considers the case when the smallest node in the right subtree is a leaf node.
    */
    let mut avl = AVLT::<u64>::new();
    for i in [11, 31, 18, 2, 1, 33, 28, 4, 3, 7, 5, 25, 20, 19, 15, 12, 14, 29] {
        avl.insert(&i);
    }
    avl.print_tree();
    avl.delete(&18);
    avl.print_tree();
}

#[test]
fn test_avl_delete_case4_min_has_right_tree() {
    /*
        Case 4 is when the node to be deleted has both a left and right subtree. The way the delete implementation
        works is that it replaces the current node with the smallest node in the right subtree. The smallest node in the
        right subtree will be deleted instead. To delete the smallest node in the right subtree there are two cases
        to consider. This test case considers the case when the smallest node in the right subtree has a right subtree.
    */
    let mut avl = AVLT::<u64>::new();
    for i in [11, 31, 18, 2, 1, 33, 28, 4, 3, 7, 5, 25, 20, 19, 15, 12, 14, 29] {
        avl.insert(&i);
    }
    avl.print_tree();
    avl.delete(&25);
    avl.print_tree();
}

#[test]
fn test_avl_delete_only_root_left() {
    let mut avl = AVLT::<u64>::new();
    avl.insert(&1);
    avl.print_tree();
    avl.delete(&1);
    avl.print_tree();
}

#[test]
fn test_avl_delete_empty() {
    let mut avl = AVLT::<u64>::new();
    avl.delete(&1);
}

#[test]
fn test_avl_count_leaves() {
    let mut avl = AVLT::<u64>::new();
    for i in [15, 12, 19, 8, 23] {
        avl.insert(&i);
    }
    assert_eq!(2, avl.get_num_leaves());
}

#[test]
fn test_avl_get_height() {
    let mut avl = AVLT::<u64>::new();
    for i in [15, 12, 19, 8, 23] {
        avl.insert(&i);
    }
    assert_eq!(3, avl.get_height());
}

#[test]
fn test_avl_print_traverse() {
    let mut avl = AVLT::<u64>::new();
    for i in [15, 12, 19, 8, 23] {
        avl.insert(&i);
    }
    avl.print_traverse(Ordering::Greater);
    avl.print_traverse(Ordering::Less);
}

#[test]
fn test_avl_check_empty() {
    let mut avl = AVLT::<u64>::new();
    assert_eq!(true, avl.is_empty());

    for i in [15, 12, 19, 8, 23] {
        avl.insert(&i);
    }
    assert_eq!(false, avl.is_empty());
}

// #[test]
// fn test_rb_insert_find() {
//     let root = RBTreeNode::new(5, NodeColor::Black, None);
//     let root_cp = RBTreeNode::find_node(&root, 5);
//     //let root = RBTreeNode::empty();
//     for i in [5, 3, 2, 4, 7, 6, 9, 5] {
//         RBTreeNode::insert_node(&root, i);
//     }
//     // println!("{:#?}", root);
//     //RBTreeNode::print_tree(&root);

//     let root = RBTreeNode::left_rotate(root, 5);
//     RBTreeNode::print_tree(&root);
//     return;
//     RBTreeNode::print_tree(&RBTreeNode::get_parent(&root));
//     println!("{:#?}", RBTreeNode::get_parent(&root));

//     let parent = RBTreeNode::get_parent(&root);
//     RBTreeNode::set_child(&parent, root, red_black_tree::Direction::Right);
//     println!("{:#?}", parent);

//     // let subtree_7 = RBTreeNode::find_node(&root, 7);
//     // //println!("{:#?}", subtree_7);

//     // let nothing_8 = RBTreeNode::find_node(&root, 8);
//     // println!("{:#?}", nothing_8);

//     // let parent_6 = RBTreeNode::get_parent_by_key(&root, 6);
//     // println!("{:#?}", parent_6);

//     // RBTreeNode::set_parent(&subtree_7, &RBTreeNode::find_node(&root, 4));
//     //let new_parent = RBTreeNode::get_parent(&subtree_7);
//     //println!("{:#?}", root);
//     // RBTreeNode::print_tree(&root);
// }

// #[test]
// fn test_rb_set_child() {
//     let root = RBTreeNode::new(2, NodeColor::Black, None);
//     //let root = RBTreeNode::empty();
//     for i in [1, 3] {
//         RBTreeNode::insert_node(&root, i);
//     }

//     let root2 = RBTreeNode::new(2, NodeColor::Black, None);
//     //let root = RBTreeNode::empty();
//     for i in [1, 3] {
//         RBTreeNode::insert_node(&root2, i);
//     }

//     RBTreeNode::set_child(&root, root2, red_black_tree::Direction::Right);
//     println!("{:#?}", root);
// }
