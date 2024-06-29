use alsa::card;
use alsa::ctl::Ctl;
use alsa::mixer::{Mixer, SelemChannelId, SelemId};
use std::ffi::CString;

fn print_volume() {
    let mixer = Mixer::new("default", false).unwrap();
    let selem_id = SelemId::new("Master", 0);
    let selem = mixer.find_selem(&selem_id).unwrap();
    let (min, max) = selem.get_playback_volume_range();
    let volume = selem
        .get_playback_volume(SelemChannelId::FrontLeft)
        .unwrap();
    let p_volume = ((volume as f64 / (max - min) as f64) as f32 * 100.0).round();
    let switch = selem
        .get_playback_switch(SelemChannelId::FrontLeft)
        .unwrap();

    if switch == 0 {
        println!("󰸈");
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
    print_volume();
    let ctl = Ctl::open(CString::new("default").unwrap().as_ref(), false).unwrap();
    ctl.subscribe_events(true).unwrap();
    for c in card::Iter::new() {
        let ctl = Ctl::from_card(&c.unwrap(), false).unwrap();
        ctl.subscribe_events(true).unwrap();
        while let Some(maybe_event) = ctl.read().transpose() {
            let event = maybe_event.unwrap();
            if event.get_mask().value() {
                print_volume();
            }
        }
    }
}
