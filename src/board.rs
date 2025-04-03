use bevy_math::{IVec2, Vec2};

use crate::graph::{Graph, graph_key::EdgeKey};

pub(crate) struct Board {
    root: Vec2,
    offset: Vec2,
    bounds_min: IVec2,
    bounds_max: IVec2,
    movement_graph: Graph<()>,
}

impl Board {
    pub(crate) fn new(
        root: Vec2,
        offset: Vec2,
        bounds_min: IVec2,
        bounds_max: IVec2,
        movement_graph: Graph<()>,
    ) -> Option<Self> {
        if !root.is_finite() || !offset.is_finite() || offset.x as i32 <= 0 || offset.y as i32 <= 0
        {
            return None;
        }

        let bounds_min_result = IVec2::new(
            i32::min(bounds_min.x, bounds_max.x),
            i32::min(bounds_min.y, bounds_max.y),
        );
        let bounds_max_result = IVec2::new(
            i32::max(bounds_min.x, bounds_max.x),
            i32::max(bounds_min.y, bounds_max.y),
        );

        Some(Self {
            root,
            offset,
            bounds_min: bounds_min_result,
            bounds_max: bounds_max_result,
            movement_graph,
        })
    }

    pub(crate) const fn is_point_in_grid(&self, point: IVec2) -> bool {
        point.x >= self.bounds_min.x
            && point.y >= self.bounds_min.y
            && point.x <= self.bounds_max.x
            && point.y <= self.bounds_max.y
    }

    pub(crate) fn is_movement_blocked(&self, edge: EdgeKey) -> bool {
        self.movement_graph.get_edge_one_way(edge).is_some()
    }

    pub(crate) fn world_to_grid_space(&self, point: Vec2) -> IVec2 {
        IVec2::new(
            ((point.x - self.root.x) / self.offset.x).round() as i32,
            ((point.y - self.root.y) / self.offset.y).round() as i32,
        )
    }

