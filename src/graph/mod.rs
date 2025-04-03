use core::fmt::Debug;
use graph_key::{EdgeData, EdgeKey, VertexPointAndData};
use indexmap_nostd::IndexMap;
use indexmap_nostd::map::Entry;

pub(crate) mod graph_key;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Graph<DataType: Clone + Eq + Debug> {
    edges_data: IndexMap<EdgeKey, EdgeData<DataType>>,
}

impl<DataType: Clone + Eq + Debug> Graph<DataType> {
    pub fn new() -> Self {
        Self {
            edges_data: IndexMap::new(),
        }
    }

    pub fn insert_edge_one_way(&mut self, edge_key: EdgeKey, data: DataType) {
        match self.edges_data.entry(edge_key.clone()) {
            Entry::Occupied(mut occupied) => {
                let occupied_data = occupied.get_mut();
                match occupied_data {
                    EdgeData::One(existing) => {
                        if existing.0 == edge_key.from {
                            *occupied_data = EdgeData::One(VertexPointAndData(edge_key.from, data));
                        } else if existing.0 == edge_key.to {
                            if existing.1 == data {
                                *occupied_data = EdgeData::IdencticalTwo(data);
                            } else {
                                *occupied_data = EdgeData::NonIdencticalTwo(
                                    existing.clone(),
                                    VertexPointAndData(edge_key.from, data),
                                );
                            }
                        }
                    }
                    EdgeData::IdencticalTwo(existing_data) => {
                        *occupied_data = EdgeData::NonIdencticalTwo(
                            VertexPointAndData(edge_key.to, existing_data.clone()),
                            VertexPointAndData(edge_key.from, data.clone()),
                        );
                    }
                    EdgeData::NonIdencticalTwo(first, second) => {
                        if first.0 == edge_key.from {
                            *occupied_data = EdgeData::NonIdencticalTwo(
                                VertexPointAndData(edge_key.from, first.1.clone()),
                                second.clone(),
                            );
                        } else {
                            *occupied_data = EdgeData::NonIdencticalTwo(
                                first.clone(),
                                VertexPointAndData(edge_key.from, data.clone()),
                            );
                        }
                    }
                }
            }
            Entry::Vacant(vacant) => {
                vacant.insert(EdgeData::One(VertexPointAndData(edge_key.from, data)));
            }
        }
    }

    pub fn insert_edge_two_way(&mut self, edge_key: EdgeKey, data: DataType) {
        match self.edges_data.entry(edge_key) {
            Entry::Occupied(mut occupied) => {
                let occupied_data = occupied.get_mut();
                match occupied_data {
                    EdgeData::One(_) => {
                        *occupied_data = EdgeData::IdencticalTwo(data);
                    }
                    EdgeData::IdencticalTwo(_) => {
                        *occupied_data = EdgeData::IdencticalTwo(data.clone());
                    }
                    EdgeData::NonIdencticalTwo(_, _) => {
                        *occupied_data = EdgeData::IdencticalTwo(data.clone());
                    }
                }
            }
            Entry::Vacant(vacant) => {
                vacant.insert(EdgeData::IdencticalTwo(data));
            }
        }
    }

    pub fn remove_edge_one_way(&mut self, edge_key: EdgeKey) {
        let new_edge_data: Option<EdgeData<DataType>> =
            if let Some(edge_data) = self.edges_data.get(&edge_key) {
                match edge_data {
                    EdgeData::One(stored) => {
                        if stored.0 == edge_key.from || stored.0 == edge_key.to {
                            None
                        } else {
                            Some(edge_data.clone())
                        }
                    }
                    EdgeData::IdencticalTwo(stored_data) => Some(EdgeData::One(
                        VertexPointAndData(edge_key.to, stored_data.clone()),
                    )),
                    EdgeData::NonIdencticalTwo(edge_key_and_data1, edge_key_and_data2) => {
                        if edge_key_and_data1.0 == edge_key.from {
                            Some(EdgeData::One(edge_key_and_data2.clone()))
                        } else if edge_key_and_data2.0 == edge_key.from {
                            Some(EdgeData::One(edge_key_and_data1.clone()))
                        } else {
                            None
                        }
                    }
                }
            } else {
                None
            };

        if let Some(new_edge_data) = new_edge_data {
            self.edges_data.insert(edge_key, new_edge_data);
        } else {
            self.edges_data = self
                .edges_data
                .clone()
                .into_iter()
                .filter(|(key, _)| *key != edge_key)
                .collect();
        }
    }

    pub fn remove_edge_two_way(&mut self, edge_key: EdgeKey) {
        self.edges_data = self
            .edges_data
            .clone()
            .into_iter()
            .filter(|(key, _)| *key != edge_key)
            .collect();
    }

    pub fn get_edge_one_way(&self, edge_key: EdgeKey) -> Option<&DataType> {
        match self.get_edge_two_way(edge_key.clone()) {
            Some(edge_data) => match edge_data {
                EdgeData::One(stored) => {
                    if stored.0 == edge_key.from {
                        Some(&stored.1)
                    } else {
                        None
                    }
                }
                EdgeData::IdencticalTwo(stored_data) => Some(stored_data),
                EdgeData::NonIdencticalTwo(edge_key_and_data1, edge_key_and_data2) => {
                    if edge_key_and_data1.0 == edge_key.from {
                        Some(&edge_key_and_data1.1)
                    } else if edge_key_and_data2.0 == edge_key.from {
                        Some(&edge_key_and_data2.1)
                    } else {
                        None
                    }
                }
            },
            None => None,
        }
    }

