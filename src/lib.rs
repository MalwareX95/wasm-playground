use wasm_bindgen::prelude::*;
use rayon::prelude::*;
use core::arch::*;
use js_sys::Array;
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn execute_parallel() -> Array {
    let (a, b) = rayon::join(
        || (0..10000).map(|_| rayon::current_thread_index().unwrap()).collect::<Vec<usize>>(),
        || (0..10000).map(|_| rayon::current_thread_index().unwrap()).collect::<Vec<usize>>());

    Array::of2(
        &a.iter().copied().map(JsValue::from).collect::<Array>(),
        &b.iter().copied().map(JsValue::from).collect::<Array>())
}

// pub fn test_mm_mul_epu32() {
//     let x = wasm32::u64x2(0x0264_432C_CD8A_70E0, 0x0B28_E3EF_EBB3_172D);
//     let y = wasm32::u64x2(0x0B28_E3EF_EBB3_172D, 0x0264_432C_CD8A_70E0);
//     let z = _mm_mul_epu32(x, y);
//     let result: i32 = z[0].into();
// }

#[wasm_bindgen]
pub fn _mm_mul_epu32() -> u32 {
    let a = wasm32::u32x4(10, 20, 30, 40);
    let b = wasm32::u32x4_splat(1);
    let c = wasm32::u32x4_add(a, b);
    wasm32::u32x4_extract_lane::<0>(c)
}

pub fn i8x16_swizzle() -> wasm::v128 {
    let a = wasm32::i8x16(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
    let s = wasm32::i16x8_splat(1);
    wasm32::i8x16_swizzle(a, s)
}