use std::ffi::CString;

use alsa::ctl::Ctl;
use alsa::mixer::{Mixer, SelemChannelId, SelemId};
use alsa::poll::poll;
use alsa::PollDescriptors;

fn print_volume() {
    let mut mixer: Mixer;
    let selem = loop {
        mixer = Mixer::new("default", false).unwrap();
        let selem_id = SelemId::new("Master", 0);
        if let Some(selem) = mixer.find_selem(&selem_id) {
            break selem;
        }
    };
    let (min, max) = selem.get_playback_volume_range();
    let volume = selem
        .get_playback_volume(SelemChannelId::FrontLeft)
        .unwrap();
    let p_volume = (volume as f64 / (max - min) as f64 * 100.0).round();
    let switch = selem
        .get_playback_switch(SelemChannelId::FrontLeft)
        .unwrap();

    if switch == 0 {
        println!("󰸈 mute");
        return;
    }
    if p_volume == 0.0 {
        println!("󰕿  {}%", p_volume);
        return;
    }
    if p_volume < 50.0 {
        println!("󰖀  {}%", p_volume);
        return;
    }
    if p_volume > 50.0 {
        println!("󰕾  {}%", p_volume);
    }
}

fn main() {
    let ctl = Ctl::open(CString::new("default").unwrap().as_ref(), false).unwrap();
    ctl.subscribe_events(true).unwrap();
    let mut notify_fds = ctl.get().unwrap();
    loop {
        poll(&mut notify_fds, -1).unwrap();
        if let Ok(Some(_)) = ctl.read() {
            print_volume();
        }
    }
}
