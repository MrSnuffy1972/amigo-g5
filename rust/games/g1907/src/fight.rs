//! The fighters: joystick to move, the computer fighters' choices, the move animations, hit detection and scoring.
//! Moves are table driven as in the original: each move is a run of animation cursors, and each cursor gives a
//! frame, a step and flags (the `anim` table).

use crate::ik::{Ik, NF};

// Offsets of the original's small tables inside the extracted blocks (named after the original's labels).
const AN_WIDTH: usize = 0x00; // d0_92A2 frame widths
const AN_STUN: usize = 0x50; // d0_92F2 get-up time by level
const AN_START: usize = 0x568; // d0_980A first cursor of each move
const AN_FRAME: usize = 0x592; // d0_9834 frame of each cursor
const AN_STEP: usize = 0x686; // d0_9928 step of each cursor
const AN_FLAGS: usize = 0x77A; // d0_9A1C flags of each cursor
const HT_FRAMES: usize = 0x04; // $74B0 the attacking frames (10)
const HT_DX: usize = 0x0F; // $74BB
const HT_MASK: usize = 0x19; // $74C5
const HT_REACH: usize = 0x23; // $74CF
const HT_PTS_B: usize = 0x2D; // $74D9
const HT_PTS_A: usize = 0x37; // $74E3
const HT_GFX_B: usize = 0x41; // $74ED
const HT_GFX_A: usize = 0x4B; // $74F7
const HT_T_B: usize = 0x55; // $7501
const HT_T_A: usize = 0x5F; // $750B
const HT_ZONE: usize = 0x69; // $7515
const HT_REACT: usize = 0x70; // $751C
const HT_CLASS: usize = 0x98; // $7544
const HT_BODY: usize = 0xE1; // $758D
const AI_DEF: usize = 0x00; // d0_6CA2
const AI_DEF2: usize = 0x06; // d0_6CA8
const AI_ACT: usize = 0x0C; // d0_6CAE
const AI_LV: usize = 0x12; // d0_6CB4
const AI_AWAY: usize = 0x18; // d0_6CBA
const AI_HALF: usize = 0x20; // $72C2
const AI_COUNTER: usize = 0x30; // d0_6CD2
const AI_DOWN: usize = 0x54; // d0_6CF6
const AI_FAR_AWAY: usize = 0x61; // d0_6D03
const AI_NEAR_AWAY: usize = 0x6D; // d0_6D0F
const AI_FAR: usize = 0x79; // d0_6D1B
const AI_BLOCK: usize = 0x85; // d0_6D27
const AI_NEAR: usize = 0xC5; // d0_6D67
const AI_DIST: usize = 0x131; // d0_6DD3

impl Ik {
    fn an(&self, o: usize, i: usize) -> u8 {
        self.a.t("anim")[o + i]
    }
    fn ht(&self, o: usize, i: usize) -> u8 {
        self.a.t("hit")[o + i]
    }
    fn ai(&self, o: usize, i: usize) -> u8 {
        self.a.t("ai")[o + i]
    }

    /// compute_state_flags ($6E02): where fighter `a` stands against `b`: the side and facing combination (bit 2 = a
    /// is to the right, bit 1 = a faces left, bit 0 = b faces left) and the distance between their middles.
    pub fn compute_state_flags(&mut self, a: usize, b: usize) -> u8 {
        let mid = |s: &Ik, i: usize| -> u8 {
            if s.facing[i] != 0 {
                s.x[i].wrapping_add(0x28 - 0xc)
            } else {
                s.x[i].wrapping_add(0xc)
            }
        };
        let d = mid(self, a).wrapping_sub(mid(self, b)) as i8;
        let right = (d >= 0) as u8;
        self.sides = (((right << 1) | self.facing[a]) << 1) | self.facing[b];
        self.dist = d.unsigned_abs();
        self.dist4 = self.dist >> 2;
        self.sides
    }

