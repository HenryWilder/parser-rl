use std::fmt;
use raylib::prelude::*;

/// A 2D transform
pub struct Transform2D {
    /// The object's position in the world
    pub position: Vector2,
    /// The direction the object is pointing
    pub rotation: f32,
}

impl Transform2D {
    /// Construct a new transform from position and rotation
    pub fn new(position: Vector2, rotation: f32) -> Self {
        Self { position, rotation }
    }
}

impl fmt::Debug for Transform2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Transform2D")
            .field("pos", &format!("{{ x: {}m, y: {}m }}", self.position.x, self.position.y))
            .field("rot", &format!("{}pi", self.rotation as f64 / PI))
            .finish()
    }
}

/// An object posessing transformation
pub trait Positioned {
    /// The object's transform in the world
    fn transform(&self) -> &Transform2D;

    /// A mutable reference to the object's transform in the world
    fn transform_mut(&mut self) -> &mut Transform2D;

    /// The object's position in the world
    fn position(&self) -> &Vector2 {
        &self.transform().position
    }

    /// A mutable reference to the object's position in the world
    fn position_mut(&mut self) -> &mut Vector2 {
        &mut self.transform_mut().position
    }
}

/// An object possibly attached to another object
pub trait Child {
    /// The type this object can be attached to
    type Parent;

    /// A reference to the object's parent, or None if not currently attached to anything
    fn parent(&self) -> Option<&Self::Parent>;

    /// Attach the object to a new parent or orphan if None
    fn attach_to(&mut self, new_parent: Option<&Self::Parent>);

    /// Alias for `attach(None)`
    fn detach(&mut self) {
        self.attach_to(None);
    }

    /// Whether the object is attached to a parent or not
    fn is_child(&self) -> bool {
        self.parent().is_some()
    }
}

/// An object that can have iterable children
pub trait Parent {
    /// The type of object that can be attached to this
    type Child;

    /// The iterator type this parent uses
    type Iter: Iterator<Item = Self::Child>;

    /// An error preventing attachment
    type AttachError;
    
    /// An iterator to the object's children
    fn children(&self) -> Self::Iter;

    /// Try to append a child to the parent
    fn try_append_child(&mut self, child: &Self::Child) -> Result<(), Self::AttachError>;

    /// Whether the object has children attached or not
    fn is_parent(&self) -> bool {
        self.children().next().is_some()
    }

    /// How many children the object has
    fn num_children(&self) -> usize {
        self.children().count()
    }
}
