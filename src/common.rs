#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum ModelSelection {
    #[default]
    Cylinder,
    Branch,
    Leaf,
    Twig,
    Monkey,
}