    /// decode_byte_stream ($6724): the players' joysticks become the moves they ask for.
    #[allow(clippy::needless_range_loop)]
    pub fn decode_byte_stream(&mut self) {
        let pa = self.a.t("pair_a").to_vec();
        let pb = self.a.t("pair_b").to_vec();
        let behind = self.a.t("behind").to_vec();
        for d7 in 0..2usize {
            if self.human[d7] == 0 {
                continue;
            }
            let d0 = if self.phase == 3 {
                0
            } else {
                let j = self.joy[d7] as usize;
                self.joy_tmp = j as u8;
                let mut special = false;
                if self.hold[d7] == 0 {
                    let o = d7 ^ 1;
                    if self.attacking[o] != 0 {
                        let s = self.compute_state_flags(o, pa[o] as usize);
                        let mut hit = behind[s as usize] != 0;
                        if !hit {
                            let s = self.compute_state_flags(d7, pb[d7] as usize);
                            hit = behind[s as usize] != 0;
                        }
                        special = hit && self.dist4 < 8;
                    }
                }
                if special {
                    self.a.t(if self.facing[d7] != 0 {
                        "input_ld"
                    } else {
                        "input_rd"
                    })[j]
                } else if self.facing[d7] ^ self.turned[d7] == 0 {
                    self.a.t("input_r")[j]
                } else {
                    self.a.t("input_l")[j]
                }
            };
            self.want[d7] = d0;
            if d0 != 0 {
                self.last_move[d7] = d0;
            }
        }
    }

    fn sel(&self, i: usize) -> u8 {
        let a = self.a.t("sel_a");
        let b = self.a.t("sel_b");
        if i < a.len() {
            a[i]
        } else {
            b[(i - a.len()).min(b.len() - 1)]
        }
    }

    /// select_table_index ($6942): which opponent the computer fighter `me` goes for.
    fn select_table_index(&mut self, me: usize) {
        let pa = self.a.t("pair_a").to_vec();
        let pb = self.a.t("pair_b").to_vec();
        let all_ai = |s: &Ik| s.humans == 0 || s.humans == 2;
        let d1 = if all_ai(self) {
            let mut d1 = pa[me] as usize;
            let d0 = self.points[d1];
            if (d0 as i8) < self.points[pb[me] as usize] as i8 {
                d1 = pb[me] as usize;
            }
            if self.hit[d1] != 0 {
                d1 = pa[d1] as usize;
            }
            d1
        } else {
            let d0 = me + 3 * self.human[1] as usize + self.skill[2] as usize;
            let mut d1 = self.sel(d0) as usize;
            if self.hit[d1] != 0 {
                d1 = self.sel(6 + d0) as usize;
            }
            d1
        };
        self.target = d1.min(2);
    }

    /// self_4 ($6DDE): a random number below `n`.
    fn self_4(&mut self, n: u8) -> usize {
        loop {
            let r = self.random() & 0x1f;
            if r < n {
                return r as usize;
            }
            let r = r & 0xf;
            if r < n {
                return r as usize;
            }
        }
    }

