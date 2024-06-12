use super::IntoDart;
use crate::platform_types::DartAbi;

pub trait BoxIntoDart {
    fn box_into_dart(self: Box<Self>) -> DartAbi;
}

impl<T: IntoDart> BoxIntoDart for T {
    fn box_into_dart(self: Box<Self>) -> DartAbi {
        (*self).into_dart()
    }
}
