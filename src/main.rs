use std::{fs::File, io::Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = &args[1];
    let output_path = &args[2];
    let (
        ats_usb_old::packet::GeneralConfig {
            impact_threshold,
            accel_odr,
            camera_model_nf,
            camera_model_wf,
            stereo_iso,
            uuid: _,
        },
        pkts,
    ) = ats_playback_old::read_file(&input_path.into()).unwrap();

    let new_config = ats_usb_master::packet::GeneralConfig {
        impact_threshold,
        camera_model_nf,
        camera_model_wf,
        stereo_iso,
        accel_config: ats_usb_master::packet::AccelConfig {
            accel_odr,
            b_x: 0.0,
            b_y: 0.0,
            b_z: 0.0,
            s_x: 1.0,
            s_y: 1.0,
            s_z: 1.0,
        },
    };

    let mut bytes = vec![];
    new_config.serialize(&mut bytes);
    for (timestamp, packet) in pkts {
        bytes.extend_from_slice(&timestamp.to_le_bytes());
        let i = bytes.len();
        packet.serialize(&mut bytes);
        if bytes[i + 2] > 5 {
            bytes[i + 2] += 2;
        }
    }

    let mut output_file = File::create_new(output_path).unwrap();
    output_file.write_all(&bytes).unwrap();
}
