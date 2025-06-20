use std::{
    collections::{hash_map::HashMap, HashSet},
    time::Instant,
};

use crate::particle::{Mass, Particle};
use bevy::{math::bounding::Aabb2d, prelude::*};

#[derive(Default, Resource, Debug)]
/// Quad tree with a vector of [Node]s and a hash map used to look up the index of the node that a entity is in in the tree
pub struct QuadTree {
    nodes: Vec<Node>,
    _hash_map: HashMap<Entity, usize>,
}

/// Nodes have a unique id, and store ids to their children
/// A nodes id is their index in the array of nodes in quadtree
#[derive(Clone, Copy, Debug)]
struct Node {
    bounds: Aabb2d,
    _parent_id: usize,
    id: usize,
    // This is the index of the first child, the rest will be just +1 +2 and +3
    children: Option<usize>,
    particle: Option<(Entity, Vec2, Mass)>,
    _center_of_mass: Vec2,
    _mass: f32,
    //    Node
    // +---+---+
    // | 1 | 2 |
    // +---+---+
    // | 3 | 4 |
    // +---+---+
}

impl Node {
    fn new(parent_id: usize, id: usize, bounds: Aabb2d) -> Self {
        let (_, _, mid) = get_max_min_center(&bounds);
        debug!("new node created id: {id} parent id: {parent_id}");
        Self {
            bounds,
            _parent_id: parent_id,
            id,
            children: None,
            particle: None,
            _center_of_mass: mid,
            _mass: 0.0,
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_none()
    }

    fn has_particle(&self) -> bool {
        self.particle.is_some()
    }

    fn _is_root_node(&self) -> bool {
        // because of how the quadtree is built, the 0th node should always be the root
        self.id == 0
    }

    fn contains(&self, pos: &Vec2) -> bool {
        contains(&self.bounds, pos)
    }
}

impl QuadTree {
    fn new(particles: &[(Entity, Vec2, Mass)]) -> Self {
        let mut node_vec = Vec::new();
        let hash_map: HashMap<Entity, usize> = HashMap::new();
        // we need to get a vec of Vec2 to make an aabb for the root node
        let positions: Vec<Vec2> = particles.iter().map(|(_, position, _)| *position).collect();
        let root_aabb = Aabb2d::from_point_cloud(Isometry2d::IDENTITY, &positions);
        let root_node = Node::new(0, 0, root_aabb);

        node_vec.push(root_node);

        Self {
            nodes: node_vec,
            _hash_map: hash_map,
        }
    }

    fn get_node_mut(&mut self, node_id: usize) -> &mut Node {
        if let Some(node) = self.nodes.get_mut(node_id) {
            return node;
        }

        error!("failed to retrive node at id {node_id} as it did not exist");
        panic!();
    }

    fn insert(&mut self, entity: Entity, position: Vec2, mass: Mass) {
        let mut id_to_subdivide: Option<usize> = None;

        for node in self.nodes.iter_mut().rev() {
            if node.is_leaf() && node.contains(&position) {
                if node.has_particle() {
                    id_to_subdivide = Some(node.id);
                } else {
                    node.particle = Some((entity, position, mass))
                };

                break;
            }
        }

        if let Some(id) = id_to_subdivide {
            self.subdivide(id);
        };
    }

    fn subdivide(&mut self, node_id: usize) -> &mut Self {
        debug!("subdividing node {node_id}");
        let (aabbs, particle) = {
            if let Some(node) = self.nodes.get_mut(node_id) {
                let (max, min, center) = get_max_min_center(&node.bounds);

                let aabbs = [
                    Aabb2d {
                        min: Vec2::new(min.x, center.y),
                        max: Vec2::new(center.x, max.y),
                    },
                    Aabb2d {
                        min: Vec2::new(center.x, center.y),
                        max: Vec2::new(max.x, max.y),
                    },
                    Aabb2d {
                        min: Vec2::new(min.x, min.y),
                        max: Vec2::new(center.x, center.y),
                    },
                    Aabb2d {
                        min: Vec2::new(center.x, min.y),
                        max: Vec2::new(max.x, center.y),
                    },
                ];
                // return the new aabbs and the the particle that may or may not have been there
                let particle = node.particle.take();
                (aabbs, particle)
            } else {
                error!(
                    "failed to subdivide node at id: {} beacuse it did not exist!",
                    node_id
                );
                panic!();
            }
        };

        let first_node_id = self.nodes.len();

        for (child_num, aabb) in aabbs.iter().enumerate() {
            let new_node = Node::new(node_id, first_node_id + child_num, *aabb);
            self.nodes.push(new_node);
        }

        self.get_node_mut(node_id).children = Some(first_node_id);

        if let Some(particle) = particle {
            let (entity, position, mass) = particle;
            self.insert(entity, position, mass);
        }
        self
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

    #[allow(dead_code)]
    fn visualize_from_root(&self) {
        self.visualize(0, 0, &mut HashSet::new());
    }

    fn visualize(&self, node: usize, indent: usize, visited: &mut HashSet<usize>) {
        if !visited.insert(node) {
            return;
        }

        let indent_string = "  │".repeat(indent);

        if let Some(current_node) = self.nodes.get(node) {
            let mut particle_string = "";
            let mut leaf_string = "";

            if current_node.has_particle() {
                particle_string = " [contains particle]"
            }

            if current_node.is_leaf() {
                leaf_string = " [is leaf]"
            }

            let id = current_node.id;
            println!("{indent_string} node {id}{leaf_string}{particle_string}");

            if let Some(children_start) = current_node.children {
                let children = [
                    children_start,
                    children_start + 1,
                    children_start + 2,
                    children_start + 3,
                ];

                for &child in children.iter() {
                    self.visualize(child, indent + 1, visited);
                }
            }
        }
    }
}

/// Checks if a aabb contains a point ( why is this not in the crate?? )
pub fn contains(aabb: &Aabb2d, pos: &Vec2) -> bool {
    aabb.min.x <= pos.x && aabb.min.y <= pos.y && aabb.max.x >= pos.x && aabb.max.y >= pos.y
}

fn get_max_min_center(aabb: &Aabb2d) -> (Vec2, Vec2, Vec2) {
    let min = aabb.min;
    let max = aabb.max;
    let center = (max + min) / 2.0;
    (max, min, center)
}

pub fn quadtree_system(
    mut commands: Commands,
    particles: Query<(Entity, &Transform, &Mass), With<Particle>>,
    quadtree: Option<ResMut<QuadTree>>,
) {
    let start_time = Instant::now();

    if particles.is_empty() {
        debug!("skipping quadtree construction as there are no particles");
        return;
    }

    let mut particles = particles
        .iter()
        .map(|(entity, transform, mass)| (entity, transform.translation.truncate(), *mass))
        .collect::<Vec<_>>();

    particles.sort_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap());

    let mut qt = QuadTree::new(&particles);

    for (entity, transform, mass) in &particles {
        qt.insert(*entity, *transform, *mass);
    }

    let finish_time = Instant::now();
    let time_taken = finish_time - start_time;
    info!("built quadtree in {}ms", time_taken.as_millis());

    if let Some(mut quadtree) = quadtree {
        *quadtree = qt;
    } else {
        commands.insert_resource(qt);
    }
}
