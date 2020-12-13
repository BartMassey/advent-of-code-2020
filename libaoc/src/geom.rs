//! To use this, make a new `GridBox` to set clipping bounds,
//! then call its methods to get iterators over coordinates.
//!
//! # Examples
//!
//! ```
//! use aoc::geom::*;
//!
//! let clip_box = GridBox::new(4, 4);
//! let mut neighbors = clip_box
//!     .neighbors((2, 0), 1)
//!     .collect::<Vec<_>>();
//! neighbors.sort();
//! let desired = vec![
//!     (1, 0), (1, 1),
//!             (2, 1),
//!     (3, 0), (3, 1),
//! ];
//! assert_eq!(neighbors, desired);
//! ```

use std::marker::PhantomData;

use crate::convert::ConvertInto;
use crate::dirns;

/// Description of the grid, for possible clipping.
#[derive(Copy, Clone)]
pub enum GridBox {
    /// Grid is clipped on bottom and right.
    ClipBox((i64, i64)),
    /// Grid is unclipped.
    Unclipped,
}

use self::GridBox::*;

impl GridBox {
    /// Create a clip box for neighbor calculations.
    #[allow(dead_code)]
    pub fn new<T>(row_size: T, col_size: T) -> GridBox
    where
        T: ConvertInto<i64>,
    {
        ClipBox((row_size.convert_into(), col_size.convert_into()))
    }

    /// Create an "unbounded clip box" for neighbor
    /// calculations.  **Negative locations will still be
    /// clipped.**
    pub fn new_grid() -> GridBox {
        Unclipped
    }

    /// Return an iterator that will produce the neighbors
    /// of the given location, clipped as needed.
    pub fn neighbors<T, U>(
        &self,
        location: (T, T),
        dist: U,
    ) -> Neighbors<T>
    where
        T: ConvertInto<i64>,
        i64: ConvertInto<T>,
        U: ConvertInto<i64>,
    {
        let r = location.0.convert_into();
        let c = location.1.convert_into();
        assert!(r >= 0 && c >= 0);
        if let ClipBox((row_size, col_size)) = *self {
            assert!(r < row_size && c < col_size);
        };
        Neighbors::new(self, (r, c), dist.convert_into())
    }

    /// Return an iterator that will a beam from the
    /// given location in the given direction, stopping
    /// at a grid boundary.
    pub fn beam<'a, T, U>(
        &'a self,
        location: (T, T),
        step: (U, U),
    ) -> Beam<'a, T>
    where
        T: ConvertInto<i64>,
        i64: ConvertInto<T>,
        U: ConvertInto<i64>,
    {
        let r = location.0.convert_into();
        let c = location.1.convert_into();
        let dr = step.0.convert_into();
        let dc = step.1.convert_into();
        Beam::new(self, (r, c), (dr, dc))
    }

    /// Return the source location adjusted by the given offset
    /// iff the dest location is in-bounds. This is useful when
    /// "manual" clipping is needed.
    pub fn clip<T, U>(&self, loc: (T, T), off: (U, U)) -> Option<(T, T)>
    where
        T: ConvertInto<i64>,
        i64: ConvertInto<T>,
        U: ConvertInto<i64>,
    {
        let r = loc.0.convert_into();
        let c = loc.1.convert_into();
        let dr = off.0.convert_into();
        let dc = off.1.convert_into();
        let nr = r + dr;
        let nc = c + dc;
        if nr < 0 || nc < 0 {
            return None;
        }
        if let ClipBox((row_size, col_size)) = *self {
            if nr >= row_size || nc >= col_size {
                return None;
            }
        };
        Some((nr.convert_into(), nc.convert_into()))
    }
}

/// Iterator over the neighbors of a point in the four cardinal
/// directions, clipped as appropriate.
pub struct Neighbors<T> {
    // Origin.
    orig: (i64, i64),
    // Current location.
    loc: (i64, i64),
    // Upper-left corner.
    start: (i64, i64),
    // Lower-right corner.
    end: (i64, i64),
    // Phantom type for iterator.
    phantom: PhantomData<T>,
}

impl<T> Neighbors<T> {
    /// Return an iterator over the neighbors of
    /// the given grid box starting at the given location.
    pub fn new(
        bounds: &GridBox,
        orig: (i64, i64),
        dist: i64,
    ) -> Self {
        assert!(dist > 0);
        let (r, c) = orig;
        let start = (0.max(r - dist), 0.max(c - dist));
        let end = if let ClipBox((rows, cols)) = *bounds {
            (rows.min(r + dist + 1), cols.min(c + dist + 1))
        } else {
            (r + dist + 1, c + dist + 1)
        };
        Neighbors {
            orig,
            loc: start,
            start,
            end,
            phantom: PhantomData,
        }
    }
}

