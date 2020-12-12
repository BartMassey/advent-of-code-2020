// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Direction and rotation management for Advent of Code
//! solutions.

/// Symbolic direction constants. It is unfortunate that
/// these need to be matched to DIRNS below.
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
/// down, left, right in an x-y coordinate system where
/// increasing y is down.
pub const DIRNS: [(i64, i64); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

/// The possible facings.
pub const FACINGS: [Dirn; 4] =
    [Dirn::Up, Dirn::Left, Dirn::Down, Dirn::Right];

impl Dirn {
    /// Displacement resulting from a step in the given
    /// direction.
    pub fn disp(self) -> (i64, i64) {
        DIRNS[self as usize]
    }

    /// Apply the appropriate displacement for
    /// this direction to the given point.
    pub fn displace(self, mut p: Point) -> Point {
        let disp = self.disp();
        p.0 += disp.0;
        p.1 += disp.1;
        p
    }

    /// Direction resulting from turning in the given
    /// rotation direction.
    pub fn turn(self, rot: Rot) -> Dirn {
        let offset = match rot {
            Rot::CCW => 1,
            Rot::CW => FACINGS.len() - 1,
        };
        FACINGS[(self as usize + offset) % FACINGS.len()]
    }

    /// Direction resulting from turning around.
    pub fn reverse(self) -> Dirn {
        FACINGS[(self as usize + 2) % FACINGS.len()]
    }
}

#[test]
fn test_rot() {
    use Dirn::*;
    use Rot::*;
    assert_eq!(Left, Up.turn(CCW));
    assert_eq!(Right, Up.turn(CW));
    assert_eq!(Down, Left.turn(CCW));
    assert_eq!(Down, Right.turn(CW));
}