    /// evaluate_game_state_flags ($69F0): the computer fighter `cur` decides its next move.
    pub fn evaluate_game_state_flags(&mut self) {
        if self.phase != 1 && self.demo == 0 {
            return;
        }
        let me = self.cur;
        if self.human[me] != 0 {
            return;
        }
        self.select_table_index(me);
        let t = self.target;
        self.compute_state_flags(me, t);
        if self.mv[me] != 0 && self.flags[me] & 0x10 == 0 {
            return;
        }
        let mv: Option<u8>;
        // against an attack: a counter (block or dodge) from a table of the attack and the sides
        let attack_now: bool;
        if self.attacking[t] != 0 && self.hold[t] < 4 {
            if self.dist4 >= 9 {
                attack_now = self.may_act();
                mv = None;
            } else {
                let d2 = self.mv[t] as usize;
                let d0 = self.ai(AI_COUNTER, d2);
                if (d0 as i8) < 0 {
                    attack_now = self.may_act();
                    mv = None;
                } else {
                    let mut res = None;
                    if matches!(d2, 5 | 6 | 2) {
                        let r = self.random();
                        let sk = self.skill[0];
                        let pass = (sk as i8) < 0 || r >= self.ai(AI_DEF, sk as usize);
                        if pass {
                            // the random byte decides the low bits too (as the original, which reuses it)
                            let mut ds = self.sides;
                            if ds >= 4 {
                                ds = (ds & 3) ^ 3;
                            }
                            let i =
                                ((((ds << 1).wrapping_add(d0)) << 3).wrapping_add(r & 3)) as usize;
                            res = Some(self.ai(AI_BLOCK, i));
                        }
                    }
                    if res.is_none() {
                        let r = self.random();
                        let sk = self.skill[0];
                        let pass = (sk as i8) < 0 || r >= self.ai(AI_DEF2, sk as usize);
                        if pass {
                            let mut ds = self.sides;
                            if ds >= 4 {
                                ds = (ds & 3) ^ 3;
                            }
                            let i =
                                ((((ds << 1).wrapping_add(d0)) << 3).wrapping_add(r & 7)) as usize;
                            res = Some(self.ai(AI_BLOCK, i));
                        }
                    }
                    match res {
                        Some(m) => {
                            mv = Some(m);
                            attack_now = false;
                        }
                        None => {
                            attack_now = self.may_act();
                            mv = None;
                        }
                    }
                }
            }
        } else if self.idle[me] >= 0x30 {
            attack_now = true;
            mv = None;
        } else {
            attack_now = self.may_act();
            mv = None;
        }
        let mv = match mv {
            Some(m) => m,
            None if attack_now => self.choose_attack(),
            None => return,
        };
        self.want[me] = mv;
        self.reset_animation_state(me);
        self.want[me] = 0;
    }

    /// l0_6B76: whether the computer fighter acts this step, by its skill.
    fn may_act(&mut self) -> bool {
        let sk = self.skill[1];
        if (sk as i8) < 0 {
            return false;
        }
        let r = self.random();
        r >= self.ai(AI_ACT, sk as usize)
    }

    /// l0_6B9E: which attack, by the target's state, sides and distance.
    fn choose_attack(&mut self) -> u8 {
        let t = self.target;
        if self.hit[t] != 0 {
            let d1 = self.self_4(0xc);
            return self.ai(AI_DOWN, d1);
        }
        if self.ai(AI_AWAY, self.sides as usize) != 0 {
            let d1 = self.self_4(0xc);
            if self.dist4 >= 0xc {
                self.ai(AI_FAR_AWAY, d1)
            } else if self.dist4 >= 8 {
                0x0e
            } else {
                self.ai(AI_NEAR_AWAY, d1)
            }
        } else if self.dist4 >= 0xa {
            let d1 = self.self_4(0xc);
            self.ai(AI_FAR, d1)
        } else {
            let t652 = self.ai(AI_DIST, self.dist4 as usize);
            let n = self.ai(AI_LV, self.level as usize);
            let d1 = self.self_4(n);
            self.ai(AI_NEAR, (t652 as usize + d1) & 0xff)
        }
    }

    /// initialize_sprite_structure ($9230): starts move `m` for fighter `i` from its first cursor.
    pub fn initialize_sprite_structure(&mut self, i: usize, m: u8) {
        self.idle[i] = m;
        self.start_move(i, m);
    }

    fn start_move(&mut self, i: usize, m: u8) {
        self.mv[i] = m;
        self.start_cursors(i, m);
    }

    fn start_cursors(&mut self, i: usize, m: u8) {
        let c = self.an(AN_START, m as usize);
        self.cursor[i] = c;
        self.next_cursor[i] = c;
        self.same[i] = 1;
        self.rev[i] = 0;
        self.reversible[i] = 0;
        self.hold[i] = 0;
        self.turned[i] = 0;
        self.flags[i] = 0;
        self.prev_flags[i] = 0;
        self.frozen[i] = 0;
    }

    /// reset_animation_state ($91B0): a move has ended: start the one asked for, or stand (and now and then a
    /// computer fighter fidgets: move $20).
    pub fn reset_animation_state(&mut self, i: usize) {
        let w = self.want[i];
        if w != 0 {
            self.initialize_sprite_structure(i, w);
            return;
        }
        let c = self.cur;
        self.idle[c] = self.idle[c].wrapping_add(1);
        let fidget = if self.idle[c] & 0x80 != 0 {
            let r = self.random() & 0x7f;
            r < 8
        } else if self.round >= 5 {
            false
        } else {
            self.keys[0] == 0x14
        };
        if !fidget {
            self.start_move(i, 0);
            return;
        }
        self.mv[i] = 0x20;
        self.idle[i] = (self.random() & 0x3f) + 1;
        self.start_cursors(i, 0x20);
    }

