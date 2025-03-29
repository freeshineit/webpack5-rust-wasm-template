use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// mod g711
pub mod g711 {
    pub struct G711 {}

    const SIGN_BIT: u8 = 0x80; /* Sign bit for a A-law byte. */
    const QUANT_MASK: u8 = 0xf; /* Quantization field mask. */
    const NSEGS: usize = 8; /* Number of A-law segments. */
    const SEG_SHIFT: u8 = 4; /* Left shift for segment number. */
    const SEG_MASK: u8 = 0x70; /* Segment field mask. */
    const BIAS: i16 = 0x84; /* Bias for linear code. */

    static SEG_END: [i16; 8] = [0xFF, 0x1FF, 0x3FF, 0x7FF, 0xFFF, 0x1FFF, 0x3FFF, 0x7FFF];

    /* copy from CCITT G.711 specifications */
    static U2A: [u8; 128] = [
        1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 27, 29, 31, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 46, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 64, 65, 66, 67, 68, 69, 70, 71, 72,
        73, 74, 75, 76, 77, 78, 79, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96,
        97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115,
        116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128,
    ];

    static A2U: [u8; 128] = [
        1, 3, 5, 7, 9, 11, 13, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
        32, 32, 33, 33, 34, 34, 35, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 48, 49,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92,
        93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
        112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
    ];

    impl G711 {
        pub fn search(val: i16, table: &[i16]) -> usize {
            for (i, &entry) in table.iter().enumerate() {
                if val <= entry {
                    return i;
                }
            }
            table.len()
        }

        /**
         * @brief Convert a 16-bit linear PCM value to 8-bit A-law
         *
         * linear2alaw() accepts an 16-bit integer and encodes it as A-law data.
         *
         *   Linear Input Code   Compressed Code
         *  ------------------------    ---------------
         *  0000000wxyza            000wxyz
         *  0000001wxyza            001wxyz
         *  000001wxyzab            010wxyz
         *  00001wxyzabc            011wxyz
         *  0001wxyzabcd            100wxyz
         *  001wxyzabcde            101wxyz
         *  01wxyzabcdef            110wxyz
         *  1wxyzabcdefg            111wxyz
         *
         * For further information see John C. Bellamy's Digital Telephony, 1982,
         * John Wiley & Sons, pps 98-111 and 472-476.
         */
        #[wasm_bindgen]
        pub fn linear2alaw(pcm_val: i16) -> u8 {
            let mask;
            let seg;
            let mut aval;

            let mut pcm_val = if pcm_val >= 0 {
                mask = 0xD5; /* sign (7th) bit = 1 */
                pcm_val
            } else {
                mask = 0x55; /* sign bit = 0 */
                -pcm_val - 8
            };

            /* Convert the scaled magnitude to segment number. */
            seg = G711::search(pcm_val, &SEG_END);

            /* Combine the sign, segment, and quantization bits. */
            if seg >= 8 {
                /* out of range, return maximum value. */
                return 0x7F ^ mask;
            } else {
                aval = (seg << SEG_SHIFT) as u8;
                if seg < 2 {
                    aval |= ((pcm_val >> 4) & QUANT_MASK as i16) as u8;
                } else {
                    aval |= ((pcm_val >> (seg + 3)) & QUANT_MASK as i16) as u8;
                }
                return aval ^ mask;
            }
        }

        /**
         * @brief Convert an A-law value to 16-bit linear PCM
         *
         */

        #[wasm_bindgen]
        pub fn alaw2linear(a_val: u8) -> i16 {
            let t;
            let seg;

            let a_val = a_val ^ 0x55;
            t = ((a_val & QUANT_MASK) << 4) as i16;
            seg = ((a_val & SEG_MASK) >> SEG_SHIFT) as i16;

            let t = match seg {
                0 => t + 8,
                1 => t + 0x108,
                _ => {
                    let mut t = t + 0x108;
                    t <<= seg - 1;
                    t
                }
            };

            if (a_val & SIGN_BIT) != 0 {
                t
            } else {
                -t
            }
        }

        /**
         * @brief Convert a linear PCM value to u-law
         *
         * In order to simplify the encoding process, the original linear magnitude
         * is biased by adding 33 which shifts the encoding range from (0 - 8158) to
         * (33 - 8191). The result can be seen in the following encoding table:
         *
         *  Biased Linear Input Code    Compressed Code
         *  ------------------------    ---------------
         *  00000001wxyza           000wxyz
         *  0000001wxyzab           001wxyz
         *  000001wxyzabc           010wxyz
         *  00001wxyzabcd           011wxyz
         *  0001wxyzabcde           100wxyz
         *  001wxyzabcdef           101wxyz
         *  01wxyzabcdefg           110wxyz
         *  1wxyzabcdefgh           111wxyz
         *
         * Each biased linear code has a leading 1 which identifies the segment
         * number. The value of the segment number is equal to 7 minus the number
         * of leading 0's. The quantization interval is directly available as the
         * four bits wxyz.  * The trailing bits (a - h) are ignored.
         *
         * Ordinarily the complement of the resulting code word is used for
         * transmission, and so the code word is complemented before it is returned.
         *
         * For further information see John C. Bellamy's Digital Telephony, 1982,
         * John Wiley & Sons, pps 98-111 and 472-476.
         */

