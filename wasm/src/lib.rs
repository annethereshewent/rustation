extern crate rsx;

use rsx::cpu::CPU;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
pub struct WasmEmulator {
  cpu: CPU
}

#[wasm_bindgen]
impl WasmEmulator {
  #[wasm_bindgen(constructor)]
  pub fn new(bios: &[u8], game_data: &[u8]) -> Self {
    Self {
      cpu: CPU::new(bios.to_vec(), game_data.to_vec())
    }
  }
  pub fn run_frame(&mut self) {
    self.cpu.run_frame();
  }

  pub fn get_framebuffer(&self) -> *const u8 {
    self.cpu.bus.gpu.picture.as_ptr()
  }

  pub fn update_audio_buffers() {

  }
}