    /// process_player_state_flags ($92F8): one animation step of fighter `cur`.
    pub fn process_player_state_flags(&mut self) {
        let d2 = self.cur;
        if self.phase != 6 && self.frozen[d2] != 0 {
            if self.hit_t2[d2] != 0 {
                return;
            }
            let get_up = if self.human[d2] == 0 {
                let lim = self.an(AN_STUN, self.level as usize);
                self.stun[d2] <= lim
            } else {
                self.want[d2] != 0 || self.stun[d2] == 0
            };
            if !get_up {
                return;
            }
            let m = match self.frame_x[d2] {
                0x27 => 0x23,
                0x2b => 0x24,
                _ => 0x25,
            };
            self.want[d2] = 0;
            self.hit[d2] = 0;
            self.initialize_sprite_structure(d2, m);
            return;
        }
        if self.phase != 6 {
            // l0_93B2
            self.facing[d2] &= 1;
            self.human[d2] &= 1;
            if self.human[d2] != 0 {
                if self.want[d2] != self.mv[d2] {
                    self.same[d2] = 0;
                }
                if self.same[d2] == 0 && self.reversible[d2] != 0 {
                    self.rev[d2] = 1;
                }
                if self.rev[d2] != 0 {
                    self.play_back(d2);
                    return;
                }
            }
        }
        loop {
            match self.step_forward(d2) {
                Some(true) => continue,
                _ => return,
            }
        }
    }

    /// l0_942C onwards: the next cursor of the move; `Some(true)` = run l0_93B2 again (a held move let go).
    fn step_forward(&mut self, d2: usize) -> Option<bool> {
        self.cursor[d2] = self.next_cursor[d2];
        if self.mv[d2] == 0 {
            self.reset_animation_state(d2);
        }
        loop {
            let m = self.mv[d2] as usize;
            if self.cursor[d2] >= self.an(AN_START + 1, m) {
                self.reset_animation_state(d2);
                continue;
            }
            break;
        }
        let c = self.cursor[d2] as usize;
        self.frame_x[d2] = self.an(AN_FRAME, c).min(0x57);
        self.prev_flags[d2] = self.flags[d2];
        let f = self.an(AN_FLAGS, c);
        self.flags[d2] = f;
        self.reversible[d2] = f & 4;
        self.attacking[d2] = f & 0x40;
        if self.human[d2] != 0 && self.phase != 6 {
            if f & 8 != 0 {
                if self.want[d2] == self.mv[d2] {
                    if self.hold[d2] == 0 {
                        self.update_byte_with_table_offset(d2);
                    }
                    let c = self.cur;
                    if self.hold[c] < 0x78 {
                        self.hold[c] += 1;
                    }
                    return None;
                }
                if self.hold[d2] != 0 {
                    self.hold[d2] = 0;
                    self.rev[d2] = 0;
                    self.reversible[d2] = 0;
                    self.next_cursor[d2] = self.cursor[d2].wrapping_add(1);
                    // l0_93B2 again
                    self.facing[d2] &= 1;
                    self.human[d2] &= 1;
                    if self.human[d2] != 0 {
                        if self.want[d2] != self.mv[d2] {
                            self.same[d2] = 0;
                        }
                        if self.same[d2] == 0 && self.reversible[d2] != 0 {
                            self.rev[d2] = 1;
                        }
                        if self.rev[d2] != 0 {
                            self.play_back(d2);
                            return None;
                        }
                    }
                    return Some(true);
                }
            }
            if f & 0x10 != 0 && self.same[d2] == 0 {
                let m = self.mv[d2];
                if m == 3 || m == 7 || self.want[d2] != 0 {
                    if m == 7 {
                        let s: u8 = if self.facing[d2] != 0 { 3 } else { 0xfd };
                        self.x[d2] = self.x[d2].wrapping_add(s);
                    }
                    self.reset_animation_state(d2);
                    // falls into l0_944C with the new move
                    return self.after_reset(d2);
                }
            }
        }
        self.finish_step(d2, f);
        None
    }

