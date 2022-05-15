use wasm_bindgen_test::*;
use wasm_rayon::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    wasm_rayon::_mm_mul_epu32();
    assert_eq!(1 + 1, 3);
}
