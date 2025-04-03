use bevy_math::IVec2;

/// An error indicating that a direction is invalid.
#[derive(Debug, PartialEq)]
pub enum InvalidDirectionError {
    /// The length of the direction vector is zero or very close to zero.
    Zero,
    /// Cant find direction for this vector
    XYAbsEqual,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct IDir2(IVec2);

impl IDir2 {
    /// A unit vector pointing along the positive X axis.
    pub const X: Self = IDir2(IVec2 { x: 1, y: 0 });

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = IDir2(IVec2 { x: 0, y: 1 });

    /// Create a direction from a finite, nonzero [`Vec2`], normalizing it.
    ///
    /// Returns [`Err(InvalidDirectionError)`](InvalidDirectionError) if the length
    /// of the given vector is zero (or very close to zero), infinite, or `NaN`.
    pub fn new(value: IVec2) -> Result<Self, InvalidDirectionError> {
        Self::new_and_length_squared(value).map(|(dir, _)| dir)
    }

    /// Create a direction from a finite, nonzero [`Vec2`], normalizing it and
    /// also returning its original length.
    ///
    /// Returns [`Err(InvalidDirectionError)`](InvalidDirectionError) if the length
    /// of the given vector is zero (or very close to zero), infinite, or `NaN`.
    pub fn new_and_length_squared(value: IVec2) -> Result<(Self, i32), InvalidDirectionError> {
        let length = value.length_squared();

        if value.x == 0 && value.y == 0 {
            return Err(InvalidDirectionError::Zero);
        }

        if value.x.abs() == value.y.abs() {
            return Err(InvalidDirectionError::XYAbsEqual);
        }

        let result_value = if value.x.abs() < value.y.abs() {
            IVec2::new(0, value.y.signum())
        } else {
            IVec2::new(value.x.signum(), 0)
        };

        Ok((Self(result_value), length))
    }

    /// Create a direction from its `x` and `y` components.
    ///
    /// Returns [`Err(InvalidDirectionError)`](InvalidDirectionError) if the length
    /// of the vector formed by the components is zero (or very close to zero), infinite, or `NaN`.
    pub fn from_xy(x: i32, y: i32) -> Result<Self, InvalidDirectionError> {
        Self::new(IVec2::new(x, y))
    }

    /// Returns the inner [`Vec2`]
    pub const fn as_ivec2(&self) -> IVec2 {
        self.0
    }
}

#[cfg(test)]
mod tests {

    //TODO add tests
}
