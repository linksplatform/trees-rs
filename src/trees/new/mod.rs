use platform_data::LinkType;
use tap::Pipe;

pub trait Tree<T: LinkType> {
    type Item;

    fn size(slice: &[Self::Item], idx: T) -> Option<T>;
    fn left(slice: &[Self::Item], idx: T) -> Option<T>;
    fn right(slice: &[Self::Item], idx: T) -> Option<T>;

    fn set_size(slice: &mut [Self::Item], idx: T, value: T);
    fn set_left(slice: &mut [Self::Item], idx: T, value: Option<T>);
    fn set_right(slice: &mut [Self::Item], idx: T, value: Option<T>);

    fn is_left_of(slice: &[Self::Item], first: T, second: T) -> bool;
    fn is_right_of(slice: &[Self::Item], first: T, second: T) -> bool {
        first != second && !Self::is_left_of(slice, first, second)
    }

    fn left_size(slice: &[Self::Item], idx: T) -> Option<T> {
        Self::left(slice, idx).and_then(|idx| Self::size(slice, idx))
    }

    fn right_size(slice: &[Self::Item], idx: T) -> Option<T> {
        Self::right(slice, idx).and_then(|idx| Self::size(slice, idx))
    }

    fn rightest(slice: &[Self::Item], mut current: T) -> Option<T> {
        while let Some(next) = Self::right(slice, current) {
            current = next;
        }
        Some(current)
    }

    fn leftest(slice: &[Self::Item], mut current: T) -> Option<T> {
        while let Some(next) = Self::left(slice, current) {
            current = next;
        }
        Some(current)
    }

    fn next(slice: &[Self::Item], idx: T) -> Option<T> {
        Self::right(slice, idx).and_then(|idx| Self::leftest(slice, idx))
    }

    fn prev(slice: &[Self::Item], idx: T) -> Option<T> {
        Self::left(slice, idx).and_then(|idx| Self::rightest(slice, idx))
    }

    fn is_contains(slice: &[Self::Item], mut root: T, idx: T) -> bool {
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

    fn inc_size(slice: &mut [Self::Item], idx: T) -> Option<()> {
        Self::set_size(slice, idx, Self::size(slice, idx)? + T::one()).pipe(Some)
    }

    fn dec_size(slice: &mut [Self::Item], idx: T) -> Option<()> {
        Self::set_size(slice, idx, Self::size(slice, idx)? - T::one()).pipe(Some)
    }

    fn fix_size(slice: &mut [Self::Item], idx: T) {
        Self::set_size(
            slice,
            idx,
            Self::left_size(slice, idx).unwrap_or_default()
                + Self::right_size(slice, idx).unwrap_or_default()
                + T::one(),
        )
    }

    fn clear(slice: &mut [Self::Item], idx: T) {
        Self::set_left(slice, idx, None);
        Self::set_right(slice, idx, None);
        Self::set_size(slice, idx, T::zero());
    }

    fn rotate_left(slice: &mut [Self::Item], root: T) -> Option<T> {
        let right = Self::right(slice, root)?;
        Self::set_right(slice, root, Self::left(slice, right));
        Self::set_left(slice, right, Some(root));
        Self::set_size(slice, right, Self::size(slice, root)?);
        Self::fix_size(slice, root);
        Some(right)
    }

    fn rotate_right(slice: &mut [Self::Item], root: T) -> Option<T> {
        let left = Self::left(slice, root)?;
        Self::set_left(slice, root, Self::right(slice, left));
        Self::set_right(slice, left, Some(root));
        Self::set_size(slice, left, Self::size(slice, root)?);
        Self::fix_size(slice, root);
        Some(left)
    }
}

fn attach_impl<T, Tree>(slice: &mut [Tree::Item], mut root: T, idx: T) -> Option<T>
where
    T: LinkType,
    Tree: self::Tree<T>,
{
    loop {
        let left = Tree::left(slice, root);
        let right = Tree::right(slice, root);
        if Tree::is_left_of(slice, idx, root) {
            match left {
                None => {
                    Tree::inc_size(slice, root)?;
                    Tree::set_size(slice, idx, T::one());
                    Tree::set_left(slice, root, Some(idx));
                    return Some(root);
                }
                Some(left) => {
                    let left_size = Tree::size(slice, left)?;
                    let right_size = Tree::right_size(slice, root).unwrap_or_default();
                    if Tree::is_left_of(slice, idx, left) {
                        if left_size >= right_size {
                            root = Tree::rotate_right(slice, root)?;
                        } else {
                            Tree::inc_size(slice, root)?;
                            root = left;
                        }
                    } else {
                        let lr_size = Tree::right(slice, left)
                            .and_then(|right| Tree::size(slice, right))
                            .unwrap_or_default();
                        if lr_size >= right_size {
                            if lr_size == T::zero() && right_size == T::zero() {
                                Tree::set_left(slice, idx, Some(left));
                                Tree::set_right(slice, idx, Some(root));
                                Tree::set_size(slice, idx, left_size + T::two());
                                Tree::set_left(slice, root, None);
                                Tree::set_size(slice, root, T::one());
                                return Some(idx);
                            } else {
                                let new = Tree::rotate_left(slice, left)?;
                                Tree::set_left(slice, root, Some(new));
                                root = Tree::rotate_right(slice, root)?;
                            }
                        } else {
                            Tree::inc_size(slice, root)?;
                            root = left;
                        }
                    }
                }
            }
        } else {
            match right {
                None => {
                    Tree::inc_size(slice, root)?;
                    Tree::set_size(slice, idx, T::one());
                    Tree::set_right(slice, root, Some(idx));
                    return Some(root);
                }
                Some(right) => {
                    let right_size = Tree::size(slice, right)?;
                    let left_size = Tree::left_size(slice, root).unwrap_or_default();
                    if Tree::is_right_of(slice, idx, right) {
                        if right_size >= left_size {
                            root = Tree::rotate_left(slice, root)?;
                        } else {
                            Tree::inc_size(slice, root)?;
                            root = right;
                        }
                    } else {
                        let rl_size = Tree::left(slice, right)
                            .and_then(|left| Tree::size(slice, left))
                            .unwrap_or_default();
                        if rl_size >= left_size {
                            if rl_size == T::zero() && left_size == T::zero() {
                                Tree::set_left(slice, idx, Some(root));
                                Tree::set_right(slice, idx, Some(right));
                                Tree::set_size(slice, idx, right_size + T::two());
                                Tree::set_left(slice, root, None);
                                Tree::set_size(slice, root, T::one());
                                return Some(idx);
                            } else {
                                let new = Tree::rotate_right(slice, right)?;
                                Tree::set_right(slice, root, Some(new));
                                root = Tree::rotate_left(slice, root)?;
                            }
                        } else {
                            Tree::inc_size(slice, root)?;
                            root = right;
                        }
                    }
                }
            }
        }
    }
}

pub trait NoRecur<T: LinkType>: Tree<T> + Sized {
    fn attach(slice: &mut [Self::Item], root: Option<T>, idx: T) -> Option<T> {
        if let Some(root) = root {
            attach_impl::<_, Self>(slice, root, idx)
        } else {
            Self::set_size(slice, idx, T::one());
            Some(idx)
        }
    }

    fn detach(_slice: &mut [Self::Item], _root: Option<T>, _idx: T) -> Option<T> {
        todo!()
    }
}
