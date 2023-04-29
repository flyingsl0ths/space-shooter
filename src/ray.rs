use bevy::prelude::*;

pub type LineSegment = (Vec2, Vec2);

pub fn casted(s1: LineSegment, s2: Vec2) -> bool {
    let (p1, p2) = s1;
    let (p3, p4) = to_segment(s2);

    let den = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);

    // Ray and Target are parallel
    if den == 0. {
        return false;
    }

    let t = (((p1.x - p3.x) * (p3.y - p4.y)) - ((p1.y - p3.y) * (p3.x - p4.x)))
        / den;

    let u = -(((p1.x - p2.x) * (p1.y - p3.y))
        - ((p1.y - p2.y) * (p1.x - p3.x)))
        / den;

    t > 0. && t < 1. && u > 0.
}

fn to_segment(v: Vec2) -> LineSegment {
    let position = v.clone();
    let direction = v.normalize();

    (
        position,
        Vec2::new(position.x + direction.x, position.y + direction.y),
    )
}
