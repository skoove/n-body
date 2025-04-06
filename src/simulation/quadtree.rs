use bevy::{math::bounding::Aabb2d, prelude::*, reflect::List};

use crate::particle::Particle;

#[derive(Resource)]
pub struct QuadTreeSettings {
    render: bool,
    capacity: u32,
}

impl Default for QuadTreeSettings {
    fn default() -> Self {
        Self {
            render: true,
            capacity: 1,
        }
    }
}

#[derive(Resource, Debug)]
pub struct QuadTree {
    bounds: Aabb2d,
    points: Vec<(Entity, Vec2)>,
    children: Option<[Box<QuadTree>; 4]>,
    //   Nodes
    // +---+---+
    // | 0 | 1 |
    // +---+---+
    // | 2 | 3 |
    // +---+---+
}

impl Default for QuadTree {
    fn default() -> Self {
        QuadTree {
            bounds: Aabb2d::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)),
            points: Vec::new(),
            children: None,
        }
    }
}

impl QuadTree {
    /// Create a new quadtree, this only actually creates the root node which is a
    /// bounding box of the points you dump in it. Returns [`None`] if there are no points
    /// in the [`Vec<Vec2>`]
    pub fn from_points(points: Vec<Vec2>) -> Option<Self> {
        // return early if there is nothing in there
        if points.iter().count() == 0 {
            return None;
        }
        Some(Self {
            bounds: Aabb2d::from_point_cloud(Isometry2d::IDENTITY, &points),
            points: Vec::new(),
            children: None,
        })
    }

    /// New tree from a bounding rect
    pub fn new(quad: Aabb2d) -> Self {
        Self {
            bounds: quad,
            points: Vec::new(),
            children: None,
        }
    }

    /// Subdivide the quadtree into four equal quadrents
    pub fn subdivide(&mut self, settings: &Res<QuadTreeSettings>) -> &mut Self {
        let min = self.bounds.min;
        let max = self.bounds.max;
        let mid = (max + min) / 2.0;

        let q0 = Aabb2d {
            min: Vec2::new(min.x, mid.y),
            max: Vec2::new(mid.x, max.y),
        };
        let q1 = Aabb2d {
            min: Vec2::new(mid.x, mid.y),
            max: Vec2::new(max.x, max.y),
        };
        let q2 = Aabb2d {
            min: Vec2::new(min.x, min.y),
            max: Vec2::new(mid.x, mid.y),
        };
        let q3 = Aabb2d {
            min: Vec2::new(mid.x, min.y),
            max: Vec2::new(max.x, mid.y),
        };

        self.children = Some([
            Box::new(QuadTree::new(q0)),
            Box::new(QuadTree::new(q1)),
            Box::new(QuadTree::new(q2)),
            Box::new(QuadTree::new(q3)),
        ]);

        for (entity, point) in self.points.drain(..).collect::<Vec<(Entity, Vec2)>>() {
            self.insert(settings, point, entity);
        }

        self
    }

    pub fn render(&self, gizmos: &mut Gizmos) {
        let min = self.bounds.min;
        let max = self.bounds.max;
        let center = (min + max) / 2.0;
        let size = max - min;
        let color = Color::hsva(120.0, 1.0, 1.0, 0.01);

        gizmos.rect_2d(Isometry2d::new(center, Rot2::IDENTITY), size, color);

        if let Some(children) = &self.children {
            children.iter().for_each(|child| {
                child.render(gizmos);
            });
        }
    }

    /// Insert an entity into the quadtree
    pub fn insert(
        &mut self,
        settings: &Res<QuadTreeSettings>,
        pos: Vec2,
        entity: Entity,
    ) -> &mut Self {
        if self.points.len() < settings.capacity as usize && self.children.is_none() {
            self.points.push((entity, pos));
            return self;
        }

        if self.children.is_none() {
            self.subdivide(settings);
        };

        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                if contains(&child.bounds, &pos) {
                    child.insert(settings, pos, entity);
                    break;
                }
            }
        }
        self
    }
}

/// Checks if a aabb contains a point ( why is this not in the crate?? )
pub fn contains(aabb: &Aabb2d, pos: &Vec2) -> bool {
    aabb.min.x < pos.x && aabb.min.y < pos.y && aabb.max.x > pos.x && aabb.max.y > pos.y
}

pub fn quadtree_system(
    mut gizmos: Gizmos,
    settings: Res<QuadTreeSettings>,
    mut commands: Commands,
    points: Query<(Entity, &Transform), With<Particle>>,
) {
    let mut particle_positions = Vec::<Vec2>::new();

    for (_, position) in points.iter() {
        let pos = position.translation.truncate();
        particle_positions.push(pos);
    }

    if let Some(mut qt) = QuadTree::from_points(particle_positions) {
        for (entity, position) in points.iter() {
            let pos = position.translation.truncate();
            qt.insert(&settings, pos, entity);
        }
        commands.insert_resource(qt);
    }
}
