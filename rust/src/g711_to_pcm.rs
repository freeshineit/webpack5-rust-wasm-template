use crate::g711::G711;
use std::ptr;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Constant for PCM size
const CON_PCM_SIZE: i32 = 160; // Assuming a constant value for PCM size. Adjust as necessary.

// Constant for G711 frame size
const G711_ONE_LEN: i32 = 160; // Assuming a constant value for G711 frame size. Adjust as necessary.

// Enum for G711 type
#[derive(Copy, Clone)]
enum G711Type {
    G711A = 1,
    G711U = 2,
}

// Structure to hold input audio information
struct InputAudioInfo {
    codec_type: i32, // Codec type
}

impl InputAudioInfo {
    // Method to get codec type
    fn codec_type(&self) -> i32 {
        self.codec_type
    }
}

// Structure for G711 to PCM conversion
struct G711ToPcm {
    m_type: G711Type,   // G711 type
    m_g7FrameSize: i32, // G711 frame size
}

impl G711ToPcm {
    // Constructor
    fn new() -> Self {
        println!("G711ToPcm constructor");
        G711ToPcm {
            m_type: G711Type::G711A,
            m_g7FrameSize: 0,
        }
    }

    // Method to set the origin type
    fn set_origin_type(&mut self, gtype: G711Type) {
        self.m_type = gtype;
    }

    // Method to decode G711 data to PCM
    fn decode(
        &self,
        out_buffer: &mut [u8],
        out_len: &mut u32,
        input_buffer: &[u8],
        in_buffer_len: usize,
    ) -> i32 {
        let result = G711::decode(out_buffer, input_buffer, in_buffer_len, self.m_type as i32);
        *out_len = result as u32;
        result
    }

    // Method to encode PCM data to G711
    fn encode(
        &self,
        out_buffer: &mut [u8],
        out_len: &mut u32,
        input_buffer: &[u8],
        in_buffer_len: usize,
    ) -> i32 {
        let result = G711::encode(out_buffer, input_buffer, in_buffer_len, self.m_type as i32);
        *out_len = result as u32;
        result
    }

    // Method to get PCM size
    fn pcm_size(&self) -> i32 {
        CON_PCM_SIZE
    }

    // Method to initialize the G711 type based on input audio information
    fn init(&mut self, info: InputAudioInfo) {
        if info.codec_type() == 1 {
            self.set_origin_type(G711Type::G711A);
        } else {
            self.set_origin_type(G711Type::G711U);
        }
        self.m_g7FrameSize = G711_ONE_LEN;
    }

    // Method to get G711 frame size
    fn g7_frame_size(&self) -> i32 {
        self.m_g7FrameSize
    }
}
