//! The match camera ($C090C6): it follows the ball with a look-ahead that leans up to 40 pixels towards where play
//! is heading, eases a quarter of the distance per frame (at most 5 pixels), in 16.16 fixed point, and stays within
//! the pitch. Set pieces and the referee's view override the target. The display shows the previous frame's camera
//! (see `pitch::render`).

/// What the camera follows this frame (all from the previous frame's state, as the original read it).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CamInput {
    /// Ball position and velocity (16.16 per frame).
    pub ball: (i16, i16),
    pub ball_v: (i32, i32),
    /// The fixed view of the centre spot area ($C0E8CA).
    pub fixed: bool,
    /// A forced target ($C0DD9C on, $C0DDA2/$C0DDA4), with no edge margin.
    pub focus: Option<(i16, i16)>,
    /// Where the look-ahead leans: the ball's velocity, or a facing direction 0 to 7, or nowhere.
    pub aim: Aim,
    /// The match event code ($C0E8AC): $15 to $1C hold the camera on a goal mouth for set pieces.
    pub event: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aim {
    /// The ball is loose ($C0E8AA = 100): lean with its velocity.
    Ball,
    /// Lean towards a facing direction (the controlled player's, or $C0E8B6).
    Facing(u8),
    /// No direction ($C0E8B6 negative): the look-ahead holds.
    None,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Camera {
    /// 16.16 fixed point ($C0DE86/$C0DE88, $C0DE8A/$C0DE8C).
    pub x: i32,
    pub y: i32,
    /// The look-ahead ($C09368, $C0936A).
    pub look_x: i16,
    pub look_y: i16,
}

impl Camera {
    pub fn px(&self) -> i32 {
        self.x >> 16
    }
    pub fn py(&self) -> i32 {
        self.y >> 16
    }

    pub fn place(x: i32, y: i32) -> Camera {
        Camera {
            x: x << 16,
            y: y << 16,
            look_x: 0,
            look_y: 0,
        }
    }

    /// One frame of $C090C6.
    pub fn step(&mut self, i: &CamInput, aim_table: &[(i32, i32); 8]) {
        let (tx, ty, margin) = if i.fixed {
            self.look_x = 0;
            self.look_y = 0;
            (0x130, 0x6B, 0x10)
        } else if let Some((fx, fy)) = i.focus {
            self.look_x = 0;
            self.look_y = 0;
            (fx as i32, fy as i32, 0)
        } else {
            let (dx, dy) = match i.aim {
                Aim::Ball => i.ball_v,
                Aim::Facing(d) => aim_table[(d & 7) as usize],
                Aim::None => (0, 0),
            };
            if dx > 0 && self.look_x != 0x28 {
                self.look_x += 2;
            } else if dx < 0 && self.look_x != -0x28 {
                self.look_x -= 2;
            }
            if dy > 0 && self.look_y != 0x28 {
                self.look_y += 2;
            } else if dy < 0 && self.look_y != -0x28 {
                self.look_y -= 2;
            }
            match i.event {
                0x15..=0x18 => {
                    self.look_x = 0;
                    self.look_y = 0;
                    (0x22E, 0x171, 0x10)
                }
                0x19..=0x1C => {
                    self.look_x = 0;
                    self.look_y = 0;
                    (0x130, 0x171, 0x10)
                }
                _ => (i.ball.0 as i32, i.ball.1 as i32, 0x10),
            }
        };
        let mut tx = tx - 0xA8 + self.look_x as i32;
        let mut ty = ty - 0x88 + self.look_y as i32;
        if tx < margin {
            tx = margin;
        }
        if tx >= 0x110 - margin {
            tx = 0x110 - margin;
        }
        ty = ty.clamp(9, 0x1C4);
        let (ddx, ddy) = (tx - self.px(), ty - self.py());
        if ddx == 0 && ddy == 0 {
            return;
        }
        let lim = 0x5_0000;
        let vx = (0x4000 * (ddx >> 2)).clamp(-lim, lim);
        let vy = (0x4000 * (ddy >> 2)).clamp(-lim, lim);
        let (nx, ny) = (self.x.wrapping_add(vx), self.y.wrapping_add(vy));
        let ix = (nx >> 16).clamp(0, 0x110);
        let iy = (ny >> 16).clamp(9, 0x1C4);
        self.x = (ix << 16) | (nx & 0xFFFF);
        self.y = (iy << 16) | (ny & 0xFFFF);
    }
}
