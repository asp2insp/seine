/// A 2D Scene Graph model.
/// The scene is represented by a tree where each node represents a single entity.
/// An entity may be drawable, or it may simply transform its children.
/// Internally, the scene graph is a copy-on-write immutable tree structure. This allows
/// easy caching of intermediate calculations, such as transformations, middleware, and
/// measure passes.
/// The scene graph maintains the invariant that each node is contained completely
///  within its parent in the XY plane, and is equal to or above its parent in the Z plane.

use renderer::*;

pub struct Node<'n> {
    transformer: Vec<&'n Transformer<'n>>,
    children: Vec<Node<'n>>,
}

pub trait Drawable {
    fn draw(renderer: &mut Renderer, context: &mut Context);
}

pub trait Measurable {
    fn measure(context: &Context);
}

pub trait Transformer<'n> {
    fn transform(node: &'n Node) -> Node<'n>;
}