    /// l0_9446 then l0_944C: the move was replaced; show its first cursor.
    fn after_reset(&mut self, d2: usize) -> Option<bool> {
        loop {
            let m = self.mv[d2] as usize;
            if self.cursor[d2] >= self.an(AN_START + 1, m) {
                self.reset_animation_state(d2);
                continue;
            }
            break;
        }
        let c = self.cursor[d2] as usize;
        self.frame_x[d2] = self.an(AN_FRAME, c).min(0x57);
        self.prev_flags[d2] = self.flags[d2];
        let f = self.an(AN_FLAGS, c);
        self.flags[d2] = f;
        self.reversible[d2] = f & 4;
        self.attacking[d2] = f & 0x40;
        self.finish_step(d2, f);
        None
    }

    /// l0_95E6: turn, front fighter, knocked down, the step, and the next cursor.
    fn finish_step(&mut self, d2: usize, f: u8) {
        if f & 2 != 0 {
            self.facing[d2] ^= 1;
            self.turned[d2] = 1;
        }
        if f & 1 != 0 && d2 < 2 {
            self.front = d2 as u8;
        }
        if f & 0x20 != 0 {
            self.frozen[d2] = 1;
            self.extra[d2] = 0;
        }
        self.update_byte_with_table_offset(d2);
        self.next_cursor[d2] = self.cursor[d2].wrapping_add(1);
    }

    /// l0_9662: a held move let go plays back to its start.
    fn play_back(&mut self, d2: usize) {
        self.cursor[d2] = self.cursor[d2].wrapping_sub(1);
        let m = self.mv[d2] as usize;
        if self.cursor[d2] < self.an(AN_START, m) {
            self.reset_animation_state(d2);
            self.after_reset(d2);
            return;
        }
        let c = self.cursor[d2] as usize;
        self.frame_x[d2] = self.an(AN_FRAME, c);
        let left = self.rev[d2] ^ self.facing[d2];
        self.update_byte_table_entry(d2, left != 0, c);
    }

    /// update_byte_with_table_offset ($96C6).
    fn update_byte_with_table_offset(&mut self, d2: usize) {
        let c = self.cursor[d2] as usize;
        let left = self.facing[d2] != 0;
        self.update_byte_table_entry(d2, left, c);
    }

    /// update_byte_table_entry ($96D8): moves the fighter by the cursor's step and keeps it on the floor.
    fn update_byte_table_entry(&mut self, d2: usize, left: bool, c: usize) {
        let step = self.an(AN_STEP, c);
        let wd = |s: &Ik| s.an(AN_WIDTH, s.frame_x[d2] as usize);
        let (t652, t653);
        if left {
            self.x[d2] = self.x[d2].wrapping_sub(step);
            t653 = self.x[d2].wrapping_add(0x28);
            t652 = t653.wrapping_sub(wd(self));
        } else {
            self.x[d2] = self.x[d2].wrapping_add(step);
            t652 = self.x[d2];
            t653 = self.x[d2].wrapping_add(wd(self));
        }
        if t652 <= 0x11 {
            if !left {
                self.x[d2] = 0x11;
            } else {
                let v = 0x11i32 + wd(self) as i32 - 0x28;
                self.x[d2] = v.max(0) as u8;
            }
        } else if t653 > 0xae {
            self.x[d2] = if !left { 0xae - wd(self) } else { 0x86 };
        }
        if self.x[d2] >= 0xd0 {
            self.x[d2] = 0x10;
        }
    }

    /// set_up_three_fighters ($717E): the players' moves, then each fighter's choice and animation step.
    pub fn set_up_three_fighters(&mut self) {
        self.decode_byte_stream();
        self.nfighters = 3;
        for c in 0..NF {
            self.cur = c;
            self.evaluate_game_state_flags();
            self.process_player_state_flags();
        }
    }

    /// set_up_three_fighters_alt ($7172).
    pub fn set_up_three_fighters_alt(&mut self) {
        self.nfighters = 3;
        for c in 1..NF {
            self.cur = c;
            self.evaluate_game_state_flags();
            self.process_player_state_flags();
        }
    }

