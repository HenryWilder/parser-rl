use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum PluginDirection {
    In,
    Out,
    InOut,
}

impl PluginDirection {
    pub fn is_input(&self) -> bool {
        matches!(self, PluginDirection::In)
    }

    pub fn is_output(&self) -> bool {
        matches!(self, PluginDirection::Out)
    }

    pub fn is_in_out(&self) -> bool {
        matches!(self, PluginDirection::InOut)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PluginKind {
    Execution,
    Value,
}

impl PluginKind {
    pub fn radius(&self) -> f32 {
        match self {
            PluginKind::Execution => 8.0,
            PluginKind::Value     => 5.0,
        }
    }

    pub fn thickness(&self) -> f32 {
        match self {
            PluginKind::Execution => 3.5,
            PluginKind::Value     => 1.5,
        }
    }

    pub fn is_execution(&self) -> bool {
        matches!(self, PluginKind::Execution)
    }

    pub fn is_variable(&self) -> bool {
        matches!(self, PluginKind::Value)
    }
}

#[derive(Debug, Clone)]
pub struct Plugin {
    kind: PluginKind,
    direction: PluginDirection,
    pub offset: Vector2,
}

impl Plugin {
    pub fn new(kind: PluginKind, direction: PluginDirection, offset: Vector2) -> Self {
        Self { kind, direction, offset }
    }

    pub fn exec_in(offset: Vector2) -> Self {
        Self::new(PluginKind::Execution, PluginDirection::In, offset)
    }
    pub fn exec_out(offset: Vector2) -> Self {
        Self::new(PluginKind::Execution, PluginDirection::Out, offset)
    }
    pub fn exec_inout(offset: Vector2) -> Self {
        Self::new(PluginKind::Execution, PluginDirection::InOut, offset)
    }

    pub fn value_in(offset: Vector2) -> Self {
        Self::new(PluginKind::Value, PluginDirection::In, offset)
    }
    pub fn value_out(offset: Vector2) -> Self {
        Self::new(PluginKind::Value, PluginDirection::Out, offset)
    }
    pub fn value_inout(offset: Vector2) -> Self {
        Self::new(PluginKind::Value, PluginDirection::InOut, offset)
    }

    pub fn is_input(&self) -> bool {
        self.direction.is_input()
    }
    pub fn is_output(&self) -> bool {
        self.direction.is_output()
    }
    pub fn is_in_out(&self) -> bool {
        self.direction.is_in_out()
    }

    pub fn is_execution(&self) -> bool {
        self.kind.is_execution()
    }
    pub fn is_variable(&self) -> bool {
        self.kind.is_variable()
    }

    pub fn radius(&self) -> f32 {
        self.kind.radius()
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, center: Vector2) {
        let color = match self.kind {
            PluginKind::Execution => match self.direction {
                PluginDirection::In    => Color::GRAY,
                PluginDirection::Out   => Color::GAINSBORO,
                PluginDirection::InOut => Color::GREEN,
            }
            PluginKind::Value => match self.direction {
                PluginDirection::In    => Color::BLUE,
                PluginDirection::Out   => Color::RED,
                PluginDirection::InOut => Color::VIOLET,
            }
        };
        let radius = self.radius();
        let thick = self.kind.thickness();
        d.draw_circle_v(center + Vector2::new(0.0, thick * 0.25), radius + thick * 0.5, Color::new(0, 0, 0, 40));
        d.draw_circle_v(center, radius, color);
        d.draw_circle_v(center + Vector2::new(0.0, thick * 0.25), radius - thick * 0.5, Color::new(0, 0, 0, 40));
        d.draw_circle_v(center, radius - thick, Color::BLACK);
    }
}
