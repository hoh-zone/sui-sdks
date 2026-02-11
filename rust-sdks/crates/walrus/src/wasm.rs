#[derive(Debug, Clone)]
pub struct WasmBindings;

pub fn get_wasm_bindings(_url: Option<&str>) -> WasmBindings {
    WasmBindings
}