    pub(crate) const fn grid_to_world_space(&self, point: IVec2) -> Vec2 {
        Vec2::new(
            self.root.x + point.x as f32 * self.offset.x,
            self.root.y + point.y as f32 * self.offset.y,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use bevy_math::{I16Vec3, IVec2, Vec2};

    #[test]
    fn test_new_invalid_root() {
        let root = Vec2::new(f32::NAN, 0.0);
        let offset = Vec2::new(1.0, 1.0);
        let bounds_min = IVec2::new(0, 0);
        let bounds_max = IVec2::new(10, 10);
        let movement_graph = Graph::new();

        assert!(Board::new(root, offset, bounds_min, bounds_max, movement_graph).is_none());
    }

    #[test]
    fn test_new_invalid_offset() {
        let root = Vec2::new(0.0, 0.0);
        let offset = Vec2::new(f32::NAN, 1.0);
        let bounds_min = IVec2::new(0, 0);
        let bounds_max = IVec2::new(10, 10);
        let movement_graph = Graph::new();

        assert!(Board::new(root, offset, bounds_min, bounds_max, movement_graph).is_none());
    }

    #[test]
    fn test_new_offset_x_zero() {
        let root = Vec2::new(0.0, 0.0);
        let offset = Vec2::new(0.0, 1.0);
        let bounds_min = IVec2::new(0, 0);
        let bounds_max = IVec2::new(10, 10);
        let movement_graph = Graph::new();

        assert!(Board::new(root, offset, bounds_min, bounds_max, movement_graph).is_none());
    }

    #[test]
    fn test_new_offset_y_zero() {
        let root = Vec2::new(0.0, 0.0);
        let offset = Vec2::new(1.0, 0.0);
        let bounds_min = IVec2::new(0, 0);
        let bounds_max = IVec2::new(10, 10);
        let movement_graph = Graph::new();

        assert!(Board::new(root, offset, bounds_min, bounds_max, movement_graph).is_none());
    }

    #[test]
    fn test_new_valid_input() {
        let root = Vec2::new(0.0, 0.0);
        let offset = Vec2::new(1.0, 1.0);
        let bounds_min = IVec2::new(0, 0);
        let bounds_max = IVec2::new(10, 10);
        let movement_graph = Graph::new();

        let board = Board::new(root, offset, bounds_min, bounds_max, movement_graph).unwrap();

        assert_eq!(board.root, root);
        assert_eq!(board.offset, offset);
        assert_eq!(board.bounds_min, bounds_min);
        assert_eq!(board.bounds_max, bounds_max);
    }

    #[test]
    fn test_new_valid_input_swapped_bounds() {
        let root = Vec2::new(0.0, 0.0);
        let offset = Vec2::new(1.0, 1.0);
        let bounds_min = IVec2::new(10, 10);
        let bounds_max = IVec2::new(0, 0);
        let movement_graph = Graph::new();

        let board = Board::new(root, offset, bounds_min, bounds_max, movement_graph).unwrap();

        assert_eq!(board.root, root);
        assert_eq!(board.offset, offset);
        assert_eq!(board.bounds_min, IVec2::new(0, 0));
        assert_eq!(board.bounds_max, IVec2::new(10, 10));
    }

    #[test]
    fn test_new_valid_input_equal_bounds() {
        let root = Vec2::new(0.0, 0.0);
        let offset = Vec2::new(1.0, 1.0);
        let bounds_min = IVec2::new(5, 5);
        let bounds_max = IVec2::new(5, 5);
        let movement_graph = Graph::new();

        let board = Board::new(root, offset, bounds_min, bounds_max, movement_graph).unwrap();

        assert_eq!(board.root, root);
        assert_eq!(board.offset, offset);
        assert_eq!(board.bounds_min, IVec2::new(5, 5));
        assert_eq!(board.bounds_max, IVec2::new(5, 5));
    }

    #[test]
    fn test_is_point_in_grid_within_bounds() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 1.0),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };
        let point = IVec2::new(5, 5);
        assert!(board.is_point_in_grid(point));
    }
    #[test]
    fn test_is_point_in_grid_outside_bounds_x_too_low() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 1.0),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };
        let point = IVec2::new(-1, 5);
        assert!(!board.is_point_in_grid(point));
    }
    #[test]
    fn test_is_point_in_grid_outside_bounds_x_too_high() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 1.0),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };
        let point = IVec2::new(11, 5);
        assert!(!board.is_point_in_grid(point));
    }
    #[test]
    fn test_is_point_in_grid_outside_bounds_y_too_low() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 1.0),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };
        let point = IVec2::new(5, -1);
        assert!(!board.is_point_in_grid(point));
    }
    #[test]
    fn test_is_point_in_grid_outside_bounds_y_too_high() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 1.0),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };
        let point = IVec2::new(5, 11);
        assert!(!board.is_point_in_grid(point));
    }
    #[test]
    fn test_is_point_in_grid_on_bounds_x_min() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 1.0),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };
        let point = IVec2::new(0, 5);
        assert!(board.is_point_in_grid(point));
    }
    #[test]
    fn test_is_point_in_grid_on_bounds_x_max() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 1.0),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };
        let point = IVec2::new(10, 5);
        assert!(board.is_point_in_grid(point));
    }
    #[test]
    fn test_is_point_in_grid_on_bounds_y_min() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 1.0),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };
        let point = IVec2::new(5, 0);
        assert!(board.is_point_in_grid(point));
    }

    #[test]
    fn test_is_movement_blocked_edge_exists() {
        let mut movement_graph = Graph::new();
        let from = I16Vec3::new(0, 0, 0);
        let to = I16Vec3::new(1, 1, 1);
        let edge = EdgeKey::new(from, to).unwrap();
        movement_graph.insert_edge_one_way(edge.clone(), ());
        let board = Board::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 1.0),
            IVec2::new(0, 0),
            IVec2::new(1, 1),
            movement_graph,
        )
        .unwrap();
        assert!(board.is_movement_blocked(edge));
    }

    #[test]
    fn test_is_movement_blocked_edge_does_not_exist() {
        let movement_graph = Graph::new();
        let from = I16Vec3::new(0, 0, 0);
        let to = I16Vec3::new(1, 1, 1);
        let edge = EdgeKey::new(from, to).unwrap();
        let board = Board::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 1.0),
            IVec2::new(0, 0),
            IVec2::new(1, 1),
            movement_graph,
        )
        .unwrap();
        assert!(!board.is_movement_blocked(edge));
    }

    #[test]
    fn test_world_to_grid_space_positive() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 1.0),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };
        let point = Vec2::new(3.0, 4.0);
        let expected = IVec2::new(3, 4);
        assert_eq!(board.world_to_grid_space(point), expected);
    }
    #[test]
    fn test_world_to_grid_space_negative() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 1.0),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };
        let point = Vec2::new(-3.0, -4.0);
        let expected = IVec2::new(-3, -4);
        assert_eq!(board.world_to_grid_space(point), expected);
    }
    #[test]
    fn test_world_to_grid_space_zero() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 1.0),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };
        let point = Vec2::new(0.0, 0.0);
        let expected = IVec2::new(0, 0);
        assert_eq!(board.world_to_grid_space(point), expected);
    }
    #[test]
    fn test_world_to_grid_space_non_integer() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 1.0),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };
        let point = Vec2::new(3.5, 4.2);
        let expected = IVec2::new(4, 4);
        assert_eq!(board.world_to_grid_space(point), expected);
    }
    #[test]
    fn test_world_to_grid_space_offset_1() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 1.0),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };

        let test_cases = vec![
            (Vec2::new(3.0, 4.0), IVec2::new(3, 4)),
            (Vec2::new(-3.0, -4.0), IVec2::new(-3, -4)),
            (Vec2::new(0.0, 0.0), IVec2::new(0, 0)),
            (Vec2::new(3.4, 4.2), IVec2::new(3, 4)),
            (Vec2::new(3.5, 4.2), IVec2::new(4, 4)),
            (Vec2::new(3.6, 4.2), IVec2::new(4, 4)),
        ];

        for (point, expected) in test_cases {
            assert_eq!(board.world_to_grid_space(point), expected);
        }
    }

    #[test]
    fn test_world_to_grid_space_offset_2() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 1.5),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };

        let test_cases = vec![
            (Vec2::new(3.0, 4.0), IVec2::new(3, 3)),
            (Vec2::new(-3.0, -4.0), IVec2::new(-3, -3)),
            (Vec2::new(0.0, 0.0), IVec2::new(0, 0)),
            (Vec2::new(3.4, 4.2), IVec2::new(3, 3)),
            (Vec2::new(3.5, 4.2), IVec2::new(4, 3)),
            (Vec2::new(3.6, 4.2), IVec2::new(4, 3)),
        ];

        for (point, expected) in test_cases {
            assert_eq!(board.world_to_grid_space(point), expected);
        }
    }
    #[test]
    fn test_world_to_grid_space_offset_not_1() {
        let board = Board {
            root: Vec2::new(0.0, 0.0),
            offset: Vec2::new(1.0, 2.0),
            bounds_min: IVec2::new(0, 0),
            bounds_max: IVec2::new(10, 10),
            movement_graph: Graph::new(),
        };

        let test_cases = vec![
            (Vec2::new(3.0, 4.0), IVec2::new(3, 2)),
            (Vec2::new(-3.0, -4.0), IVec2::new(-3, -2)),
            (Vec2::new(0.0, 0.0), IVec2::new(0, 0)),
            (Vec2::new(3.4, 4.2), IVec2::new(3, 2)),
            (Vec2::new(3.5, 4.2), IVec2::new(4, 2)),
            (Vec2::new(3.6, 4.2), IVec2::new(4, 2)),
        ];

        for (point, expected) in test_cases {
            assert_eq!(board.world_to_grid_space(point), expected);
        }
    }
}
