/*

local mathUtils = require("root.source.utils.MathUtils")

---@class WorldSide
---@field Offset vector3
---@field Rotation quat
---@field Next WorldSide

local WorldSides = {}

---@type WorldSide
WorldSides.UP = { Offset = vmath.vector3(0, 0, 1), Rotation = vmath.quat_rotation_y(math.rad(0)) }

---@type WorldSide
WorldSides.RIGHT = { Offset = vmath.vector3(-1, 0, 0), Rotation = vmath.quat_rotation_y(math.rad(-90)) }

---@type WorldSide
WorldSides.DOWN = { Offset = vmath.vector3(0, 0, -1), Rotation = vmath.quat_rotation_y(math.rad(-180)) }

---@type WorldSide
WorldSides.LEFT = { Offset = vmath.vector3(1, 0, 0), Rotation = vmath.quat_rotation_y(math.rad(-270)) }

WorldSides.UP.Next = WorldSides.RIGHT
WorldSides.RIGHT.Next = WorldSides.DOWN
WorldSides.DOWN.Next = WorldSides.LEFT
WorldSides.LEFT.Next = WorldSides.UP

---@param rotation quat
---@return WorldSide
function WorldSides:RotationToSide(rotation)
    local currentSide = self.UP
    local distance = mathUtils.AngularDistance(rotation, currentSide.Rotation)
    for i = 1, 3, 1 do
        local currentDistance = mathUtils.AngularDistance(rotation, currentSide.Rotation)
        if currentDistance < distance then
            distance = currentDistance
            currentSide = currentSide.Next
        end
    end
    return currentSide
end

---@param offset vector3
---@return WorldSide
function WorldSides:OffsetToSide(offset)
    local currentSide = self.UP
    for i = 1, 4, 1 do
        if offset == currentSide.Offset then
            return currentSide
        end
        currentSide = currentSide.Next
    end
    assert(true)
    return nil
end

---@param from WorldSide
---@param to WorldSide
---@param coordinates vector3
function WorldSides:RotateCoordinates(from, to, coordinates)
    local currentCoordinates = vmath.vector3(coordinates)
    while from ~= to do
        from = from.Next
        local x = currentCoordinates.x
        local z = currentCoordinates.z
        currentCoordinates.x = -z
        currentCoordinates.z = x
    end
    return currentCoordinates
end

return WorldSides

*/

use core::cmp::Ordering;

use bevy_math::IVec2;
use bevy_math::{I16Vec3, Quat};

use crate::idir2::IDir2;

#[derive(Debug, Clone, Copy)]
struct WorldSideInfo {
    offset: I16Vec3,
    rotation: Quat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorldSides {
    Up,
    Right,
    Down,
    Left,
}

impl WorldSides {
    const fn info(&self) -> &WorldSideInfo {
        static INFO: [WorldSideInfo; 4] = [
            WorldSideInfo {
                offset: I16Vec3::new(0, 0, 1),
                rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
            },
            WorldSideInfo {
                offset: I16Vec3::new(-1, 0, 0),
                rotation: Quat::from_xyzw(0.0, -0.70710677, 0.0, 0.70710677),
            },
            WorldSideInfo {
                offset: I16Vec3::new(0, 0, -1),
                rotation: Quat::from_xyzw(0.0, -1.0, 0.0, -4.371139e-8),
            },
            WorldSideInfo {
                offset: I16Vec3::new(1, 0, 0),
                rotation: Quat::from_xyzw(0.0, -0.70710677, 0.0, -0. - 0.707_106_77),
            },
        ];

        match self {
            WorldSides::Up => &INFO[0],
            WorldSides::Right => &INFO[1],
            WorldSides::Down => &INFO[2],
            WorldSides::Left => &INFO[3],
        }
    }

    const fn to_array() -> [WorldSides; 4] {
        [
            WorldSides::Up,
            WorldSides::Right,
            WorldSides::Down,
            WorldSides::Left,
        ]
    }

    fn rotation_to_side(rotation: Quat) -> WorldSides {
        Self::to_array()
            .iter()
            .min_by(|l, r| {
                let left_quat = l.info().rotation;
                let right_quat = r.info().rotation;

                let diff_left_target_rotation = left_quat.angle_between(rotation);

                let diff_right_target_rotation = right_quat.angle_between(rotation);

                diff_left_target_rotation
                    .partial_cmp(&diff_right_target_rotation)
                    .unwrap_or(Ordering::Equal)
            })
            .cloned()
            .unwrap_or(WorldSides::Up) // Default to Up in case of an empty iterator
    }

    fn dir_to_side(direction: IDir2) -> WorldSides {
        Self::to_array()
            .iter()
            .find(|side| {
                let world_side = side.info();
                let dir = IDir2::new(IVec2::new(
                    world_side.offset.x.into(),
                    world_side.offset.z.into(),
                ));
                let result_dir = dir.unwrap_or(IDir2::Y);
                direction == result_dir
            })
            .cloned()
            .unwrap_or(WorldSides::Up)
    }
}

impl Iterator for WorldSides {
    type Item = WorldSides;

    /// Переход к следующей стороне по часовой стрелке
    fn next(&mut self) -> Option<WorldSides> {
        match self {
            WorldSides::Up => Some(WorldSides::Right),
            WorldSides::Right => Some(WorldSides::Down),
            WorldSides::Down => Some(WorldSides::Left),
            WorldSides::Left => Some(WorldSides::Up),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::f32::consts::FRAC_PI_2;
    use std::println;

    use super::*;

    #[test]
    fn test_rotation_to_edge() {
        let q = Quat::from_rotation_y(0.0);
        assert_eq!(WorldSides::rotation_to_side(q), WorldSides::Up);
        println!("{:?}", q);
        let q = Quat::from_rotation_y(-1.0 * FRAC_PI_2);
        assert_eq!(WorldSides::rotation_to_side(q), WorldSides::Right);
        println!("{:?}", q);
        let q = Quat::from_rotation_y(-2.0 * FRAC_PI_2);
        assert_eq!(WorldSides::rotation_to_side(q), WorldSides::Down);
        println!("{:?}", q);
        let q = Quat::from_rotation_y(-3.0 * FRAC_PI_2);
        assert_eq!(WorldSides::rotation_to_side(q), WorldSides::Left);
        println!("{:?}", q);
    }

    #[test]
    fn test_dir_to_edge() {
        let dir_to_check = IDir2::from_xy(0, 1).unwrap();
        assert_eq!(WorldSides::dir_to_side(dir_to_check), WorldSides::Up);

        let dir_to_check = IDir2::from_xy(-1, 0).unwrap();
        assert_eq!(WorldSides::dir_to_side(dir_to_check), WorldSides::Right);

        let dir_to_check = IDir2::from_xy(0, -1).unwrap();
        assert_eq!(WorldSides::dir_to_side(dir_to_check), WorldSides::Down);

        let dir_to_check = IDir2::from_xy(1, 0).unwrap();
        assert_eq!(WorldSides::dir_to_side(dir_to_check), WorldSides::Left);
    }
}
