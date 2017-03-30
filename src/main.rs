use std::cmp::Ordering;
use std::cmp::max;

#[derive(Clone,Debug)]
enum BinaryTree<T> where T: Ord {
    INode(Box<BinaryTree<T>>,T,Box<BinaryTree<T>>),
    Leaf(T),
    None,
}

fn leaves<T: Ord>(bt : &BinaryTree<T>) -> usize {
    match *bt {
        BinaryTree::INode(ref l,_,ref r) => leaves(l) + leaves(r),
        BinaryTree::Leaf(_) => 1,
        BinaryTree::None => 0,
    }
}

fn edges<T: Ord>(bt : &BinaryTree<T>) -> usize {
    match *bt {
        BinaryTree::INode(ref l,_,ref r) => {
            (match **l { BinaryTree::None => 0, _ => edges(l) + 1}) +
            (match **r { BinaryTree::None => 0, _ => edges(r) + 1})
        },
        _ => 0,
    }
}

fn height<T: Ord>(bt : &BinaryTree<T>) -> usize {
    match *bt {
        BinaryTree::INode(ref l,_,ref r) => 1 + max(height(l), height(r)),
        BinaryTree::Leaf(_) => 1,
        BinaryTree::None => 0,
    }
}

fn nodes<T: Ord>(bt : &BinaryTree<T>) -> usize {
    match *bt {
        BinaryTree::INode(ref l,_,ref r) => 1 + nodes(l) + nodes(r),
        BinaryTree::Leaf(_) => 1,
        BinaryTree::None => 0,
    }
}

fn main() {
    let mut bt : BinaryTree<i32> = BinaryTree::None;
    bt.insert(320);
    bt.insert(320);
    bt.insert(319);
    bt.insert(2);
    bt.insert(5);
    bt.insert(7);
    bt.insert(420);
    println!("{:?}", bt);
    println!("Nodes: {}", nodes(&bt));
    println!("Leaves: {}", leaves(&bt));
    println!("Edges: {}", edges(&bt));
    println!("Height: {}", height(&bt));
}

trait Bag<T> {
    fn insert(&mut self, val: T);
}

impl<T> Bag<T> for BinaryTree<T> where T: Ord + Clone {
    fn insert(&mut self, val: T) {
        let update = match *self {
            BinaryTree::INode(ref mut l, ref d, ref mut r) => {
                match d.cmp(&val) {
                    Ordering::Less => {r.insert(val)},
                    Ordering::Greater => {l.insert(val)},
                    _ => {}
                };
                None
            },
            BinaryTree::Leaf(ref d) => {
                if *d != val {
                    let mut new = BinaryTree::INode(Box::new(BinaryTree::None), d.clone(), Box::new(BinaryTree::None));
                    new.insert(val);
                    Some(new)
                } else {
                    None
                }
            },
            BinaryTree::None => Some(BinaryTree::Leaf(val)),
        };
        if let Some(v) = update { 
            *self = v;
        }
    }
}

