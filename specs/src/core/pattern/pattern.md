```rust
#![allow(
    dead_code,
    unused_imports,
)]
use crate::core::pattern::note::{BaseNote, Note};
use crate::core::pattern::effects::Effect;
use envelope::Envelope;
use envelope::interpolation::Spatial;
use std::iter::{FromIterator, once};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Points<P>(Vec<P>);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gain;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}
impl envelope::Point for Point {
    type X = u32;
    type Y = u32;
    fn x_to_scalar(x: u32) -> <Self::Y as Spatial>::Scalar { x as <Self::Y as Spatial>::Scalar }
    fn x(&self) -> u32 { self.x }
    fn y(&self) -> u32 { self.y }
}
impl<'a> Envelope<'a> for Points<Point> {
    type X = u32;
    type Y = u32;
    type Point = Point;
    type Points = std::slice::Iter<'a, Point>;
    fn points(&'a self) -> Self::Points { self.0.iter() }
}

pub struct Pattern {
    length: u32,
    width: u32,
    notedata: Vec::<Note>,
    voldata: Vec::<Gain>,
    fxdata: Vec::<Vec::<Effect>>,
    // envdata: Vec::<Vec::<dyn 'Envelope>>,
}
```
