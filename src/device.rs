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
    Int(i32),
    UInt(u32),
    Float(f32),
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
struct DevicePlugins {
    top:    Vec<Plugin>,
    bottom: Vec<Plugin>,
    left:   Vec<Plugin>,
    right:  Vec<Plugin>,
}

impl DevicePlugins {
    pub fn new() -> Self {
        Self {
            top:    Vec::new(),
            bottom: Vec::new(),
            left:   Vec::new(),
            right:  Vec::new(),
        }
    }

    pub fn init() -> DevicePluginsBuilder {
        DevicePluginsBuilder::new()
    }
}

struct DevicePluginsBuilder(DevicePlugins);

impl DevicePluginsBuilder {
    pub fn new() -> Self {
        Self(DevicePlugins::new())
    }

    pub fn top(mut self, plugs: impl IntoIterator<Item = Plugin>) -> Self {
        self.0.top.extend(plugs);
        self
    }
    pub fn bottom(mut self, plugs: impl IntoIterator<Item = Plugin>) -> Self {
        self.0.bottom.extend(plugs);
        self
    }
    pub fn left(mut self, plugs: impl IntoIterator<Item = Plugin>) -> Self {
        self.0.left.extend(plugs);
        self
    }
    pub fn right(mut self, plugs: impl IntoIterator<Item = Plugin>) -> Self {
        self.0.right.extend(plugs);
        self
    }

    pub fn build(self) -> DevicePlugins {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Device {
    rec: Rectangle,
    kind: DeviceKind,
    plugins: DevicePlugins,
}

impl Device {
    pub const PLUGIN_SHARED_RADIUS: f32 = 10.0;
    pub const PLUGIN_GAP: f32 = 10.0;
    pub const LABEL_FONT_SIZE: i32 = 20;

    fn refresh_dimensions(&mut self) {
        let DevicePlugins { top, bottom, left, right } = &self.plugins;

        let [top_width, bottom_width, left_height, right_height] = [top, bottom, left, right]
            .map(|list| list.len() as f32 * (Self::PLUGIN_GAP + Self::PLUGIN_SHARED_RADIUS * 2.0) + Self::PLUGIN_GAP);

        self.rec.width = top_width.max(bottom_width);
        self.rec.height = left_height.max(right_height);

        match &self.kind {
            DeviceKind::Label(text) => {
                let comment_width = measure_text(text.as_str(), Self::LABEL_FONT_SIZE) as f32;
                let comment_height = Self::LABEL_FONT_SIZE as f32;

                self.rec.width  = self.rec.width .max(Self::PLUGIN_GAP * 2.0 + comment_width);
                self.rec.height = self.rec.height.max(Self::PLUGIN_GAP * 2.0 + comment_height);
            }

            _ => {}
        }
    }

    pub fn new(position: Vector2, kind: DeviceKind) -> Self {
        let plugins: DevicePlugins = match kind {
            // .-O----.
            // & text |
            // '-O----'
            DeviceKind::Label(_)
                => DevicePlugins::init()
                    .top([Plugin::exec_in()])
                    .left([Plugin::exec_inout()])
                    .bottom([Plugin::exec_out()])
                    .build(),

            // .-O---.
            // | [A] O
            // '-----'
            DeviceKind::Jump { condition: JumpCondition::Always, .. }
                => DevicePlugins::init()
                    .top([Plugin::exec_in()])
                    .right([Plugin::exec_out()])
                    .build(),
            // .-O-*-.
            // | [?] O
            // '-O---'
            DeviceKind::Jump { .. }
                => DevicePlugins::init()
                    .top([Plugin::exec_in(), Plugin::value_in()])
                    .right([Plugin::exec_out()])
                    .bottom([Plugin::exec_out()])
                    .build(),

            // .-O-*-.
            // | [~] |
            // '-O-*-'
            DeviceKind::Math(Operation::Not)
                => DevicePlugins::init()
                    .top([Plugin::exec_in(), Plugin::value_in()])
                    .bottom([Plugin::exec_out(), Plugin::value_out()])
                    .build(),
            // .-O-*-.
            // | [=] |
            // '-O-*-'
            DeviceKind::Math(Operation::Mov)
                => DevicePlugins::init()
                    .top([Plugin::exec_in(), Plugin::value_in()])
                    .bottom([Plugin::exec_out(), Plugin::value_out()])
                    .build(),
            // .-O-*-*-.
            // | [?]   |
            // '-O-*---'
            DeviceKind::Math(_)
                => DevicePlugins::init()
                    .top([Plugin::exec_in(), Plugin::value_in(), Plugin::value_in()])
                    .bottom([Plugin::exec_out(), Plugin::value_out()])
                    .build(),

            // .---.
            // | ? |
            // '-*-'
            DeviceKind::Immediate(_)
                => DevicePlugins::init()
                    .bottom([Plugin::value_out()])
                    .build(),

            // .-O-.
            // |   &
            // '-O-'
            DeviceKind::Call
                => DevicePlugins::init()
                    .top([Plugin::exec_in()])
                    .right([Plugin::exec_inout()])
                    .bottom([Plugin::exec_out()])
                    .build(),

            // .-O-.
            // |   |
            // '---'
            DeviceKind::Ret
                => DevicePlugins::init()
                    .top([Plugin::exec_in()])
                    .build(),
        };

        let mut output = Self {
            rec: Rectangle::new(position.x, position.y, Default::default(), Default::default()),
            kind,
            plugins,
        };
        output.refresh_dimensions();
        output
    }

    pub fn move_y(&mut self, y_amount: f32) {
        self.rec.y += y_amount;
    }

    pub fn set_position(&mut self, position: Vector2) {
        (self.rec.x, self.rec.y) = (position.x, position.y);
    }

    pub fn rectangle(&self) -> &Rectangle {
        &self.rec
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        let rec = self.rec;
        d.draw_rectangle_rec(rec, Color::BLUE);
        for (list, y) in [(&self.plugins.top, rec.y), (&self.plugins.bottom, rec.y + rec.height)] {
            for (i, plugin) in list.iter().enumerate() {
                plugin.draw(d, Vector2::new(rec.x + (i * 2 + 1) as f32 * (Self::PLUGIN_SHARED_RADIUS + Self::PLUGIN_GAP), y));
            }
        }
        for (list, x) in [(&self.plugins.left, rec.x), (&self.plugins.right, rec.x + rec.width)] {
            for (i, plugin) in list.iter().enumerate() {
                plugin.draw(d, Vector2::new(x, rec.y + (i * 2 + 1) as f32 * (Self::PLUGIN_SHARED_RADIUS + Self::PLUGIN_GAP)));
            }
        }
        match &self.kind {
            DeviceKind::Label(text) => {
                d.draw_text(
                    text,
                    (rec.x + Self::PLUGIN_GAP) as i32,
                    (rec.y + Self::PLUGIN_GAP) as i32,
                    Self::LABEL_FONT_SIZE,
                    Color::WHITE,
                );
            }
            _ => {}
        }
    }
}
