use core::cmp::Ordering;

use bevy_math::{I16Vec3, IVec3};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Vertex3Key(pub(crate) I16Vec3);

impl From<I16Vec3> for Vertex3Key {
    fn from(vec: I16Vec3) -> Self {
        Vertex3Key(vec)
    }
}

impl From<IVec3> for Vertex3Key {
    fn from(vec: IVec3) -> Self {
        Vertex3Key(vec.as_i16vec3())
    }
}

impl Ord for Vertex3Key {
    fn cmp(&self, other: &Self) -> Ordering {
        let [x1, y1, z1] = self.0.to_array();
        let [x2, y2, z2] = other.0.to_array();
        if x1 < x2 {
            Ordering::Less
        } else if x1 > x2 {
            Ordering::Greater
        } else if y1 < y2 {
            Ordering::Less
        } else if y1 > y2 {
            Ordering::Greater
        } else if z1 < z2 {
            Ordering::Less
        } else if z1 > z2 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Vertex3Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub(in crate::graph) struct VertexPointAndData<DataType: Clone>(
    pub(in crate::graph) Vertex3Key,
    pub(in crate::graph) DataType,
);

#[derive(Clone, Eq, Debug)]
pub(crate) enum EdgeData<DataType: Clone + Eq + PartialEq> {
    One(VertexPointAndData<DataType>),
    IdencticalTwo(DataType),
    NonIdencticalTwo(VertexPointAndData<DataType>, VertexPointAndData<DataType>),
}

impl<DataType: Clone + Eq> PartialEq for EdgeData<DataType> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EdgeData::One(data1), EdgeData::One(data2)) => data1 == data2,
            (EdgeData::IdencticalTwo(data1), EdgeData::IdencticalTwo(data2)) => data1 == data2,
            (
                EdgeData::NonIdencticalTwo(data1, data2),
                EdgeData::NonIdencticalTwo(data3, data4),
            ) => data1 == data3 && data2 == data4 || data1 == data4 && data2 == data3,
            _ => false,
        }
    }
}

#[derive(Eq, Clone, Debug)]
pub(crate) struct EdgeKey {
    pub(crate) from: Vertex3Key,
    pub(crate) to: Vertex3Key,
}

impl EdgeKey {
    pub fn new<T: Into<Vertex3Key>>(from: T, to: T) -> Option<Self> {
        let from = from.into();
        let to = to.into();
        if from == to {
            return None;
        }
        Some(Self { from, to })
    }
}

impl PartialEq for EdgeKey {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
            || self.from == other.to && self.to == other.from
    }
}

impl Ord for EdgeKey {
    fn cmp(&self, other: &Self) -> Ordering {
        let (self_sorted_from, self_sorted_to) = match self.from.cmp(&self.to) {
            Ordering::Greater => (&self.to, &self.from),
            _ => (&self.from, &self.to),
        };

        let (other_sorted_from, other_sorted_to) = match other.from.cmp(&other.to) {
            Ordering::Greater => (&other.to, &other.from),
            _ => (&other.from, &other.to),
        };

        match (
            self_sorted_from.cmp(other_sorted_from),
            self_sorted_to.cmp(other_sorted_to),
        ) {
            (Ordering::Less, _) => Ordering::Less,
            (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
            (Ordering::Equal, Ordering::Less) => Ordering::Less,
            (Ordering::Equal, Ordering::Greater) => Ordering::Greater,
            (Ordering::Greater, _) => Ordering::Greater,
        }
    }
}

impl PartialOrd for EdgeKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use bevy_math::I16Vec3;

    use super::*;

    #[test]
    fn test_create_edge_keys() {
        let a = I16Vec3::new(0, 0, 0);
        let b = I16Vec3::new(1, 1, 1);
        let c = I16Vec3::new(2, 2, 2);
        let d = I16Vec3::new(3, 3, 3);
        let e = I16Vec3::new(4, 4, 4);

        let nodes = [a, b, c, d, e];

        for from in &nodes {
            for to in &nodes {
                if from == to {
                    assert_eq!(EdgeKey::new(*from, *to), None);
                } else {
                    assert_eq!(
                        EdgeKey::new(*from, *to),
                        Some(EdgeKey {
                            from: (*from).into(),
                            to: (*to).into()
                        })
                    );
                }
            }
        }
    }
}
