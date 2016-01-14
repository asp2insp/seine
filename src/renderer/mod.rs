/// A backend for SceneGraph which draws to SFML
/// The renderer walks the scene graph tree and draws to its buffer.
/// The renderer maintains a context that each node can manipulate.
/// The context can also be modified before each render pass. The
/// renderer walks the tree twice for each frame. Once by calling measure
/// from the leaves up to the root. The second time by calling draw from the
/// root down to the leaves.
///
use sfml::graphics::Color;

pub struct Renderer;

pub struct Context {
    background_color: Color,
    text_color: Color,
    shadow_color: Color,
}
