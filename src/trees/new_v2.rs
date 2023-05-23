use tap::Pipe;

#[derive(Debug)]
pub struct Node {
    pub size: usize,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

macro_rules! fn_set {
    ($($name:ident => $set:ident: $ty:ty)*) => {
        $(
            fn $name(slice: &mut [Self::Item], idx: usize, $set: $ty) {
                Self::_get(slice, idx).map(|node| Self::_set(slice, idx, Node { $set, ..node }));
            }
        )*
    };
}

pub trait Tree {
    type Item;

    fn get(item: &Self::Item) -> Node;
    fn set(item: &mut Self::Item, val: Node);

    fn _get(slice: &[Self::Item], idx: usize) -> Option<Node> {
        slice.get(idx).map(Self::get)
    }

    fn _set(slice: &mut [Self::Item], idx: usize, node: Node) {
        if let Some(val) = slice.get_mut(idx) {
            Self::set(val, node)
        }
    }

    fn size(slice: &[Self::Item], idx: usize) -> Option<usize> {
        Self::_get(slice, idx)?.size.pipe(Some)
    }

    fn left(slice: &[Self::Item], idx: usize) -> Option<usize> {
        Self::_get(slice, idx)?.left
    }

    fn right(slice: &[Self::Item], idx: usize) -> Option<usize> {
        Self::_get(slice, idx)?.right
    }

    fn_set! {
        set_size => size: usize
        set_left => left: Option<usize>
        set_right => right: Option<usize>
    }

    fn is_left_of(slice: &[Self::Item], first: usize, second: usize) -> bool;
    fn is_right_of(slice: &[Self::Item], first: usize, second: usize) -> bool {
        first != second && !Self::is_left_of(slice, first, second)
    }

    fn left_size(slice: &[Self::Item], idx: usize) -> Option<usize> {
        Self::left(slice, idx).and_then(|idx| Self::size(slice, idx))
    }

    fn right_size(slice: &[Self::Item], idx: usize) -> Option<usize> {
        Self::right(slice, idx).and_then(|idx| Self::size(slice, idx))
    }

    fn rightest(slice: &[Self::Item], mut current: usize) -> usize {
        while let Some(next) = Self::right(slice, current) {
            current = next;
        }
        current
    }

    fn leftest(slice: &[Self::Item], mut current: usize) -> usize {
        while let Some(next) = Self::left(slice, current) {
            current = next;
        }
        current
    }

    fn next(slice: &[Self::Item], idx: usize) -> Option<usize> {
        Self::right(slice, idx).map(|idx| Self::leftest(slice, idx))
    }

    fn prev(slice: &[Self::Item], idx: usize) -> Option<usize> {
        Self::left(slice, idx).map(|idx| Self::rightest(slice, idx))
    }

    fn is_contains(slice: &[Self::Item], mut root: usize, idx: usize) -> bool {
        loop {
            // match (
            //     Self::is_left_of(slice, idx, root),
            //     Self::is_right_of(slice, idx, root),
            // ) {
            //     (true, _) => root = tri!(Self::left(slice, root)),
            //     (_, true) => root = tri!(Self::right(slice, root)),
            //     (_, _) => return false,
            // }
            if Self::is_left_of(slice, idx, root) {
                root = tri!(Self::left(slice, root));
            } else if Self::is_right_of(slice, idx, root) {
                root = tri!(Self::right(slice, root));
            } else {
                return true;
            }
        }
    }

    fn inc_size(slice: &mut [Self::Item], idx: usize) {
        if let Some(size) = Self::size(slice, idx) {
            Self::set_size(slice, idx, size + 1)
        }
    }

    fn dec_size(slice: &mut [Self::Item], idx: usize) {
        if let Some(size) = Self::size(slice, idx) {
            Self::set_size(slice, idx, size - 1)
        }
    }

    fn fix_size(slice: &mut [Self::Item], idx: usize) {
        Self::set_size(
            slice,
            idx,
            Self::left_size(slice, idx).unwrap_or_default()
                + Self::right_size(slice, idx).unwrap_or_default()
                + 1,
        )
    }

    fn clear(slice: &mut [Self::Item], idx: usize) {
        Self::_set(slice, idx, Node { size: 0, left: None, right: None })
    }

    #[must_use]
    fn rotate_left(slice: &mut [Self::Item], root: usize) -> Option<usize> {
        let right = Self::right(slice, root)?;
        Self::set_right(slice, root, Self::left(slice, right));
        Self::set_left(slice, right, Some(root));
        Self::set_size(slice, right, Self::size(slice, root)?);
        Self::fix_size(slice, root);
        Some(right)
    }

