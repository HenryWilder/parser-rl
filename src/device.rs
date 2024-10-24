use std::fmt;

use raylib::prelude::*;

use crate::{measure_text, Plugin};

#[derive(Debug, Clone, Copy)]
pub enum JumpCondition {
    Always,
    Zero,
    Sign,
    Overflow,
    Carry,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Mov,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Orr,
    Not,
    Xor,
    Shl,
    Shr,
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    I32(i32),
    U32(u32),
    F32(f32),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::I32  (n) => write!(f, "{n}"),
            Value::U32 (n) => write!(f, "{n}u"),
            Value::F32(n) => write!(f, "{n:.3}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DeviceKind {
    Label(String),
    Jump { is_not: bool, condition: JumpCondition },
    Math(Operation),
    Immediate(Value),
    Call,
    Ret,
}

#[derive(Debug, Clone)]
pub struct Device {
    rec: Rectangle,
    kind: DeviceKind,
    plugins: Vec<Plugin>,
}

impl Device {
    pub const PLUGIN_SHARED_RADIUS: f32 = 10.0;
    pub const PLUGIN_GAP: f32 = 10.0;
    pub const PLUGIN_INSET: f32 = Self::PLUGIN_GAP + Self::PLUGIN_SHARED_RADIUS;
    pub const LABEL_FONT_SIZE: i32 = 20;
    pub const WIDTH: f32 = 200.0;
    pub const GRIP_WIDTH: f32 = 10.0;

    pub fn new(position: Vector2, kind: DeviceKind) -> Self {
        let (height, plugins) = match &kind {
            // .-&----------.
            // | text       |
            // '-O----------'
            DeviceKind::Label(_)
                => {
                    let height = (Self::PLUGIN_INSET + Self::PLUGIN_SHARED_RADIUS) * 2.0 + Self::PLUGIN_GAP;
                    (height, Vec::from([
                        Plugin::exec_inout(Self::PLUGIN_INSET, Self::PLUGIN_INSET),
                        Plugin::exec_out  (Self::PLUGIN_INSET, height - Self::PLUGIN_INSET),
                    ]))
                },

            // .-O----------.
            // | [A]        O
            // '------------'
            DeviceKind::Jump { condition: JumpCondition::Always, .. }
                => todo!(),
            // .-O-*--------.
            // | [?]        O
            // '-O----------'
            DeviceKind::Jump { .. }
                => todo!(),

            // .-O-*--------.
            // | [~]        |
            // '-O-*--------'
            DeviceKind::Math(Operation::Not)
                => todo!(),
            // .-O-*--------.
            // | [=]        |
            // '-O-*--------'
            DeviceKind::Math(Operation::Mov)
                => todo!(),
            // .-O-*-*------.
            // | [?]        |
            // '-O-*--------'
            DeviceKind::Math(_)
                => todo!(),

            // .-----------.
            // | ?         *
            // '-----------'
            DeviceKind::Immediate(_)
                => {
                    let height = Self::PLUGIN_INSET * 2.0;
                    (height, Vec::from([
                        Plugin::value_out(Self::WIDTH - Self::PLUGIN_INSET, height * 0.5),
                    ]))
                },

            // .-O---------.
            // |           &
            // '-O---------'
            DeviceKind::Call
                => todo!(),

            // .-O---------.
            // |           |
            // '-----------'
            DeviceKind::Ret
                => todo!(),
        };
        Self {
            rec: Rectangle::new(position.x, position.y, Self::WIDTH, height),
            plugins,
            kind,
        }
    }

    pub fn move_y(&mut self, y_amount: f32) {
        self.rec.y += y_amount;
    }

    pub fn set_y(&mut self, y: f32) {
        self.rec.y = y;
    }

    pub fn position(&self) -> Vector2 {
        Vector2::new(self.rec.x, self.rec.y)
    }

    pub fn rectangle(&self) -> &Rectangle {
        &self.rec
    }

    pub fn bottom_y(&self) -> f32 {
        self.rec.y + self.rec.height
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        let Rectangle { x, y, width, height } = self.rec;
        let mut d = d.begin_scissor_mode(x as i32, y as i32, width as i32, height as i32);
        d.draw_rectangle_rec(self.rec, Color::new(90, 90, 110, 255));
        d.draw_rectangle_rec(Rectangle::new(x + Self::GRIP_WIDTH, y, width - Self::GRIP_WIDTH * 2.0, height), Color::new(50, 50, 60, 255));

        for plugin in self.plugins.iter() {
            plugin.draw(&mut d, self.position() + plugin.offset);
        }

        match &self.kind {
            DeviceKind::Label(text) => {
                d.draw_text(
                    text.as_str(),
                    (self.rec.x + Self::GRIP_WIDTH + Self::PLUGIN_INSET * 2.0) as i32,
                    (self.rec.y + Self::PLUGIN_GAP) as i32,
                    Self::LABEL_FONT_SIZE,
                    Color::WHITE,
                );
            }
            DeviceKind::Immediate(value) => {
                d.draw_text(
                    value.to_string().as_str(),
                    (self.rec.x + Self::GRIP_WIDTH + Self::PLUGIN_GAP) as i32,
                    (self.rec.y + Self::PLUGIN_GAP) as i32,
                    Self::LABEL_FONT_SIZE,
                    Color::WHITE,
                );
            }
            _ => {}
        }
    }
}
