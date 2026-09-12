use std::io::Cursor;
use ogg::writing::{PacketWriter, PacketWriteEndInfo};
use rusty_vorbis::{VorbisEncoder, VorbisEncoderConfig};

/// Checks whether the audio bytes already start with the Ogg container magic header "OggS".
pub fn is_ogg(data: &[u8]) -> bool {
    data.len() >= 4 && &data[0..4] == b"OggS"
}

/// Encodes raw interleaved 32-bit float PCM samples to a valid Ogg Vorbis byte vector.
pub fn encode_pcm_to_ogg(
    interleaved_pcm: &[f32],
    channels: u16,
    sample_rate: u32,
) -> Result<Vec<u8>, String> {
    if interleaved_pcm.is_empty() || channels == 0 || sample_rate == 0 {
        return Err("Invalid PCM input: empty or zero channels/sample_rate".to_string());
    }

    // Vorbis encoder profile is configured for stereo (2 channels).
    // Normalize mono or multi-channel audio to standard stereo:
    let (stereo_pcm, enc_channels) = if channels == 1 {
        let mut stereo = Vec::with_capacity(interleaved_pcm.len() * 2);
        for &sample in interleaved_pcm {
            stereo.push(sample);
            stereo.push(sample);
        }
        (stereo, 2u16)
    } else if channels > 2 {
        let n_frames = interleaved_pcm.len() / channels as usize;
        let mut stereo = Vec::with_capacity(n_frames * 2);
        for frame in 0..n_frames {
            stereo.push(interleaved_pcm[frame * channels as usize]);
            stereo.push(interleaved_pcm[frame * channels as usize + 1]);
        }
        (stereo, 2u16)
    } else {
        (interleaved_pcm.to_vec(), channels)
    };

    let mut enc = VorbisEncoder::new(VorbisEncoderConfig::default());
    enc.push_pcm_f32(&stereo_pcm, enc_channels, sample_rate)
        .map_err(|e| format!("VorbisEncoder push error: {e:?}"))?;
    enc.finish();

    let mut out = Vec::new();
    let serial = 0x544F544D; // "TOTM" serial number for stream
    let mut writer = PacketWriter::new(Cursor::new(&mut out));

    let mut packets = Vec::new();
    while let Ok(pkt) = enc.next_packet() {
        packets.push(pkt);
    }

    if packets.is_empty() {
        return Err("No Vorbis packets were produced by the encoder.".to_string());
    }

    let total = packets.len();
    for (i, pkt) in packets.into_iter().enumerate() {
        let absgp = pkt.pts.max(0) as u64;
        let end_info = if i == 0 {
            // First header packet (ident) must be in its own page per Vorbis spec
            PacketWriteEndInfo::EndPage
        } else if i == 1 {
            // Second header packet (comment)
            PacketWriteEndInfo::NormalPacket
        } else if i == 2 {
            // Third header packet (setup) finishes the header pages
            PacketWriteEndInfo::EndPage
        } else if i == total - 1 {
            // Last audio packet ends the stream
            PacketWriteEndInfo::EndStream
        } else {
            PacketWriteEndInfo::NormalPacket
        };

        writer
            .write_packet(pkt.data, serial, end_info, absgp)
            .map_err(|e| format!("Ogg write packet error: {e:?}"))?;
    }

    Ok(out)
}