impl<T> Iterator for Neighbors<T>
where
    i64: ConvertInto<T>,
{
    type Item = (T, T);

    /// Return the next neighbor of the source point.
    fn next(&mut self) -> Option<Self::Item> {
        if self.loc == self.orig {
            self.loc.1 += 1;
            return self.next();
        }
        if self.loc.0 >= self.end.0 {
            return None;
        }
        if self.loc.1 >= self.end.1 {
            self.loc = (self.loc.0 + 1, self.start.1);
            return self.next();
        }
        let result =
            (self.loc.0.convert_into(), self.loc.1.convert_into());
        self.loc.1 += 1;
        Some(result)
    }
}


// Low case is taken care of by doctest above.
#[test]
fn test_neighbors_hi() {
    let clip_box = GridBox::new(4, 4);
    let mut neighbors = clip_box
        .neighbors((3, 3), 1)
        .collect::<Vec<_>>();
    neighbors.sort();
    let desired = vec![
        (2, 2), (2, 3),
        (3, 2),
    ];
    assert_eq!(neighbors, desired);
}

/// Beam iterator in a given direction until edge-of-grid is
/// reached.
pub struct Beam<'a, T> {
    // Clipper.
    clip: &'a GridBox,
    // Current location.
    loc: (i64, i64),
    // Step direction.
    step: (i64, i64),
    // Phantom type for iterator.
    phantom: PhantomData<T>,
}

impl<'a, T> Beam<'a, T> {
    /// Return an iterator stepping in the given direction
    /// until edge-of-grid is reached.
    pub fn new(
        clip: &'a GridBox,
        loc: (i64, i64),
        step: (i64, i64),
    ) -> Self {
        assert!(step != (0, 0));
        Beam {
            clip,
            loc,
            step,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Beam<'a, T>
where
    i64: ConvertInto<T>,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.clip
            .clip::<i64, i64>(self.loc, self.step)
            .map(|l| {
                self.loc = l;
                (l.0.convert_into(), l.1.convert_into())
            })
    }
}

#[test]
fn test_beam_infinite() {
    let grid = GridBox::new_grid();

    let beam: Vec<(u8, u8)> = grid
        .beam((5, 2), (1i8, 1))
        .take(4)
        .collect();
    let expected = vec![(6, 3), (7, 4), (8, 5), (9, 6)];

    assert_eq!(beam, expected);
    let beam: Vec<(u8, u8)> = grid
        .beam((5, 2), (1i8, -1))
        .collect();
    let expected = vec![(6, 1), (7, 0)];
    assert_eq!(beam, expected);
}

#[test]
fn test_beam_finite() {
    let grid = GridBox::new(6, 6);
    let beam: Vec<(u8, u8)> = grid.beam((3, 2), (1i8, -1)).collect();
    let expected = vec![(4, 1), (5, 0)];
    assert_eq!(beam, expected);
}

pub fn neighbors4<T>() -> impl Iterator<Item = (T, T)>
where
    i64: ConvertInto<T>,
{
    dirns::DIRNS
        .iter()
        .map(|&(r, c)| (r.convert_into(), c.convert_into()))
}

pub fn neighbors8<T, U>(dist: T) -> impl Iterator<Item = (U, U)>
where
    T: ConvertInto<i64>,
    i64: ConvertInto<U>,
{
    let dist = dist.convert_into();
    assert!(dist > 0);
    (-dist ..= dist)
        .flat_map(move |r| (-dist ..= dist).map(move |c| (r, c)))
        .filter(|&p| p != (0, 0))
        .map(|(r, c)| (r.convert_into(), c.convert_into()))
}

#[test]
fn test_neighbors8() {
    let mut v: Vec<(i8, i8)> = neighbors8(1u8).collect();
    v.sort();
    let desired = vec![
        (-1, -1), (-1,  0), (-1,  1),
        ( 0, -1),           ( 0,  1),
        ( 1, -1), ( 1,  0), ( 1,  1),
    ];
    assert_eq!(v, desired);
}

/// The ["Manhattan Distance"][1] between two points.
///
/// [1]: http://en.wikipedia.org/wiki/Taxicab_geometry
pub fn manhattan_distance<T, U>((r1, c1): (T, T), (r2, c2): (T, T)) -> U
where
    T: ConvertInto<i64>,
    i64: ConvertInto<U>,
{
    let r1 = r1.convert_into();
    let c1 = c1.convert_into();
    let r2 = r2.convert_into();
    let c2 = c2.convert_into();
    let dr = (r1 - r2).abs();
    let dc = (c1 - c2).abs();
    (dr + dc).convert_into()
}
