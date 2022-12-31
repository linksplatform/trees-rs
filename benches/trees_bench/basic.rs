pub trait Basic<T> {
    type Node;
    fn add(&mut self, root: &mut Option<Self::Node>, node: Self::Node);
    fn remove(&mut self, root: &mut Option<Self::Node>, node: Self::Node);
}