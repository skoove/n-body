use std::{collections::hash_map::HashMap, time::Instant};

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
    _id: usize,
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
            _id: id,
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
        self._id == 0
    }

    fn contains(&self, pos: &Vec2) -> bool {
        contains(&self.bounds, pos)
    }
}

impl QuadTree {
    fn new(particles: &Vec<(Entity, Vec2, Mass)>) -> Self {
        let mut node_vec = Vec::with_capacity(particles.len() * 2);
        let hash_map: HashMap<Entity, usize> = HashMap::with_capacity(particles.len());
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
            _hash_map: hash_map,
        }
    }

    /// gives the child of a node given the parent and a number 0..=3
    fn get_child(&mut self, node_id: usize, child: usize) -> &mut Node {
        match child {
            0..=3 => (),
            _ => {
                error!("attempted to retrive the child {child}, should be within 1..=4");
                panic!()
            }
        }
        let child_id = self
            .get_node_mut(node_id)
            .children
            .expect("expected there to be children")
            + child;

        debug!("getting child {child} of node {node_id}, child id: {child_id}");

        if let Some(child) = self.nodes.get_mut(child_id) {
            return child;
        }

        error!("child {child} of node {node_id}, child id: {child_id} does not exist !!!");
        panic!();
    }

    /// returns the id of a nodes child given the parent id and a number 0..=3
    fn get_child_id(&self, node_id: usize, child_id: usize) -> usize {
        self.get_node(node_id).children.expect("expected children") + child_id
    }

    fn get_node_mut(&mut self, node_id: usize) -> &mut Node {
        if let Some(node) = self.nodes.get_mut(node_id) {
            return node;
        }

        error!("failed to retrive node at id {node_id} as it did not exist");
        panic!();
    }

    fn get_node(&self, node_id: usize) -> &Node {
        if let Some(node) = self.nodes.get(node_id) {
            return node;
        }

        error!("failed to retrive node at id {node_id} as it did not exist");
        panic!();
    }

    fn insert(
        &mut self,
        entity: Entity,
        position: Vec2,
        mass: Mass,
        target_node: usize,
    ) -> &mut Self {
        {
            let node = &mut self.get_node_mut(target_node);
            if node.is_leaf() && !node.has_particle() {
                node.particle = Some((entity, position, mass));
                return self;
            }

            if node.has_particle() && node.is_leaf() {
                self.subdivide(target_node);
            }
        }

        for child_id in 0..=3 {
            let child = self.get_child(target_node, child_id);
            if child.contains(&position) {
                debug!("child {child_id} of node {target_node} contains the particle.. inserting into node {}", self.get_child_id(target_node, child_id));
                self.insert(
                    entity,
                    position,
                    mass,
                    self.get_child_id(target_node, child_id),
                );
                return self;
            }

            debug!("child {child_id} of node {target_node} does not contain the particle");
            continue;
        }
        error!(
            "was not able to find a node to insert the particle into!! target id: {target_node}"
        );
        panic!();
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
                (aabbs, node.particle)
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
            if node.is_leaf() && !node.has_particle() {
                continue;
            }
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

fn get_max_min_center(aabb: &Aabb2d) -> (Vec2, Vec2, Vec2) {
    let min = aabb.min;
    let max = aabb.max;
    let center = (max + min) / 2.0;
    (max, min, center)
}

pub fn quadtree_system(
    mut commands: Commands,
    particles: Query<(Entity, &Transform, &Mass), With<Particle>>,
) {
    let start_time = Instant::now();

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

    let finish_time = Instant::now();
    let time_taken = finish_time - start_time;
    debug!("built quadtree in {}ms", time_taken.as_millis());

    commands.insert_resource(qt);
}
