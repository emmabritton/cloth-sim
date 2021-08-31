use ggez::event::{EventHandler, MouseButton, KeyMods, KeyCode};
use ggez::{Context, GameError, GameResult, graphics};
use crate::models::{Rope, new_point, Point};
use crate::renderer::render;
use glam::Vec2;
use ggez::timer;
use crate::SCREEN_WIDTH;
use ggez::graphics::{Color, Text, TextFragment, PxScale};

pub const POINT_SIZE: f32 = 10.0;
const GRAVITY: f32 = 80.;

pub struct SimController {
    points: Vec<Point>,
    ropes: Vec<Rope>,
    simulate: bool,
    mouse_down: Option<(usize, Point)>,
    shift_down: bool,
    ctrl_down: bool
}

impl SimController {
    pub fn new() -> Self {
        SimController {
            points: vec![],
            ropes: vec![],
            simulate: false,
            mouse_down: None,
            shift_down: false,
            ctrl_down: false
        }
    }
}

impl SimController {
    pub fn setup(&mut self) {}
}

impl EventHandler<GameError> for SimController {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.simulate {
            let delta = timer::delta(ctx).as_secs_f32();

            self.points.iter_mut()
                .filter(|point| !point.borrow().locked)
                .for_each(|point| {
                    let mut point = point.borrow_mut();
                    let pos_before_update = point.current_pos;
                    let diff = point.current_pos - point.prev_pos;
                    point.current_pos += diff;
                    point.current_pos += Vec2::new(0., 1.) * GRAVITY * delta;
                    point.prev_pos = pos_before_update;
                });

            for _ in 0..4 {
                for rope in &self.ropes {
                    let centre = rope.end1.borrow_mut().current_pos.lerp(rope.end2.borrow_mut().current_pos, 0.5);
                    let dir = (rope.end1.borrow_mut().current_pos - rope.end2.borrow_mut().current_pos).normalize();
                    if !rope.end1.borrow_mut().locked {
                        rope.end1.borrow_mut().current_pos = centre + dir * rope.length / 2.;
                    }
                    if !rope.end2.borrow_mut().locked {
                        rope.end2.borrow_mut().current_pos = centre - dir * rope.length / 2.;
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        render(ctx, &self.ropes, &self.points)?;

        draw_text(
            ctx,
            &format!("{:.0}", timer::fps(ctx)),
            Vec2::new(SCREEN_WIDTH - 60., 0.),
            Color::RED,
            24.,
            false,
        );

        if self.shift_down {
            draw_text(ctx, "Delete", Vec2::new(10., 10.), Color::BLACK, 24., false);
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let cursor = Vec2::new(x, y);
        if button == MouseButton::Left {
            for (i, point) in self.points.iter().enumerate() {
                if point.borrow().current_pos.abs_diff_eq(cursor, POINT_SIZE * 2.) {
                    self.mouse_down = Some((i, point.clone()));
                    return;
                }
            }
        }
        self.mouse_down = None;
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let cursor = Vec2::new(x, y);
        if let Some((pidx, down_at)) = &self.mouse_down {
            if self.shift_down {
                let mut ropes_to_remove = vec![];
                for (idx, rope) in self.ropes.iter().enumerate() {
                    let point = self.points[*pidx].borrow();
                    if rope.end1.borrow().current_pos == point.current_pos || rope.end2.borrow().current_pos == point.current_pos {
                        ropes_to_remove.push(idx);
                    }
                }
                self.points.remove(*pidx);
                for rope in ropes_to_remove.iter().rev() {
                    self.ropes.remove(*rope);
                }
            } else if !self.ctrl_down {
                for point in &self.points {
                    if point.borrow().current_pos.abs_diff_eq(cursor, POINT_SIZE) {
                        self.ropes.push(Rope::new(down_at.clone(), point.clone()));
                    }
                }
            }
        } else {
            if button == MouseButton::Left && !self.shift_down && !self.ctrl_down {
                self.points.push(new_point(cursor, false));
            }
            if button == MouseButton::Right {
                for point in &self.points {
                    if point.borrow().current_pos.abs_diff_eq(cursor, POINT_SIZE) {
                        let state = point.borrow_mut().locked;
                        point.borrow_mut().locked = !state;
                    }
                }
            }
        }
        self.mouse_down = None;
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        if let Some((_, point)) = &self.mouse_down {
            if self.ctrl_down {
                point.borrow_mut().current_pos.x = x;
                point.borrow_mut().current_pos.y = y;
            }
        }
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        if keycode == KeyCode::RShift || keycode == KeyCode::LShift {
            self.shift_down = true;
        }
        if keycode == KeyCode::LControl || keycode == KeyCode::RControl || keycode == KeyCode::LWin || keycode == KeyCode::RWin {
            self.ctrl_down = true;
        }
        if keycode == KeyCode::Escape {
            ggez::event::quit(ctx);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        if keycode == KeyCode::Space {
            self.simulate = !self.simulate;
        }
        if keycode == KeyCode::R {
            self.points.clear();
            self.ropes.clear();
        }
        if keycode == KeyCode::RShift || keycode == KeyCode::LShift {
            self.shift_down = false;
        }
        if keycode == KeyCode::LControl || keycode == KeyCode::RControl || keycode == KeyCode::LWin || keycode == KeyCode::RWin {
            self.ctrl_down = false;
        }
        if keycode == KeyCode::G {
            self.simulate = false;
            self.points.clear();
            self.ropes.clear();
            let w = 26;
            let h = 26;
            let step = POINT_SIZE * 5.;
            for x in 0..w {
                for y in 0..h {
                    self.points.push(new_point(Vec2::new((x as f32 * step) + step,(y as f32 * step) + step), y == 0));
                }
            }
            for x in 1..w {
                for y in 1..h {
                    let l = x - 1;
                    let t = y - 1;
                    let first = l + t * w;
                    let above = l + y * w;
                    let left = x + t * w;
                    self.ropes.push(Rope::new(self.points[first].clone(), self.points[above].clone()));
                    self.ropes.push(Rope::new(self.points[first].clone(), self.points[left].clone()));
                }
            }
        }
    }
}

pub fn draw_text(
    ctx: &mut Context,
    text: &str,
    mut position: Vec2,
    color: Color,
    font_size: f32,
    centered: bool,
) {
    let text = Text::new(TextFragment {
        text: text.to_string(),
        color: Some(color),
        scale: Some(PxScale::from(font_size)),
        ..TextFragment::default()
    });
    if centered {
        position = Vec2::new(position.x - (text.width(ctx) as f32 / 2.), position.y);
    }
    graphics::draw(ctx, &text, (position, )).expect("couldn't draw");
}