        #[wasm_bindgen]
        pub fn linear2ulaw(pcm_val: i16) -> u8 {
            let mask;
            let seg;
            let uval;

            /* Get the sign and the magnitude of the value. */
            let pcm_val = if pcm_val < 0 {
                mask = 0x7F;
                BIAS - pcm_val
            } else {
                mask = 0xFF;
                pcm_val + BIAS
            };

            /* Convert the scaled magnitude to segment number. */
            seg = search(pcm_val, &SEG_END);

            /*
             * Combine the sign, segment, quantization bits;
             * and complement the code word.
             */
            if seg >= 8 {
                /* out of range, return maximum value. */
                0x7F ^ mask
            } else {
                uval = (seg << 4) | ((pcm_val >> (seg + 3)) & 0xF) as u8;
                uval ^ mask
            }
        }

        /**
         * @brief Convert a u-law value to 16-bit linear PCM
         *
         * First, a biased linear code is derived from the code word. An unbiased
         * output can then be obtained by subtracting 33 from the biased code.
         *
         * Note that this function expects to be passed the complement of the
         * original code word. This is in keeping with ISDN conventions.
         */

        #[wasm_bindgen]
        pub fn ulaw2linear(u_val: u8) -> i16 {
            let t;

            /* Complement to obtain normal u-law value. */
            let u_val = !u_val;

            /*
             * Extract and bias the quantization bits. Then
             * shift up by the segment number and subtract out the bias.
             */
            t = (((u_val & QUANT_MASK) << 3) + BIAS) as i16;
            t <<= ((u_val & SEG_MASK) >> SEG_SHIFT) as i16;

            if (u_val & SIGN_BIT) != 0 {
                BIAS - t
            } else {
                t - BIAS
            }
        }

        /**
         * @brief A-law to u-law conversion
         *
         * @param aval A-law value
         * @return unsigned char u-law value
         */
        pub fn alaw2ulaw(aval: u8) -> u8 {
            let aval = aval & 0xff;
            if (aval & 0x80) != 0 {
                0xFF ^ A2U[(aval ^ 0xD5) as usize]
            } else {
                0x7F ^ A2U[(aval ^ 0x55) as usize]
            }
        }

        /**
         * @brief u-law to A-law conversion
         *
         * @param uval u-law value
         * @return unsigned char A-law value
         */

        #[wasm_bindgen]
        pub fn ulaw2alaw(uval: u8) -> u8 {
            let uval = uval & 0xff;
            if (uval & 0x80) != 0 {
                0xD5 ^ (U2A[(0xFF ^ uval) as usize] - 1)
            } else {
                0x55 ^ (U2A[(0x7F ^ uval) as usize] - 1)
            }
        }

        /**
         * @brief pcm data encode to g711 data
         *
         *  user should be responsible for pCodecbit memmory
         *
         * @param pCodecBits store g711 encoded data
         * @param pBuffer pcm raw data
         * @param BufferSize pcm data len
         * @param type g711 data type
         * @return int encode data length
         */

        #[wasm_bindgen]
        pub fn encode(
            p_codec_bits: &mut [u8],
            p_buffer: &[i16],
            buffer_size: usize,
            gtype: i32,
        ) -> i32 {
            if p_codec_bits.is_empty() || p_buffer.is_empty() || buffer_size <= 0 {
                return -1;
            }

            if gtype == 1 {
                for i in 0..buffer_size / 2 {
                    p_codec_bits[i] = linear2alaw(p_buffer[i]);
                }
            } else {
                for i in 0..buffer_size / 2 {
                    p_codec_bits[i] = linear2ulaw(p_buffer[i]);
                }
            }

            (buffer_size / 2) as i32
        }

        /**
         * @brief g711 data decode to pcm data
         *
         * user should be responsible for pRawData memmory
         *
         * @param pRawData store uncoded pcm data
         * @param pBuffer g711 encoded data
         * @param BufferSize g711 data len
         * @param type g711 data type
         * @return int pcm data len
         */
        #[wasm_bindgen]
        pub fn decode(
            p_raw_data: &mut [i16],
            p_buffer: &[u8],
            buffer_size: usize,
            gtype: i32,
        ) -> i32 {
            if p_raw_data.is_empty() || p_buffer.is_empty() || buffer_size <= 0 {
                return -1;
            }

            if gtype == 1 {
                for i in 0..buffer_size {
                    p_raw_data[i] = alaw2linear(p_buffer[i]);
                }
            } else {
                for i in 0..buffer_size {
                    p_raw_data[i] = ulaw2linear(p_buffer[i]);
                }
            }

            (buffer_size * 2) as i32
        }

        /**
         * @brief g711 u-law data and a-law data convert
         *
         * @param alawdata g711 a-law data
         * @param ulawdata g711 u-lwa data
         * @param datasize input data length
         * @param type target g711 data type
         * @return int sucess:1; failed:0
         */
        #[wasm_bindgen]
        pub fn type_change(
            alawdata: &mut [u8],
            ulawdata: &[u8],
            datasize: usize,
            gtype: i32,
        ) -> i32 {
            if alawdata.is_empty() || ulawdata.is_empty() || datasize <= 0 {
                return 0;
            }

            if gtype == 1 {
                for i in 0..datasize {
                    alawdata[i] = ulaw2alaw(ulawdata[i]);
                }
            } else {
                for i in 0..datasize {
                    ulawdata[i] = alaw2ulaw(alawdata[i]);
                }
            }
            1
        }
    }
}
