#![no_main]
use libfuzzer_sys::fuzz_target;
use scrap::{VpxDecoder, VpxDecoderConfig, VpxVideoCodecId};

fuzz_target!(|data: (u8, &[u8])| {
    let (meta, data) = data;
    let config = VpxDecoderConfig {
        codec: if meta & 1 == 1 {
            VpxVideoCodecId::VP8
        } else {
            VpxVideoCodecId::VP9
        },
        num_threads: 1,
    };

    if let Ok(mut decoder) = VpxDecoder::new(config) {
        let b = meta & 2 == 2;
        let _ = decoder.decode2rgb(data, b);
    }
});