    /// clear_byte_table_and_search ($7066): finds the attacks landing this step (pairs of attacker and target in a
    /// fixed order) and scores them.
    pub fn clear_byte_table_and_search(&mut self) {
        for d2 in 0..NF {
            self.held_copy[d2] = self.hold[d2];
            self.got[d2] = 0;
        }
        let rm = self.a.t("rand_misc").to_vec();
        let mut k: i32 = 5;
        while k >= 0 {
            let ku = k as usize;
            let d1 = rm[6 + ku] as usize;
            let found = (0..10)
                .rev()
                .find(|&d2| self.frame_x[d1] == self.ht(HT_FRAMES, d2));
            if let Some(d2) = found {
                if (self.first_hit as i8) < 0 {
                    self.first_hit = d1 as u8;
                }
                if self.held_copy[d1] < 2 {
                    self.cur_ai = d1;
                    self.atk = d2 as u8;
                    let t = rm[0xC + ku] as usize;
                    self.target = t;
                    if self.hit[t] == 0 {
                        self.decode_sprite_attribute();
                        if self.hit[t] != 0 {
                            k = rm[ku] as i32;
                        }
                    }
                }
            }
            k -= 1;
        }
    }

    /// decode_sprite_attribute ($72D2): does attacker `cur_ai`'s attack `atk` reach `target`'s body? If so, the
    /// target is hit: points, the score pop-up, the reaction move.
    fn decode_sprite_attribute(&mut self) {
        let me = self.cur_ai;
        let t = self.target;
        let mut cls = self.ht(HT_CLASS, self.frame_x[t] as usize);
        if (cls as i8) < 0 {
            return;
        }
        if cls == 0x17 && self.atk == 2 {
            cls = 0x23;
        }
        let side = self.compute_state_flags(me, t) & 3;
        if cls == 0x13 && (self.sides == 0 || self.sides == 7) {
            cls = 0;
        }
        let atk = self.atk as usize;
        let reach = self.ht(HT_REACH, atk);
        let ap = if self.facing[me] != 0 {
            self.x[me]
                .wrapping_add(0x28)
                .wrapping_sub(self.ht(HT_DX, atk))
        } else {
            self.x[me].wrapping_add(self.ht(HT_DX, atk))
        };
        for k in 0..6usize {
            if self.ht(HT_ZONE, k) & self.ht(HT_MASK, atk) == 0 {
                continue;
            }
            let idx = (cls as usize * 6 + k) & 0xff;
            let off = self.ht(HT_BODY, idx);
            if (off as i8) < 0 {
                continue;
            }
            let bp = if self.facing[t] != 0 {
                self.x[t].wrapping_add(0x28).wrapping_sub(off)
            } else {
                self.x[t].wrapping_add(off)
            };
            let d = match side {
                1 => ap.wrapping_sub(bp),
                0 => ap.wrapping_sub(bp).wrapping_add(reach),
                2 => bp.wrapping_sub(ap),
                _ => bp.wrapping_sub(ap).wrapping_add(reach),
            };
            if (d as i8) < 0 || d > reach {
                continue;
            }
            // a hit
            let full = self.ai(AI_HALF, self.sides as usize) != 0;
            let (pts, gfx, tm, st) = if full {
                (
                    self.ht(HT_PTS_A, atk),
                    self.ht(HT_GFX_A, atk),
                    self.ht(HT_T_A, atk),
                    1,
                )
            } else {
                (
                    self.ht(HT_PTS_B, atk),
                    self.ht(HT_GFX_B, atk),
                    self.ht(HT_T_B, atk),
                    2,
                )
            };
            self.points_to_add = pts;
            self.hit_gfx[t] = gfx;
            self.hit_t2[t] = tm;
            self.hit_t[t] = tm.wrapping_sub(0x19);
            self.hit[t] = st;
            self.half_add = st;
            self.hit_by[t] = me as u8;
            self.hit_x[t] = self.x[t].wrapping_add(0xc);
            self.hit_y[t] = (t as u8) * 4 + 0x79;
            self.stun[t] = self.a.t("rand_misc")[0x12 + self.level as usize];
            self.scored = 0;
            if me < 2 {
                self.front = me as u8;
            }
            self.cur = me;
            self.got[me] = 1;
            self.update_bcd_counters_and_recurse();
            self.update_global_and_call_routine();
            let d1 = atk * 4 + side as usize;
            let r = self.ht(HT_REACT, d1);
            self.extra[t] = r >> 7;
            let mut ex = self.x[t].wrapping_add(4);
            if self.facing[t] != 0 {
                ex = ex.wrapping_add(0x12);
            }
            self.extra_x[t] = ex;
            self.want[t] = r & 0x7f;
            self.cur = t;
            self.initialize_sprite_structure(t, r & 0x7f);
            return;
        }
    }

