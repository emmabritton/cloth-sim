use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::{DrawMode, Color};
use crate::models::{Rope, Point};
use glam::Vec2;
use crate::controller::POINT_SIZE;

const TOLERANCE: f32 = 0.6;

pub fn render(ctx: &mut Context, ropes: &[Rope], points: &[Point]) -> GameResult {
    graphics::clear(ctx, Color::WHITE);

    if !points.is_empty() {
        let mut builder = graphics::MeshBuilder::new();
        for rope in ropes {
            builder.line(&[rope.end1.borrow().current_pos, rope.end2.borrow().current_pos], POINT_SIZE * 0.25, Color::BLACK)?;
        }

        for point in points {
            builder.circle(DrawMode::fill(), point.borrow().current_pos, POINT_SIZE, TOLERANCE, get_point_color(point.borrow().locked))?;
        }

        let mesh = builder.build(ctx)?;
        graphics::draw(ctx, &mesh, (Vec2::new(0., 0.), ))?;
    }

    Ok(())
}

fn get_point_color(locked: bool) -> Color {
    if locked {
        Color::RED
    } else {
        Color::BLACK
    }
}