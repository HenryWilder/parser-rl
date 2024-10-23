use super::*;

pub trait Hoverable {
    fn is_draggable(&self) -> bool {
        false
    }
}

pub trait Draggable {
    fn drag(&mut self, delta_position: Vector2, delta_time: f32);
}

impl<T: Draggable> Hoverable for T {
    fn is_draggable(&self) -> bool {
        true
    }
}

pub type HoveredTarget = dyn Hoverable;
pub type DraggedTarget = dyn Draggable;

pub enum Hover<'target> {
    Hovering(&'target HoveredTarget),
    Dragging(&'target DraggedTarget),
}

pub struct HoverHandler<'trg>(Option<Hover<'trg>>);

impl<'trg> HoverHandler<'trg> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn is_hovering(&self) -> bool {
        self.0.is_some()
    }
    pub fn start_hovering(&mut self, target: &'trg HoveredTarget) {
        self.0 = Some(Hover::Hovering(target));
    }
    pub fn hovering(&self) -> Option<&Hover> {
        self.0.as_ref()
    }
    pub fn hovering_mut(&mut self) -> Option<&mut Hover<'trg>> {
        self.0.as_mut()
    }

    pub fn is_dragging(&self) -> bool {
        self.0.as_ref().is_some_and(|hover| matches!(hover, Hover::Dragging(_)))
    }
    pub fn start_dragging(&mut self, target: &'trg DraggedTarget) {
        self.0 = Some(Hover::Dragging(target));
    }
    pub fn dragging(&self) -> Option<&Hover> {
        self.0.as_ref().and_then(|hover| hover.is_dragging.then_some(hover))
    }
    pub fn dragging_mut(&mut self) -> Option<&mut Hover<'trg>> {
        self.0.as_mut().and_then(|hover| hover.is_dragging.then_some(hover))
    }

    pub fn clear(&mut self) {
        self.0 = None;
    }
}