    /// update_three_fighters ($5E92): draws the blue fighter, then the one behind, then the one in front.
    pub fn update_three_fighters(&mut self) {
        let f = (self.front & 1) as usize;
        for c in [2, f ^ 1, f] {
            self.cur = c;
            self.dispatch_via_indexed_table();
        }
    }

    /// dispatch_via_indexed_table ($8D76): draws fighter `cur` and, while it is hit, its score pop-up, the stars
    /// and the extra effect.
    pub fn dispatch_via_indexed_table(&mut self) {
        let i = self.cur & 3;
        if i >= NF {
            return;
        }
        self.draw_fighter(i);
        let fr = self.frame_x[i];
        if (0x24..=0x2f).contains(&fr) {
            self.draw_fighter_extra_pair(i);
            self.increment_counter_and_recurse(i, fr);
            self.draw_fighter_extra_pair_late(i, fr);
        }
    }

    /// draw_fighter_extra_pair ($8C90): the points the hit scored, over the target.
    fn draw_fighter_extra_pair(&mut self, i: usize) {
        if self.hit_t[i] == 0 {
            return;
        }
        let a = 0x22EAC + 0x600 + self.hit_gfx[i] as u32 * 4 + self.hit_by[i] as u32 * 0x640;
        let s = crate::ik::Bob::at(a);
        let (x, y) = (self.hit_x[i] as i32, self.hit_y[i] as i32);
        let b = self.back;
        self.bob2(b, s, x, y, 9, true);
        self.bob2(b, s.right(1), x + 8, y, 9, true);
    }

    /// increment_counter_and_recurse ($8BD6): the stars over a fighter knocked down.
    fn increment_counter_and_recurse(&mut self, i: usize, fr: u8) {
        if self.hit_t2[i] == 0 {
            return;
        }
        self.hit_anim[i] += 1;
        if self.hit_anim[i] >= 5 {
            self.hit_anim[i] = 0;
        }
        let mut d3 = match fr {
            0x27 => 0,
            0x2b => 2,
            0x2f => 4,
            _ => return,
        };
        if self.facing[i] != 0 {
            d3 += 6;
        }
        let t = self.a.t("hit_gfx").to_vec();
        let w = |o: usize| u16::from_be_bytes([t[o], t[o + 1]]) as i32;
        let l = |o: usize| u32::from_be_bytes([t[o], t[o + 1], t[o + 2], t[o + 3]]);
        let x = self.x[i] as i32 + w(d3);
        let y = w(12 + d3);
        let k = 4 * self.hit_anim[i] as u32;
        let s1 = crate::ik::Bob::at(l(24 + 2 * d3) + k);
        let s2 = crate::ik::Bob::at(l(48 + 2 * d3) + k);
        let b = self.back;
        self.bob2(b, s1, x, y, 12, true);
        self.bob2(b, s2, x + 8, y, 12, true);
    }

    /// draw_fighter_extra_pair_late ($8CFE).
    fn draw_fighter_extra_pair_late(&mut self, i: usize, fr: u8) {
        if self.extra[i] == 0 || !(0x28..0x2b).contains(&fr) {
            return;
        }
        let s = crate::ik::Bob::at(0x23260 + 0x600 + (fr as u32 - 0x28) * 4);
        let x = self.extra_x[i] as i32;
        let b = self.back;
        self.bob2(b, s, x, 0x83, 24, true);
        self.bob2(b, s.right(1), x + 8, 0x83, 24, true);
    }
}
