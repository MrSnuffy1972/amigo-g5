//! The bonus round (phase 5): balls bounce in from both sides and the player turns them away with the shield,
//! held in one of seven places by the joystick. A ball that gets through knocks the player down; all 64 turned away
//! scores a bonus. The ball physics run in the frame interrupt, as in the original.

use crate::ik::{Bob, Ik};

/// One bonus object (the original's ten parallel byte arrays from $6A7, slot 4 = the shield).
#[derive(Default, Clone, Copy)]
pub struct Obj {
    pub hit_f: bool,                    // $6a7 touched the fighter
    pub hit_s: bool,                    // $6b1 touched the shield
    pub y: u16,                         // $6c5:$6bb (high byte = row - $60)
    pub vy: i16,                        // $6d9:$6cf
    pub x: u16,                         // $6ed:$6e3 (high byte = 2-pixel position)
    pub vx: i16,                        // $6f7:$701
    pub state: u8,                      // $70b
    pub grav: u8,                       // $715
    pub floor: u8,                      // $71f
    pub anim: u8,                       // $729
    pub spin: u8,                       // $733
    pub bounces: u8,                    // $73d
    pub pos: Option<(usize, i32, i32)>, // $77a where it was drawn (screen, pixels)
}

#[derive(Default, Clone)]
pub struct Bonus {
    pub o: [Obj; 10],
    pub active: bool,    // $69f
    pub speed_step: u8,  // $69c balls so far
    pub done: bool,      // $69d
    pub alive: u8,       // $69e
    pub hit: bool,       // $6a5 the player was knocked down
    pub shield_on: bool, // $6a6
    pub accum: u16,      // $8d6:$8d5
    pub speed: u16,      // $8d4:$8d3
    pub shield_pose: u8, // $8dd
    pub drawing: bool,   // $8de
    pub gravity: u8,     // $8df
    pub fall: u8,        // $8d8
    pub who: usize,      // $679
}

fn t(g: &Ik, o: usize) -> u8 {
    g.a.t("bonus_in")[o]
}

/// reset_game_state ($B106): the round, for each player in turn.
pub fn reset_game_state(g: &mut Ik) {
    g.bonus.active = false;
    g.bonus.drawing = false;
    set_global_palette_entries(g);
    g.four_steps_per_frame();
    g.sil_on = true;
    g.obj_on = true;
    for who in [1usize, 0] {
        g.bonus.who = who;
        g.cur = who;
        for o in g.bonus.o.iter_mut() {
            o.pos = None;
        }
        if g.human[who] == 0 {
            continue;
        }
        g.flash[who] = 0x14;
        for k in 0..5 {
            g.bonus.o[k] = Obj::default();
        }
        let b = &mut g.bonus;
        b.drawing = false;
        b.shield_pose = 1;
        b.speed = 0x0080;
        b.speed_step = 0;
        b.done = false;
        b.fall = 0;
        b.hit = false;
        b.gravity = 0xc;
        g.want[who] = 3;
        decode_byte_from_table(g);
        g.bonus.o[4].state = 1;
        g.bonus.o[4].floor = 0x73;
        g.bonus.shield_on = true;
        g.msg_req = 0x16;
        g.wait = 100;
        self_modifying_loop_2(g);
        g.four_steps_per_frame();
        g.bonus.drawing = true;
        g.bonus.active = true;
        g.msg_req = 0xff;
        loop {
            self_modifying_loop_2(g);
            if g.bonus.hit {
                // knocked down: fall, then wait for the balls to go
                g.frozen[who] = 0;
                g.hit[who] = 1;
                let m = g.bonus.fall;
                g.want[who] = m;
                g.initialize_sprite_structure(who, m);
                loop {
                    if g.frozen[who] == 0 {
                        g.cur = who;
                        g.process_player_state_flags();
                    }
                    set_global_from_table_byte(g);
                    g.wait = 0;
                    g.step_wait = 0x14;
                    self_modifying_loop_2(g);
                    if g.bonus.alive == 0 {
                        break;
                    }
                }
                break;
            }
            if g.bonus.done && g.bonus.alive == 0 {
                g.scored = 0;
                g.shown_secs = 0x50;
                g.points_to_add = 0x50;
                g.cur = who;
                g.update_bcd_counters_and_recurse();
                g.wait = 100;
                g.msg_req = 0x15;
                self_modifying_loop_2(g);
                g.msg_req = 0xff;
                g.wait = 0x32;
                self_modifying_loop_2(g);
                break;
            }
        }
    }
    g.wait_frames(0x28);
    set_global_palette_entries(g);
    g.draw_into_both_buffers();
    g.sil_on = false;
    g.obj_on = false;
    g.four_steps_per_frame();
    g.bonus.active = false;
    g.wait_frames(1);
    g.phase = 1;
}

