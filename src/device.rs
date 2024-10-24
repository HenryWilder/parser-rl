use std::fmt;

use raylib::prelude::*;

use crate::{measure_text, Plugin, Rack};

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

enum AlignX {
    RightFromLeft(usize),
    LeftFromRight(usize),
    LeftFromCenter(usize),
    RightFromCenter(usize),
    HCenter,
}
enum AlignY {
    DownFromTop(usize),
    UpFromBottom(usize),
    UpFromCenter(usize),
    DownFromCenter(usize),
    VCenter,
}

impl Device {
    pub const PLUGIN_SHARED_RADIUS: f32 = 10.0;
    pub const PLUGIN_GAP: f32 = 10.0;
    pub const PLUGIN_INSET: f32 = Self::PLUGIN_GAP + Self::PLUGIN_SHARED_RADIUS;
    pub const LABEL_FONT_SIZE: i32 = 20;
    pub const WIDTH: f32 = 200.0;
    pub const GRIP_WIDTH: f32 = 1.0;

    /// The height of the device if it has `n` plugins vertically
    fn height_with_plugins(n: usize) -> f32 {
        let num_plugins: f32 = n as f32;
        let num_spaces: f32 = n as f32 - 1.0;
        2.0 * Self::PLUGIN_SHARED_RADIUS * num_plugins + Self::PLUGIN_GAP * (num_spaces + 2.0)
    }

    /// The center x or y coordinate of the plugin `n`-plugins along the axis
    fn plugin_coord(n: usize) -> f32 {
        Self::PLUGIN_GAP + (Self::PLUGIN_SHARED_RADIUS * 2.0 + Self::PLUGIN_GAP) * n as f32 + Self::PLUGIN_SHARED_RADIUS
    }

    fn plugin_position(height: f32, x: AlignX, y: AlignY) -> Vector2 {
        Vector2 {
            x: match x {
                AlignX::RightFromLeft  (n) =>                     Self::plugin_coord(n),
                AlignX::LeftFromRight  (n) => Self::WIDTH       - Self::plugin_coord(n),
                AlignX::LeftFromCenter (n) => Self::WIDTH * 0.5 - Self::plugin_coord(n),
                AlignX::RightFromCenter(n) => Self::WIDTH * 0.5 + Self::plugin_coord(n),
                AlignX::HCenter            => Self::WIDTH * 0.5,
            },

            y: match y {
                AlignY::DownFromTop   (n)  =>                     Self::plugin_coord(n),
                AlignY::UpFromBottom  (n)  => height            - Self::plugin_coord(n),
                AlignY::UpFromCenter  (n)  => height      * 0.5 - Self::plugin_coord(n),
                AlignY::DownFromCenter(n)  => height      * 0.5 + Self::plugin_coord(n),
                AlignY::VCenter            => height      * 0.5,
            },
        }
    }

    pub fn new(position: Vector2, kind: DeviceKind) -> Self {
        use {AlignX::*, AlignY::*};

        let height = Self::height_with_plugins(match &kind {
            | DeviceKind::Label(_)
            | DeviceKind::Jump { .. }
            | DeviceKind::Math(_)
            | DeviceKind::Call
                => 2,

            | DeviceKind::Immediate(_)
            | DeviceKind::Ret
                => 1,
        });

        let pos = |x: AlignX, y: AlignY| -> Vector2 {
            Self::plugin_position(height, x, y)
        };

        let plugins = match &kind {
            // .-&----------.
            // | text       |
            // '-O----------'
            DeviceKind::Label(_)
                => Vec::from([
                        Plugin::exec_inout(pos(RightFromLeft(0), DownFromTop (0))),
                        Plugin::exec_out  (pos(RightFromLeft(0), UpFromBottom(0))),
                    ]),

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
            DeviceKind::Math(Operation::Not | Operation::Mov)
                => todo!(),
            // .-O-*-*------.
            // | [?]        |
            // '-O-*--------'
            DeviceKind::Math(_)
                => Vec::from([
                        Plugin::exec_in  (pos(RightFromLeft(0), DownFromTop (0))),
                        Plugin::exec_out (pos(RightFromLeft(0), UpFromBottom(0))),
                        Plugin::value_in (pos(RightFromLeft(1), DownFromTop (0))),
                        Plugin::value_in (pos(RightFromLeft(2), DownFromTop (0))),
                        Plugin::value_out(pos(RightFromLeft(1), UpFromBottom(0))),
                    ]),

            // .-----------.
            // | ?         *
            // '-----------'
            DeviceKind::Immediate(_)
                => Vec::from([
                        Plugin::value_out(pos(LeftFromRight(0), VCenter)),
                    ]),

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

    const CONNECTOR_OFFSET: f32 = 2.0;
    const CONNECTOR_HEIGHT: f32 = Rack::BEAM_WIDTH;
    const CONNECTOR_EXTEND: f32 = Rack::BEAM_WIDTH;
    const CONNECTOR_LENGTH: f32 = Self::WIDTH + Self::CONNECTOR_EXTEND * 2.0;
    const BOLT_RADIUS: f32 = Self::CONNECTOR_EXTEND * 0.175;

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        let mut connector_rec = Rectangle::new(
            self.rec.x - Self::CONNECTOR_EXTEND,
            self.rec.y + Self::CONNECTOR_OFFSET,
            Self::CONNECTOR_LENGTH,
            Self::CONNECTOR_HEIGHT,
        );
        d.draw_rectangle_rounded(connector_rec, 0.25, 3, Color::new(30, 30, 40, 255));
        connector_rec.y += self.rec.height - Self::CONNECTOR_HEIGHT - Self::CONNECTOR_OFFSET * 2.0;
        d.draw_rectangle_rounded(connector_rec, 0.25, 3, Color::new(30, 30, 40, 255));

        let mut bolt_pos = Vector2::new(
            self.rec.x - Self::CONNECTOR_EXTEND * 0.5,
            self.rec.y + Self::CONNECTOR_OFFSET + Self::CONNECTOR_HEIGHT * 0.5,
        );
        d.draw_circle_v(bolt_pos, Self::BOLT_RADIUS, Color::GRAY);
        bolt_pos.x += Self::WIDTH + Self::CONNECTOR_EXTEND;
        d.draw_circle_v(bolt_pos, Self::BOLT_RADIUS, Color::GRAY);
        bolt_pos.y += self.rec.height - Self::CONNECTOR_HEIGHT - Self::CONNECTOR_OFFSET * 2.0;
        d.draw_circle_v(bolt_pos, Self::BOLT_RADIUS, Color::GRAY);
        bolt_pos.x -= Self::WIDTH + Self::CONNECTOR_EXTEND;
        d.draw_circle_v(bolt_pos, Self::BOLT_RADIUS, Color::GRAY);

        let Rectangle { x, y, width, height } = self.rec;
        let mut d = d.begin_scissor_mode(x as i32, y as i32, width as i32, height as i32);
        d.draw_rectangle_rec(self.rec, Color::new(90, 90, 110, 255));
        d.draw_rectangle_rec(
            Rectangle::new(x + Self::GRIP_WIDTH, y + Self::GRIP_WIDTH, width - Self::GRIP_WIDTH * 2.0, height - Self::GRIP_WIDTH * 2.0),
            Color::new(50, 50, 60, 255)
        );

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