    pub fn get_edge_two_way(&self, edge_key: EdgeKey) -> Option<&EdgeData<DataType>> {
        self.edges_data.get(&edge_key)
    }
}

#[cfg(test)]
mod tests {

    use bevy_math::I16Vec3;

    use crate::graph::graph_key::Vertex3Key;

    use super::*;

    #[test]
    fn get_edge_one_way_one_direction() {
        let mut graph = Graph::new();
        graph.insert_edge_one_way(
            EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap(),
            1,
        );
        assert_eq!(
            graph.get_edge_one_way(
                EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap()
            ),
            Some(&1)
        );
        assert_eq!(
            graph.get_edge_one_way(
                EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap()
            ),
            None
        );
    }

    #[test]
    fn get_edge_one_way_two_directions() {
        let mut graph = Graph::new();
        graph.insert_edge_one_way(
            EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap(),
            1,
        );

        graph.insert_edge_one_way(
            EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap(),
            2,
        );
        assert_eq!(
            graph.get_edge_one_way(
                EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap()
            ),
            Some(&2)
        );
    }

    #[test]
    fn get_edge_one_way_two_way_edge() {
        let mut graph = Graph::new();
        graph.insert_edge_two_way(
            EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap(),
            3,
        );
        assert_eq!(
            graph.get_edge_one_way(
                EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap()
            ),
            Some(&3)
        );
        assert_eq!(
            graph.get_edge_one_way(
                EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap()
            ),
            Some(&3)
        );
    }

    #[test]
    fn get_edge_two_way() {
        let mut graph = Graph::new();
        graph.insert_edge_two_way(
            EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap(),
            2,
        );
        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap()
            ),
            Some(&EdgeData::IdencticalTwo(2))
        );
        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap()
            ),
            Some(&EdgeData::IdencticalTwo(2))
        );
    }

    #[test]
    fn add_one_way_edge() {
        let mut graph = Graph::new();
        graph.insert_edge_one_way(
            EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap(),
            1,
        );
        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap()
            ),
            Some(&EdgeData::One(VertexPointAndData(
                Vertex3Key(I16Vec3::new(0, 0, 0)),
                1
            )))
        );
    }

    #[test]
    fn add_non_identical_two_way_edges() {
        let mut graph = Graph::new();
        graph.insert_edge_one_way(
            EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap(),
            1,
        );
        graph.insert_edge_one_way(
            EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap(),
            2,
        );
        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap()
            ),
            Some(&EdgeData::NonIdencticalTwo(
                VertexPointAndData(Vertex3Key(I16Vec3::new(1, 1, 1)), 2),
                VertexPointAndData(Vertex3Key(I16Vec3::new(0, 0, 0)), 1)
            ))
        );
    }

    #[test]
    fn add_identical_two_way_edge() {
        let mut graph = Graph::new();
        graph.insert_edge_two_way(
            EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap(),
            3,
        );
        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap()
            ),
            Some(&EdgeData::IdencticalTwo(3))
        );
        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap()
            ),
            Some(&EdgeData::IdencticalTwo(3))
        );
    }

    #[test]
    fn remove_one_edge() {
        let mut graph = Graph::new();
        graph.insert_edge_one_way(
            EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap(),
            1,
        );

        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap()
            ),
            Some(&EdgeData::One(VertexPointAndData(
                Vertex3Key(I16Vec3::new(0, 0, 0)),
                1
            )))
        );

        graph.remove_edge_one_way(
            EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap(),
        );

        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap()
            ),
            None
        );

        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap()
            ),
            None
        );
    }

    #[test]
    fn remove_two_edges() {
        let mut graph = Graph::new();
        graph.insert_edge_one_way(
            EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap(),
            1,
        );
        graph.insert_edge_one_way(
            EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap(),
            1,
        );

        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap()
            ),
            Some(&EdgeData::IdencticalTwo(1))
        );

        graph.remove_edge_two_way(
            EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap(),
        );

        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap()
            ),
            None
        );

        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap()
            ),
            None
        );
    }

    #[test]
    fn remove_one_direction_of_two_identical_edges() {
        let mut graph = Graph::new();
        graph.insert_edge_two_way(
            EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap(),
            1,
        );

        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap()
            ),
            Some(&EdgeData::IdencticalTwo(1))
        );

        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap()
            ),
            Some(&EdgeData::IdencticalTwo(1))
        );

        graph.remove_edge_one_way(
            EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap(),
        );

        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap()
            ),
            Some(&EdgeData::One(VertexPointAndData(
                Vertex3Key(I16Vec3::new(1, 1, 1)),
                1
            )))
        );

        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap()
            ),
            Some(&EdgeData::One(VertexPointAndData(
                Vertex3Key(I16Vec3::new(1, 1, 1)),
                1
            )))
        );
    }

    #[test]
    fn remove_remaining_direction_of_two_identical_edges() {
        let mut graph = Graph::new();
        graph.insert_edge_two_way(
            EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap(),
            1,
        );

        graph.remove_edge_one_way(
            EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap(),
        );

        graph.remove_edge_one_way(
            EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap(),
        );

        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(1, 1, 1), I16Vec3::new(0, 0, 0)).unwrap()
            ),
            None
        );

        assert_eq!(
            graph.get_edge_two_way(
                EdgeKey::new(I16Vec3::new(0, 0, 0), I16Vec3::new(1, 1, 1)).unwrap()
            ),
            None
        );
    }
}