/// set_global_palette_entries ($B2FE): the fighters off the screen while the round is set up.
fn set_global_palette_entries(g: &mut Ik) {
    g.frame_x = [0x57; 3];
    g.x = [0x88; 3];
    g.bonus.shield_on = false;
    g.msg_req = 0xff;
    g.wait = 0;
    self_modifying_loop_2(g);
    self_modifying_loop_2(g);
}

/// self_modifying_loop_2 ($B33E): steps until the wait runs out: the fighter, the shield, the judge.
fn self_modifying_loop_2(g: &mut Ik) {
    loop {
        g.erase_bobs_and_reset_list();
        crate::speech::decode_global_byte_table(g);
        decode_byte_from_table(g);
        g.cur = g.bonus.who;
        g.dispatch_via_indexed_table();
        check_and_call_flagged_routine(g);
        if g.speech.speaking {
            g.shift_lfsr_state();
        }
        g.step_wait = 0x14;
        g.flip();
        if g.wait == 0 {
            return;
        }
    }
}

/// decode_byte_from_table ($BDEC): the joystick moves the shield between its seven places (a table of the place
/// and the direction gives the next place), which sets the fighter's frame and the shield's position.
fn decode_byte_from_table(g: &mut Ik) {
    if g.bonus.hit {
        return;
    }
    let d2 = g.bonus.who;
    let d1 = (g.joy[d2] & 0xf) as usize;
    let dir = t(g, d1) as usize;
    let mut d0 = t(g, 0x10 + (g.want[d2] as usize * 9 + dir) % 63);
    if d0 & 0x80 != 0 {
        d0 = ((g.facing[d2] ^ 1) << 2) + (d0 & 3);
    }
    let d = (d0 as usize).min(6);
    g.want[d2] = d0;
    g.bonus.o[4].x = ((t(g, 0x56 + d) & 0xf8) as u16) << 8;
    g.bonus.o[4].y = (t(g, 0x64 + d) as u16) << 8;
    g.bonus.shield_pose = t(g, 0x4f + d);
    g.facing[d2] = t(g, 0x6b + d);
    g.frame_x[d2] = t(g, 0x72 + d);
    g.x[d2] = t(g, 0x5d + d) & 0xf8;
}

/// check_and_call_flagged_routine ($B37A): the shield.
fn check_and_call_flagged_routine(g: &mut Ik) {
    if !g.bonus.shield_on || g.bonus.o[4].state == 0 {
        return;
    }
    let s = g.bonus.o[4];
    let x = (((s.x >> 8) as i32) << 1) | ((s.x >> 7) & 1) as i32;
    let y = (s.y >> 8) as i32 + 0x60;
    let pose = g.bonus.shield_pose;
    let a = match pose {
        0 => 0x21FB4,
        2 => 0x21FB6,
        _ if g.shield_alt != 0 => 0x2339C,
        _ => 0x21FB8,
    };
    let src = Bob::at(a + 0x600);
    let b = g.back;
    g.bob(b, src, (x & 0x1ff) - 32, y, 0x17, true);
    if pose == 1 {
        g.bob(b, src.right(1), ((x + 16) & 0x1ff) - 32, y, 0x17, true);
    }
}

/// set_global_from_table_byte ($B45A): the thud when the knocked-down fighter lands.
fn set_global_from_table_byte(g: &mut Ik) {
    let f = g.frame_x[g.bonus.who];
    let s = match f {
        0x2e => 0xd,
        0x2a => 0xe,
        _ => return,
    };
    g.delay_snd = (3, s);
}

