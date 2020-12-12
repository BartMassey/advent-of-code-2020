// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Direction and rotation management for Advent of Code
//! solutions.

use crate::ConvertInto;

/// Symbolic direction constants. It is unfortunate that
/// these need to be matched to DIRNS and FACINGS below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dirn {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}

/// Rotation directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rot {
    /// Counter-clockwise.
    CCW,
    /// Clockwise.
    CW,
}

/// Displacements induced by the cardinal directions: up,
/// down, left, right in an r-c coordinate system where
/// increasing r is down.
pub const DIRNS: [(i64, i64); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

/// The possible facings.
pub const FACINGS: [Dirn; 4] =
    [Dirn::Up, Dirn::Left, Dirn::Down, Dirn::Right];

impl Dirn {
    /// Displacement resulting from a step in the given
    /// direction.
    pub fn disp<T>(self) -> (T, T)
    where
        i64: ConvertInto<T>,
    {
        let (r, c) = DIRNS[self as usize];
        (r.convert_into(), c.convert_into())
    }

    /// Apply the appropriate displacement for the given
    /// distance in this direction to the given point.
    pub fn displace<T, U>(self, point: (T, T), dist: U) -> (T, T)
    where
        T: ConvertInto<i64>,
        i64: ConvertInto<T>,
        U: ConvertInto<i64>,
    {
        let (dr, dc) = self.disp::<i64>();
        let mut r = point.0.convert_into();
        let mut c = point.1.convert_into();
        let dist = dist.convert_into();
        r += dist * dr;
        c += dist * dc;
        (r.convert_into(), c.convert_into())
    }

    /// Direction resulting from turning 90° in the given
    /// rotation direction the given number of times.
    pub fn turn<T>(self, rot: Rot, steps: T) -> Dirn
    where
        T: ConvertInto<i64>,
    {
        let mut steps: i64 = steps.convert_into();
        if steps < 0 {
            steps = (4 - -steps % 4) % 4;
        }
        let steps = steps as usize;
        let nfacings = FACINGS.len();
        let offset = match rot {
            Rot::CCW => steps % nfacings,
            Rot::CW => ((nfacings - 1) * steps) % nfacings,
        };
        FACINGS[(self as usize + offset) % nfacings]
    }
}

#[test]
fn test_rot() {
    use Dirn::*;
    use Rot::*;
    assert_eq!(Left, Up.turn(CCW, 1));
    assert_eq!(Right, Up.turn(CW, 1));
    assert_eq!(Down, Left.turn(CCW, 1));
    assert_eq!(Down, Right.turn(CW, 1));
    assert_eq!(Right, Right.turn(CW, -8));
    assert_eq!(Right, Right.turn(CCW, -8));
    assert_eq!(Up, Right.turn(CW, -9));
    assert_eq!(Down, Right.turn(CCW, -9));
}
