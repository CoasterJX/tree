pub mod tree_test;
pub mod red_black_tree;
pub mod tree_type;
pub mod avl_tree;
pub mod avl_tree_type;

use tree_type::RedBlackTree as RBT;
use avl_tree_type::AVLTree as AVL;
use std::io::{self, Write};
use std::fmt::Debug;
use std::str::FromStr;

const LINE: &str = "-----------------------------------------------------";

fn user_manual() {
    println!("
        insert [key..]                  - insert keys to the tree
        delete [key..]                  - delete keys from the tree
        count-leaves                    - get the number of leaves in the tree
        height                          - get the height of the tree
        is-empty                        - check if the tree is empty
        print                           - print the tree in terminal
        traverse [order: asc/desc]      - print ascending/descending traversing order
    ");
    io::stdout().flush().unwrap();
}

fn welcome() {
    println!("{}", LINE);
    log("ECE 421 Project 2 - Red Black Tree & AVL Tree");
    log("Jianxi Wang, Yihe Wang, John Yu");
    println!("{}", LINE);
}

fn invalid(cmd: &Vec<&str>, size: usize) -> bool {
    if cmd.len() < size {
        println!("Invalid command: {}", cmd.join(" "));
    }
    return cmd.len() < size
}

fn exec_cmd_rb<T: Ord + Clone + Debug + FromStr>(tree: &mut RBT<T>, cmd: &str) where <T as FromStr>::Err: Debug {
    let c = cmd.trim().split(" ").collect::<Vec<&str>>();
    match c[0] {
        "insert" => {
            if invalid(&c, 2) {return;}
            for i in 1..c.len() {
                let key = c[i].parse().unwrap();
                tree.insert(&key);
            }
        },
        "delete" => {
            if invalid(&c, 2) {return;}
            for i in 1..c.len() {
                let key = c[i].parse().unwrap();
                tree.delete(&key);
            }
        },
        "count-leaves" => {
            if invalid(&c, 1) {return;}
            println!("{:?}", tree.get_num_leaves());
        },
        "height" => {
            if invalid(&c, 1) {return;}
            println!("{:?}", tree.get_height());
        },
        "is-empty" => {
            if invalid(&c, 1) {return;}
            println!("{:?}", tree.is_empty());
        },
        "print" => {
            if invalid(&c, 1) {return;}
            tree.print_tree();
        },
        "traverse" => {
            if invalid(&c, 2) {return;}
            match c[1] {
                "asc" => tree.print_traverse(std::cmp::Ordering::Less),
                "desc" => tree.print_traverse(std::cmp::Ordering::Greater),
                _ => println!("Invalid traverse option."),
            };
        },
        _ => println!("Invalid command: {}", cmd),
    }
}

fn exec_cmd_avl<T: Ord + Clone + Debug + FromStr>(tree: &mut AVL<T>, cmd: &str) where <T as FromStr>::Err: Debug {
    let c = cmd.trim().split(" ").collect::<Vec<&str>>();
    match c[0] {
        "insert" => {
            if invalid(&c, 2) {return;}
            for i in 1..c.len() {
                let key = c[i].parse().unwrap();
                tree.insert(&key);
            }
        },
        // "delete" => {
        //     if invalid(&c, 2) {return;}
        //     for i in 1..c.len() {
        //         let key = c[i].parse().unwrap();
        //         tree.delete(&key);
        //     }
        // },
        // "count-leaves" => {
        //     if invalid(&c, 1) {return;}
        //     println!("{:?}", tree.get_num_leaves());
        // },
        // "height" => {
        //     if invalid(&c, 1) {return;}
        //     println!("{:?}", tree.get_height());
        // },
        // "is-empty" => {
        //     if invalid(&c, 1) {return;}
        //     println!("{:?}", tree.is_empty());
        // },
        "print" => {
            if invalid(&c, 1) {return;}
            tree.print_tree();
        },
        // "traverse" => {
        //     if invalid(&c, 2) {return;}
        //     match c[1] {
        //         "asc" => tree.print_traverse(std::cmp::Ordering::Less),
        //         "desc" => tree.print_traverse(std::cmp::Ordering::Greater),
        //         _ => println!("Invalid traverse option."),
        //     };
        // },
        _ => println!("Invalid command: {}", cmd),
    }
}

fn log(str: &str) {
    println!("| {:?}", str);
}

fn start_demo_rb<T: Ord + Clone + Debug + FromStr>(rbt: &mut RBT<T>) where <T as FromStr>::Err: Debug {
    user_manual();
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line).unwrap();
        exec_cmd_rb(rbt, &line);
    }
}

fn start_demo_avl<T: Ord + Clone + Debug + FromStr>(avl: &mut AVL<T>) where <T as FromStr>::Err: Debug {
    user_manual();
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line).unwrap();
        exec_cmd_avl(avl, &line);
    }
}

fn rb_main() {
    let mut key_type = String::new(); 
    print!("Decide the type of tree key (int/str): ");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut key_type).unwrap();
    
    match key_type.trim() {
        "int" => {
            let mut rbt: RBT<i128> = RBT::new();
            start_demo_rb(&mut rbt);
        },
        "str" => {
            let mut rbt: RBT<String> = RBT::new();
            start_demo_rb(&mut rbt);
        }
        _ => println!("Type not implemented in this demo but should work in real case.")
    }
}

fn avl_main() {
    let mut key_type = String::new(); 
    print!("Decide the type of tree key (int/str): ");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut key_type).unwrap();
    
    match key_type.trim() {
        "int" => {
            let mut avl: AVL<i128> = AVL::new();
            start_demo_avl(&mut avl);
        },
        "str" => {
            let mut avl: AVL<String> = AVL::new();
            start_demo_avl(&mut avl);
        }
        _ => println!("Type not implemented in this demo but should work in real case.")
    }
}

fn main() {
    welcome();

    let mut tree_type = String::new();
    print!("Decide the type of tree (1.red-black  2.AVL): ");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut tree_type).unwrap();

    match tree_type.trim() {
        "1" => rb_main(),
        "2" => avl_main(),
        _ => println!("Invalid selection.")
    }
}
