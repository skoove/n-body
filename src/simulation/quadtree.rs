use std::collections::hash_map::HashMap;

use crate::particle::{self, Mass, Particle};
use bevy::{math::bounding::Aabb2d, prelude::*};

#[derive(Default, Resource, Debug)]
/// Quad tree with a vector of [Node]s and a hash map used to look up the index of the node that a entity is in in the tree
pub struct QuadTree {
    nodes: Vec<Node>,
    hash_map: HashMap<Entity, usize>,
}

/// Nodes have a unique id, and store ids to their children
/// A nodes id is their index in the array of nodes in quadtree
#[derive(Clone, Copy, Debug)]
struct Node {
    bounds: Aabb2d,
    parent_id: usize,
    id: usize,
    children: Option<[usize; 4]>,
    particle: Option<(Entity, Vec2, Mass)>,
    center_of_mass: Vec2,
    mass: f32,
    //    Node
    // +---+---+
    // | 0 | 1 |
    // +---+---+
    // | 2 | 3 |
    // +---+---+
}

impl Node {
    fn new(parent_id: usize, id: usize, bounds: Aabb2d) -> Self {
        let (_, _, mid) = get_max_min_mid(&bounds);
        debug!("new node created id: {id} parent id: {parent_id}");
        Self {
            bounds,
            parent_id,
            id,
            children: None,
            particle: None,
            center_of_mass: mid,
            mass: 0.0,
        }
    }
}

impl QuadTree {
    fn new(particles: &Vec<(Entity, Vec2, Mass)>) -> Self {
        let mut node_vec = Vec::new();
        let hash_map: HashMap<Entity, usize> = HashMap::new();
        // we need to get a vec of Vec2 to make an aabb for the root node
        let positions: Vec<Vec2> = particles
            .iter()
            .map(|(_, position, _)| position.clone())
            .collect();
        let root_aabb = Aabb2d::from_point_cloud(Isometry2d::IDENTITY, &positions);
        let root_node = Node::new(0, 0, root_aabb);

        node_vec.push(root_node);

        Self {
            nodes: node_vec,
            hash_map,
        }
    }

    fn insert(
        &mut self,
        entity: Entity,
        position: Vec2,
        mass: Mass,
        target_node: usize,
    ) -> &mut Self {
        {
            let node = &self.nodes[target_node];
        }
        self
    }

    fn subdivide(&mut self, node_id: usize) -> &mut Self {
        let (aabbs, particle) = {
            if let Some(node) = self.nodes.get_mut(node_id) {
                let (max, min, mid) = get_max_min_mid(&node.bounds);

                let aabbs = [
                    Aabb2d {
                        min: Vec2::new(min.x, mid.y),
                        max: Vec2::new(mid.x, max.y),
                    },
                    Aabb2d {
                        min: Vec2::new(mid.x, mid.y),
                        max: Vec2::new(max.x, max.y),
                    },
                    Aabb2d {
                        min: Vec2::new(min.x, min.y),
                        max: Vec2::new(mid.x, mid.y),
                    },
                    Aabb2d {
                        min: Vec2::new(mid.x, min.y),
                        max: Vec2::new(max.x, mid.y),
                    },
                ];
                (aabbs, node.particle)
            } else {
                error!(
                    "failed to subdivide node at id: {} beacuse it did not exist!",
                    node_id
                );
                panic!();
            }
        };

        let mut new_node_ids = Vec::new();

        for aabb in aabbs {
            let id = self.nodes.len() + 1;
            new_node_ids.push(id);
            let new_node = Node::new(node_id, id, aabb);
            self.nodes.push(new_node);
        }

        self.nodes[node_id].children = Some(new_node_ids.try_into().unwrap());

        if let Some(particle) = particle {
            let (entity, position, mass) = particle;
            self.insert(entity, position, mass, node_id);
        }
        return self;
    }

    const QUADTREE_COLOR: Color = Color::Srgba(Srgba {
        red: 0.0,
        green: 1.0,
        blue: 0.0,
        alpha: 0.01,
    });

    pub fn render(&self, gizmos: &mut Gizmos) {
        for node in &self.nodes {
            let b = &node.bounds;
            let center = (b.max + b.min) / 2.0;
            let size = b.max - b.min;
            gizmos.rect_2d(center, size, Self::QUADTREE_COLOR);
        }
    }
}

/// Checks if a aabb contains a point ( why is this not in the crate?? )
pub fn contains(aabb: &Aabb2d, pos: &Vec2) -> bool {
    aabb.min.x <= pos.x && aabb.min.y <= pos.y && aabb.max.x >= pos.x && aabb.max.y >= pos.y
}

fn get_max_min_mid(aabb: &Aabb2d) -> (Vec2, Vec2, Vec2) {
    let min = aabb.min;
    let max = aabb.max;
    let mid = (max + min) / 2.0;
    (max, min, mid)
}

pub fn quadtree_system(
    mut gizmos: Gizmos,
    mut commands: Commands,
    particles: Query<(Entity, &Transform, &Mass), With<Particle>>,
) {
    if particles.is_empty() {
        debug!("skipping quadtree construction as there are no particles");
        return;
    }

    let particles = particles
        .iter()
        .map(|(entity, transform, mass)| (entity, transform.translation.truncate(), mass.clone()))
        .collect();

    let mut qt = QuadTree::new(&particles);

    for (entity, transform, mass) in particles {
        qt.insert(entity, transform, mass, 0);
    }

    commands.insert_resource(qt);
}
