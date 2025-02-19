// src/objects.rs

use macroquad::prelude::*;

// Activate collision box highlighting.
const DEBUG_HIGHLIGHT: bool = true;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CollisionType {
    None,
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub enum Shape {
    Rectangle,
    Circle,
}

pub enum Appearance {
    Color { shape: Shape, color: Color },
    Texture(Texture2D),
}

impl Appearance {
    pub fn as_texture(&self) -> Option<Texture2D> {
        match self {
            Appearance::Texture(tex) => Some(tex.clone()),
            _ => None,
        }
    }
}

/// (For circles the size.x is used as its radius.)
pub struct GameObject {
    pub id: &'static str,
    pub position: Vec2,
    pub size: Vec2,
    pub appearance: Appearance,
    pub active: bool,
    pub removed: bool,
}

impl GameObject {
    pub fn new_with_color(id: &'static str, position: Vec2, size: Vec2, shape: Shape, color: Color) -> Self {
        Self {
            id,
            position,
            size,
            appearance: Appearance::Color { shape, color },
            active: true,
            removed: false,
        }
    }

    pub fn new_with_texture(id: &'static str, position: Vec2, size: Vec2, texture: Texture2D) -> Self {
        Self {
            id,
            position,
            size,
            appearance: Appearance::Texture(texture),
            active: true,
            removed: false,
        }
    }

    pub fn draw(&self) {
        if self.active && !self.removed {
            match &self.appearance {
                Appearance::Color { shape, color } => match shape {
                    Shape::Rectangle => {
                        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, *color)
                    }
                    Shape::Circle => {
                        // For a circle, use size.x as the radius.
                        draw_circle(self.position.x, self.position.y, self.size.x, *color)
                    }
                },
                Appearance::Texture(texture) => {
                    draw_texture(texture, self.position.x, self.position.y, WHITE);
                    if DEBUG_HIGHLIGHT {
                        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, Color::new(255.0, 0.0, 0.0, 0.5));
                    }
                }
            }
        }
    }

    pub fn mark_removed(&mut self) {
        self.removed = true;
        self.active = false;
    }

    /// For a circle a bounding box from its radius (size.x).
    pub fn bounds(&self) -> Rect {
        match &self.appearance {
            Appearance::Color { shape, .. } => match shape {
                Shape::Rectangle => Rect::new(self.position.x, self.position.y, self.size.x, self.size.y),
                Shape::Circle => Rect::new(
                    self.position.x - self.size.x,
                    self.position.y - self.size.x,
                    self.size.x * 2.0,
                    self.size.x * 2.0,
                ),
            },
            Appearance::Texture(_) => Rect::new(self.position.x, self.position.y, self.size.x, self.size.y),
        }
    }

    pub fn collision_type(&self, other: &GameObject) -> CollisionType {
        let rect_a = self.bounds();
        let rect_b = other.bounds();

        if !self.active && self.removed {
            return CollisionType::None;
        }

        if !rect_a.overlaps(&rect_b) {
            return CollisionType::None;
        }

        // Compute centers of both rectangles.
        let center_a = vec2(rect_a.x + rect_a.w / 2.0, rect_a.y + rect_a.h / 2.0);
        let center_b = vec2(rect_b.x + rect_b.w / 2.0, rect_b.y + rect_b.h / 2.0);
        let dx = center_b.x - center_a.x;
        let dy = center_b.y - center_a.y;
        let combined_half_width = (rect_a.w + rect_b.w) / 2.0;
        let combined_half_height = (rect_a.h + rect_b.h) / 2.0;
        let overlap_x = combined_half_width - dx.abs();
        let overlap_y = combined_half_height - dy.abs();

        if overlap_x < overlap_y {
            // Horizontal collision.
            if dx > 0.0 {
                // Other object is to the right.
                CollisionType::Right
            } else {
                CollisionType::Left
            }
        } else {
            // Vertical collision.
            if dy > 0.0 {
                // Other object is below.
                CollisionType::Bottom
            } else {
                CollisionType::Top
            }
        }
    }
}