    #[must_use]
    fn rotate_right(slice: &mut [Self::Item], root: usize) -> Option<usize> {
        let left = Self::left(slice, root)?;
        Self::set_left(slice, root, Self::right(slice, left));
        Self::set_right(slice, left, Some(root));
        Self::set_size(slice, left, Self::size(slice, root)?);
        Self::fix_size(slice, root);
        Some(left)
    }
}

fn attach_impl<Tree: ?Sized>(slice: &mut [Tree::Item], mut root: usize, idx: usize) -> Option<usize>
where
    Tree: self::Tree,
{
    loop {
        if Tree::is_left_of(slice, idx, root) {
            let Some(left) = Tree::left(slice, root) else {
                Tree::inc_size(slice, root);
                Tree::set_size(slice, idx, 1);
                Tree::set_left(slice, root, Some(idx));
                return Some(root);
            };

            let left_size = Tree::size(slice, left)?;
            let right_size = Tree::right_size(slice, root).unwrap_or_default();

            if Tree::is_left_of(slice, idx, left) {
                if left_size >= right_size {
                    root = Tree::rotate_right(slice, root)?;
                } else {
                    Tree::inc_size(slice, root);
                    root = left;
                }
            } else {
                let lr_size = Tree::right(slice, left)
                    .and_then(|right| Tree::size(slice, right))
                    .unwrap_or_default(); // or zero
                if lr_size >= right_size {
                    if lr_size == 0 && right_size == 0 {
                        Tree::set_left(slice, idx, Some(left));
                        Tree::set_right(slice, idx, Some(root));
                        Tree::set_size(slice, idx, left_size + 2);
                        Tree::set_left(slice, root, None);
                        Tree::set_size(slice, root, 1);
                        return Some(idx);
                    } else {
                        let new = Tree::rotate_left(slice, left)?;
                        Tree::set_left(slice, root, Some(new));
                        root = Tree::rotate_right(slice, root)?;
                    }
                } else {
                    Tree::inc_size(slice, idx);
                    root = left;
                }
            }
        } else {
            let Some(right) = Tree::right(slice, root) else {
                Tree::inc_size(slice, root);
                Tree::set_size(slice, idx, 1);
                Tree::set_right(slice, root, Some(idx));
                return Some(root);
            };

            let right_size = Tree::size(slice, right)?;
            let left_size = Tree::left_size(slice, root).unwrap_or_default();

            if Tree::is_right_of(slice, idx, right) {
                if right_size >= left_size {
                    root = Tree::rotate_left(slice, root)?;
                } else {
                    Tree::inc_size(slice, root);
                    root = right;
                }
            } else {
                let rl_size = Tree::left(slice, right)
                    .and_then(|left| Tree::size(slice, left))
                    .unwrap_or_default(); // or zero
                if rl_size >= left_size {
                    if rl_size == 0 && left_size == 0 {
                        Tree::set_left(slice, idx, Some(root));
                        Tree::set_right(slice, idx, Some(right));
                        Tree::set_size(slice, idx, right_size + 2);
                        Tree::set_left(slice, root, None);
                        Tree::set_size(slice, root, 1);
                        return Some(idx);
                    } else {
                        let new = Tree::rotate_right(slice, right)?;
                        Tree::set_right(slice, root, Some(new));
                        root = Tree::rotate_left(slice, root)?;
                    }
                } else {
                    Tree::inc_size(slice, root);
                    root = right;
                }
            }
        }
    }
}

fn detach_impl<Tree>(slice: &mut [Tree::Item], mut root: usize, idx: usize) -> Option<usize>
where
    Tree: self::Tree + ?Sized,
{
    loop {
        let left = Tree::left(slice, idx);
        let right = Tree::right(slice, idx);

        if Tree::is_left_of(slice, idx, root) {
            let rl_size =
                Tree::right(slice, idx).and_then(|right| Tree::left_size(slice, right))?;
            let rr_size =
                Tree::right(slice, idx).and_then(|right| Tree::right_size(slice, right))?;
            let left_size = Tree::left_size(slice, idx).unwrap_or_default();

            if rr_size >= left_size {
                root = Tree::rotate_left(slice, root)?;
            } else if rl_size >= left_size {
                let new = Tree::rotate_right(slice, Tree::right(slice, root).expect("..."))?;
                Tree::set_right(slice, root, Some(new));
                root = Tree::rotate_left(slice, root)?;
            } else {
                Tree::dec_size(slice, root);
                root = left.expect("...");
            }
        } else if Tree::is_right_of(slice, idx, root) {
            let ll_size = Tree::left(slice, idx).and_then(|right| Tree::left_size(slice, right))?;
            let lr_size =
                Tree::left(slice, idx).and_then(|right| Tree::right_size(slice, right))?;
            let right_size = Tree::right_size(slice, idx).unwrap_or_default();

            if ll_size >= right_size {
                root = Tree::rotate_left(slice, root)?;
            } else if lr_size >= right_size {
                let new = Tree::rotate_left(slice, Tree::left(slice, root).expect("..."))?;
                Tree::set_left(slice, root, Some(new));
                root = Tree::rotate_right(slice, root)?;
            } else {
                Tree::dec_size(slice, root);
                root = right.expect("...");
            }
        } else {
            println!("{left:?}, {right:?}");
            root = match (left, right) {
                (Some(left), Some(right)) => {
                    let replacement;
                    let (left_size, right_size) =
                        (Tree::left_size(slice, idx)?, Tree::right_size(slice, idx)?);

                    if left_size != 0 && right_size != 0 {
                        replacement = Tree::rightest(slice, left);
                        root = detach_impl::<Tree>(slice, left, replacement)?;
                    } else {
                        replacement = Tree::leftest(slice, right);
                        root = detach_impl::<Tree>(slice, right, replacement)?;
                    } // `root` is never read :(
                    Tree::set_left(slice, replacement, Some(left));
                    Tree::set_right(slice, replacement, Some(right));
                    Tree::set_size(slice, replacement, left_size + right_size);
                    replacement
                }
                (Some(left), _) => left,
                (_, Some(right)) => right,
                _ => {
                    Tree::clear(slice, idx);
                    return None;
                }
            };
            Tree::clear(slice, idx);
            return Some(root);
        }
    }
}

pub trait NoRecur: Tree {
    fn attach(slice: &mut [Self::Item], root: Option<usize>, idx: usize) -> Option<usize> {
        if let Some(root) = root {
            attach_impl::<Self>(slice, root, idx)
        } else {
            Self::set_size(slice, idx, 1);
            Some(idx)
        }
    }

    fn detach(slice: &mut [Self::Item], root: Option<usize>, idx: usize) -> Option<usize> {
        if let Some(root) = root { detach_impl::<Self>(slice, root, idx) } else { None }
    }
}