/// process_state_table ($B48E, frame interrupt): the balls: collisions, physics, drawing and new balls.
pub fn process_state_table(g: &mut Ik) {
    if g.phase != 5 && g.phase != 7 || !g.bonus.active {
        return;
    }
    clear_and_update_flags(g);
    g.bonus.alive = 0;
    for k in (0..5).rev() {
        if g.bonus.o[k].state == 0 {
            continue;
        }
        if k < 4 {
            g.bonus.alive += 1;
            let o = g.bonus.o[k];
            if o.state & 2 == 0 {
                if o.hit_s {
                    let snd = if o.anim == 7 { 0xb } else { 0xf };
                    g.set_state_flags(snd);
                    g.snd_t[0] = 0x1e;
                    g.bonus.o[k].state = 0x13;
                    g.bonus.o[k].vx = -o.vx;
                    g.points_to_add = 1;
                    g.cur = g.bonus.who;
                    g.save_and_restore_global_variable();
                } else if o.hit_f {
                    if g.bonus.hit {
                        g.bonus.o[k].state |= 0x10;
                        g.bonus.o[k].vy = -o.vy;
                        g.set_state_flags(3);
                    } else {
                        g.bonus.hit = true;
                        g.set_state_flags(0x11);
                        g.snd_t = [0x32, 0x32];
                        g.bonus.o[k].vy = 0;
                        let who = g.bonus.who;
                        let away = (((o.vx >> 15) & 1) as u8 ^ g.facing[who]) & 1;
                        g.bonus.fall = g.a.t("bonus")[away as usize];
                        g.bonus.o[k].state = 0x13;
                    }
                }
            }
        }
        update_player_stats(g, k);
        decrement_or_reset_counter(g, k);
    }
    if g.bonus.drawing {
        fill_table_from_callee(g);
        ball_shadows(g);
        decode_sprite_state_table(g);
    }
}

/// update_bonus_objects ($C330): the bomb round's objects (the bomb round runs as the ball round natively).
pub fn update_bonus_objects(_g: &mut Ik) {}

/// The bomb round (phase 7): played as the ball round in this version.
pub fn reset_input_state_and_wait_for_key(g: &mut Ik) {
    reset_game_state(g);
}

/// clear_and_update_flags ($BD2C): takes the balls off the shown screen and tests each against the fighter's and
/// the shield's outlines of the last picture.
fn clear_and_update_flags(g: &mut Ik) {
    for k in (0..4).rev() {
        g.bonus.o[k].hit_f = false;
        g.bonus.o[k].hit_s = false;
        let Some((sh, x, y)) = g.bonus.o[k].pos else {
            continue;
        };
        g.bonus.o[k].pos = None;
        let o = g.bonus.o[k];
        let src = ball_src(o);
        let (mut hf, mut hs) = (false, false);
        let near = g.bonus.hit || (0x77..0x10e).contains(&(x + 32));
        for r in 0..12 {
            for c in 0..16 {
                let (xx, yy) = (x + c, y + r);
                if !(0..320).contains(&xx) || !(0..200).contains(&yy) {
                    continue;
                }
                if g.pic(src.src)
                    .at((src.x + c) as usize, (src.y + r) as usize)
                    == 0
                {
                    continue;
                }
                let p = yy as usize * 320 + xx as usize;
                g.scr[sh][p] = g.a.background.px[p];
                if near {
                    hf |= g.layer[g.lr_f][p] != 0;
                    hs |= g.layer[g.lr_o][p] != 0;
                }
            }
        }
        g.bonus.o[k].hit_f = hf;
        g.bonus.o[k].hit_s = hs;
    }
}

fn ball_src(o: Obj) -> Bob {
    Bob::at(0x1DCBC + 0x600 + o.anim as u32 * 0x6e0 + (o.spin as u32 & 7) * 2)
}

/// update_player_stats ($B6DC): gravity, the bounce off the floor (with the damping and doubling flags), the
/// sideways move, and leaving the screen.
fn update_player_stats(g: &mut Ik, k: usize) {
    let mut o = g.bonus.o[k];
    o.vy = o.vy.wrapping_add(o.grav as i16);
    o.y = o.y.wrapping_add(o.vy as u16);
    let yh = (o.y >> 8) as u8;
    if yh >= o.floor {
        let ny = (2 * o.floor as u16).wrapping_sub(yh as u16) as u8;
        o.y = ((ny as u16) << 8) | (o.y & 0xff);
        o.vy = o.vy.wrapping_neg();
        if o.vy != 0 {
            o.bounces = o.bounces.wrapping_add(1);
            if g.snd_t[0] == 0 && g.snd_t[1] == 0 {
                g.snd_t[1] = 0xa;
                g.set_state_flags(3);
            }
        }
        if o.state & 8 != 0 {
            o.vy >>= 1;
        }
        if o.state & 0x20 != 0 {
            if o.state & 0x10 != 0 {
                o.vy >>= 1;
                o.state &= 0xef;
            } else {
                o.vy = o.vy.wrapping_mul(2);
                o.state |= 0x30;
            }
        } else if o.state & 0x10 != 0 {
            o.vy >>= 1;
            o.state &= 0xef;
        }
        if o.state & 4 != 0 {
            o.vx >>= 1;
        }
    } else if yh >= 0x68 {
        o.state = 0;
        g.bonus.o[k] = o;
        return;
    }
    o.x = o.x.wrapping_add(o.vx as u16);
    let xh = (o.x >> 8) as u8;
    if !(3..0xb7).contains(&xh) {
        o.state = 0;
    }
    g.bonus.o[k] = o;
}

