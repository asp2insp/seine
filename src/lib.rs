extern crate sfml;

pub mod renderer;
pub mod scenegraph;


/// A library for building reactive, composable, pluggable, 2D (with depth) UIs
/// The library allows you to define Components, Transformers, Themes, Measurers, and Renderers.
/// The core abstraction is a a SceneGraph; a tree of drawable, measureable, and transformable nodes.
///
/// ## Workflow
/// Each frame, the conceptual workflow of the engine is:
///   1. Walk the Component tree and inflate the Components to build a SceneGraph
///   2. Walk the SceneGraph and apply all Transformers to mutate the graph
///   3. Create a rendering Context from the environment and apply all Themes to the Context
///   4. Use the Context.Measurer to walk the SceneGraph and populate the BoundingBox on each Node
///   5. Use the Context.Renderer to walk the SceneGraph and rasterize it to the screenbuffer
///   6. Blit the finished image to the screen
/// At each stage, we can save time by intelligently caching previous results if none of the
/// input properties have changed. We can do this since all functions involved are pure.
///
/// ## Structs
/// ### Component
/// A self-contained user interface element (widget) that describes itself as a pure function whose
/// output is a tree of other Components. Components are inflated recursively into Nodes.
/// Each Component recieves Properties from its parent and uses those to compute internal State.
/// ### Transformer
/// A transformer is a piece of middleware which can modify a Node after it has been created
/// ### Theme
/// A theme transforms a Context to add or modify properties such as colors or font sizes.
/// A theme can be attached at any level of the SceneGraph, and applies to that Node and all
/// Nodes underneath. Themes stack on top of each other.
/// ### Measurers
/// A measurer takes a SceneGraph subtree and a Context and turns it into a BoundingBox
/// ### A Renderer takes a SceneGraph subtree and turns it into a set of pixels


#[test]
fn it_works() {
}
