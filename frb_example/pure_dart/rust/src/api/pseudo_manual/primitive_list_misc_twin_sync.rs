// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `primitive_list_misc.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

pub struct VecOfPrimitivePackTwinSync {
    pub int8list: Vec<i8>,
    pub uint8list: Vec<u8>,
    pub int16list: Vec<i16>,
    pub uint16list: Vec<u16>,
    pub uint32list: Vec<u32>,
    pub int32list: Vec<i32>,
    pub uint64list: Vec<u64>,
    pub int64list: Vec<i64>,
    pub float32list: Vec<f32>,
    pub float64list: Vec<f64>,
    pub bool_list: Vec<bool>,
}

#[flutter_rust_bridge::frb(sync)]
pub fn handle_vec_of_primitive_twin_sync(n: i32) -> VecOfPrimitivePackTwinSync {
    VecOfPrimitivePackTwinSync {
        int8list: vec![42i8; n as usize],
        uint8list: vec![42u8; n as usize],
        int16list: vec![42i16; n as usize],
        uint16list: vec![42u16; n as usize],
        int32list: vec![42i32; n as usize],
        uint32list: vec![42u32; n as usize],
        int64list: vec![42i64; n as usize],
        uint64list: vec![42u64; n as usize],
        float32list: vec![42.0f32; n as usize],
        float64list: vec![42.0f64; n as usize],
        bool_list: vec![true; n as usize],
    }
}

// no explicit ZeroCopyBuffer anymore, it is auto zero copy
// pub struct ZeroCopyVecOfPrimitivePackTwinSync {
//     pub int8list: ZeroCopyBuffer<Vec<i8>>,
//     pub uint8list: ZeroCopyBuffer<Vec<u8>>,
//     pub int16list: ZeroCopyBuffer<Vec<i16>>,
//     pub uint16list: ZeroCopyBuffer<Vec<u16>>,
//     pub uint32list: ZeroCopyBuffer<Vec<u32>>,
//     pub int32list: ZeroCopyBuffer<Vec<i32>>,
//     pub uint64list: ZeroCopyBuffer<Vec<u64>>,
//     pub int64list: ZeroCopyBuffer<Vec<i64>>,
//     pub float32list: ZeroCopyBuffer<Vec<f32>>,
//     pub float64list: ZeroCopyBuffer<Vec<f64>>,
// }
//
// #[flutter_rust_bridge::frb(sync)] pub fn handle_zero_copy_vec_of_primitive_twin_sync(
//     n: i32,
// ) -> ZeroCopyVecOfPrimitivePackTwinSync {
//     ZeroCopyVecOfPrimitivePackTwinSync {
//         int8list: ZeroCopyBuffer(vec![42i8; n as usize]),
//         uint8list: ZeroCopyBuffer(vec![42u8; n as usize]),
//         int16list: ZeroCopyBuffer(vec![42i16; n as usize]),
//         uint16list: ZeroCopyBuffer(vec![42u16; n as usize]),
//         int32list: ZeroCopyBuffer(vec![42i32; n as usize]),
//         uint32list: ZeroCopyBuffer(vec![42u32; n as usize]),
//         int64list: ZeroCopyBuffer(vec![42i64; n as usize]),
//         uint64list: ZeroCopyBuffer(vec![42u64; n as usize]),
//         float32list: ZeroCopyBuffer(vec![42.0f32; n as usize]),
//         float64list: ZeroCopyBuffer(vec![42.0f64; n as usize]),
//     }
// }