/// decrement_or_reset_counter ($B664): the ball's spin, by its direction.
fn decrement_or_reset_counter(g: &mut Ik, k: usize) {
    let o = &mut g.bonus.o[k];
    if o.vx < 0 {
        o.spin = if o.spin == 0 { 8 } else { o.spin - 1 };
    } else {
        o.spin += 1;
        if o.spin >= 9 {
            o.spin = 0;
        }
    }
}

/// fill_table_from_callee ($BCC6): the balls, straight onto the shown screen.
fn fill_table_from_callee(g: &mut Ik) {
    let sh = g.shown;
    for k in (0..4).rev() {
        let o = g.bonus.o[k];
        if o.state == 0 {
            g.bonus.o[k].pos = None;
            continue;
        }
        let x = ((((o.x >> 8) as i32) << 1) | ((o.x >> 7) & 1) as i32) - 32;
        let y = (o.y >> 8) as i32 + 0x60;
        let keep = g.obj_on;
        g.obj_on = false;
        g.bob(sh, ball_src(o), x, y, 12, false);
        g.obj_on = keep;
        g.bonus.o[k].pos = Some((sh, x, y));
    }
}

/// decrement_global_until_negative ($BB64): each ball's shadow on the floor when it is low.
fn ball_shadows(g: &mut Ik) {
    let sh = g.shown;
    let tb = g.a.t("balls").to_vec();
    for k in (0..4).rev() {
        let o = g.bonus.o[k];
        if o.state == 0 {
            continue;
        }
        let d0 = 0xb5i32 - (o.y >> 8) as i32 - 0x60;
        if !(-12..8).contains(&d0) {
            continue;
        }
        let x = 2 * ((o.x >> 8) as i32) - 32;
        for r in 0..8i32 {
            let i = 0x28 + 2 * (r - d0);
            if !(0x28..0x40).contains(&i) {
                continue;
            }
            let m = u16::from_be_bytes([tb[i as usize], tb[i as usize + 1]]);
            for c in 0..16 {
                if m & (0x8000 >> c) != 0 {
                    let xx = x + c;
                    if (0..320).contains(&xx) {
                        g.scr[sh][(192 + r) as usize * 320 + xx as usize] |= 1;
                    }
                }
            }
        }
    }
}

/// decode_sprite_state_table ($B93E): a new ball now and then, faster as the round goes on.
fn decode_sprite_state_table(g: &mut Ik) {
    if g.bonus.hit {
        return;
    }
    if (g.bonus.accum >> 8) < 0x2c {
        g.bonus.accum = g.bonus.accum.wrapping_add(g.bonus.speed);
        return;
    }
    let Some(d1) = (0..4).rev().find(|&k| g.bonus.o[k].state == 0) else {
        return;
    };
    if g.bonus.speed_step >= 0x40 {
        g.bonus.done = true;
        return;
    }
    g.bonus.speed_step += 1;
    let tb = g.a.t("balls").to_vec();
    let av = g.ambient.v;
    let d4 = (g.vbl.rotate_right(4) ^ av[0]).wrapping_add(av[3]);
    let mut o = Obj {
        grav: g.bonus.gravity,
        floor: 0x55,
        y: (tb[(d4 & 7) as usize] as u16) << 8,
        state: 1,
        vx: g.bonus.speed as i16,
        ..Default::default()
    };
    if d4 & 0x80 == 0 {
        o.x = 4 << 8;
    } else {
        o.x = 0xb6 << 8;
        o.vx = o.vx.wrapping_neg();
    }
    g.bonus.accum = 0;
    let d2 = (g.bonus_lv[0] & 7) as usize;
    if (g.bonus.speed >> 8) < 1 || (g.bonus.speed & 0xff) < tb[16 + d2] as u16 {
        g.bonus.speed = g.bonus.speed.wrapping_add(8);
    }
    if g.bonus.gravity < 0x38 {
        g.bonus.gravity += 1;
    }
    let r = g.random() & 7;
    o.spin = r;
    o.anim = r;
    if d1 >= tb[8 + d2] as usize {
        o.state = 0x31;
        o.anim = 1;
        o.spin = 0;
        let i = (g.vbl.wrapping_add(av[6]) & 1) as usize;
        o.y = (tb[i] as u16) << 8;
    }
    g.bonus.o[d1] = o;
}
