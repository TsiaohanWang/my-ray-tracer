#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct BiTree<T> {
    parent: T,
    lchild: Box<BiTree<T>>,
    rchild: Box<BiTree<T>>,
}

