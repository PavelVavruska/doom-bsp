use std::ptr::NonNull;

use crate::player::Player;
use crate::space::line::Line;
use crate::space::subsector::Subsector;
use crate::space::vec2d::Vec2d;

pub enum TreeItem {
    Node,
    Subsector,
    Line,
}

pub struct Node {
    pub line: Line,
    pub left_node: Option<Box<Node>>,
    pub right_node: Option<Box<Node>>,
    pub left_subsector: Option<Box<Subsector>>,
    pub right_subsector: Option<Box<Subsector>>,
}

impl Node {
    pub fn new(
        line: Line,
        left_node: Option<Box<Node>>,
        right_node: Option<Box<Node>>,
        left_subsector: Option<Box<Subsector>>,
        right_subsector: Option<Box<Subsector>>,
    ) -> Self {
        Node {
            line,
            left_node,
            right_node,
            left_subsector,
            right_subsector,
        }
    }

    pub fn travers(&self, player: &Player) -> (&Option<Box<Node>>, &Option<Box<Subsector>>, &Line) {
        let is_under = self.line.is_point_under(Vec2d::new(player.x, player.y));
        let future_node = if is_under {
            &self.left_node
        } else {
            &self.right_node
        };

        let future_subsector = if is_under {
            &self.left_subsector
        } else {
            &self.right_subsector
        };

        match future_node {
            Some(node) => node.travers(player),
            None => (future_node, future_subsector, &self.line),
        };
        (future_node, future_subsector, &self.line)
    }

    pub fn get_neighbours(&self, player: Player) {
        let is_under = self.line.is_point_under(Vec2d::new(player.x, player.y));
        if !is_under {
            &self.left_node
        } else {
            &self.right_node
        };
    }
}
