use std::cell::RefCell;
use std::collections::VecDeque;
use std::option::Option;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Node<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<Node<T>>>>,
    pub right: Option<Rc<RefCell<Node<T>>>>,
}

pub struct NodeIterator<T> {
    queue: VecDeque<Option<Rc<RefCell<Node<T>>>>>,
}

impl<T> NodeIterator<T> {
    #[must_use]
    pub fn new(root: Option<Rc<RefCell<Node<T>>>>) -> Self {
        NodeIterator {
            queue: VecDeque::from_iter([root]),
        }
    }
}

impl<T> Iterator for NodeIterator<T> {
    type Item = Option<Rc<RefCell<Node<T>>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.iter().any(Option::is_some) {
            match self.queue.pop_front().flatten() {
                None => Some(None),
                Some(current) => {
                    let borrowed = current.borrow();

                    let left = borrowed.left.clone();
                    let right = borrowed.right.clone();

                    self.queue.push_back(left);
                    self.queue.push_back(right);

                    Some(Some(Rc::clone(&current)))
                },
            }
        } else {
            None
        }
    }
}

impl<T> Node<T> {
    #[inline]
    #[must_use]
    pub fn new(val: T) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

#[must_use]
pub fn vec_eq<T>(left: Vec<T>, mut right: Vec<T>) -> bool
where
    T: std::cmp::Eq,
{
    if left.len() != right.len() {
        return false;
    }

    for l in left {
        if let Some(p) = right.iter().position(|x| x == &l) {
            right.remove(p);
        } else {
            return false;
        }
    }

    true
}

pub fn sort_vec_of_vec<T>(vec: &mut [Vec<T>])
where
    T: std::cmp::Ord,
{
    for inner_v in &mut *vec {
        inner_v.sort_unstable();
    }

    vec.sort_unstable();
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl Side {
    fn flip(self) -> Self {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

/// Converts a slice of &[Option<T>] to a Binary tree
///
/// # Examples
///
/// ```
/// use advent_of_code_2022::shared::tree::to_bt;
/// use advent_of_code_2022::shared::tree::tn;
///
/// let input = [1.into(), None, 3.into()];
/// assert_eq!(to_bt(&input), tn(1, None, tn(3, None, None).into()).into());
/// ```
///
/// # Panics
///
/// Panics if cannot borrow Rc
#[must_use]
pub fn to_bt<T: Copy>(input: &[Option<T>]) -> Option<Rc<RefCell<Node<T>>>> {
    if input.is_empty() {
        return None;
    }

    let root = tn(input[0].unwrap(), None, None);
    let mut queue = VecDeque::from_iter([root.clone()]);

    let mut side = Side::Left;

    for o in input.iter().skip(1) {
        let node = queue.front().unwrap().as_ref().unwrap();

        if let Some(&v) = o.as_ref() {
            let new_node = Some(Rc::new(RefCell::new(Node::new(v))));

            match side {
                Side::Left => {
                    node.borrow_mut().left.clone_from(&new_node);
                },
                Side::Right => {
                    node.borrow_mut().right.clone_from(&new_node);
                },
            }

            queue.push_back(new_node);
        }

        if side == Side::Right {
            queue.pop_front();
        }

        side = side.flip();
    }

    root
}

/// Converts a Binary tree to a flat representation
///
/// # Examples
///
/// ```
/// use advent_of_code_2022::shared::tree::from_bt;
/// use advent_of_code_2022::shared::tree::tn;
///
/// let input = tn(1, None, tn(3, None, None).into()).into();
/// assert_eq!(from_bt(input), [1.into(), None, 3.into()]);
/// ```
///
/// # Panics
///
/// Panics if cannot borrow Rc
#[must_use]
pub fn from_bt<T: Copy>(root: Option<Rc<RefCell<Node<T>>>>) -> Vec<Option<T>> {
    if root.is_none() {
        return vec![];
    }

    let mut results = vec![];
    let mut queue = VecDeque::from_iter([root]);

    while queue.iter().any(Option::is_some) {
        match queue.pop_front().flatten() {
            None => {
                results.push(None);
            },
            Some(current) => {
                let borrow = current.borrow();
                results.push(Some(borrow.val));

                queue.push_back(borrow.left.clone());
                queue.push_back(borrow.right.clone());
            },
        }
    }

    results
}

#[must_use]
pub fn tn<T>(
    val: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
) -> Option<Rc<RefCell<Node<T>>>> {
    Some(Rc::new(RefCell::new(Node { val, left, right })))
}

#[cfg(test)]
mod tests {

    use crate::shared::tree::{NodeIterator, tn, to_bt};

    #[test]
    fn bt() {
        let input = [
            5.into(),
            4.into(),
            7.into(),
            3.into(),
            None,
            2.into(),
            None,
            (-1).into(),
            None,
            9.into(),
        ];

        let expected = tn(
            5,
            tn(4, tn(3, tn(-1, None, None), None), None),
            tn(7, tn(2, tn(9, None, None), None), None),
        );

        assert_eq!(to_bt(&input), expected);
    }

    #[test]
    fn bt_2() {
        let input = (1..=15).map(Some).collect::<Vec<_>>();

        let expected = tn(
            1,
            tn(
                2,
                tn(4, tn(8, None, None), tn(9, None, None)),
                tn(5, tn(10, None, None), tn(11, None, None)),
            ),
            tn(
                3,
                tn(6, tn(12, None, None), tn(13, None, None)),
                tn(7, tn(14, None, None), tn(15, None, None)),
            ),
        );

        assert_eq!(to_bt(&input), expected);
    }

    #[test]
    fn iter() {
        let input = (1..=15).map(Some).collect::<Vec<_>>();

        let tree = to_bt(&input);

        let result = NodeIterator::new(tree)
            .map(|v| v.as_deref().map(|i| i.borrow().val))
            .collect::<Vec<Option<i32>>>();

        assert_eq!(input, result);
    }
}
