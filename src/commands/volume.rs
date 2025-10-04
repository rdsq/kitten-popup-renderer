use crate::{
    amixer::get_volume,
    spawn_the_thing::spawn_the_thing,
};

#[derive(clap::Parser, Debug)]
/// Display current audio volume (probably after changing it)
pub struct Volume {}

pub fn volume(_args: Volume) {
    let (percent, muted) = get_volume();
    spawn_the_thing(
        percent,
        if muted { "1;33m" } else { "1;32m" },
        "2000",
    );
}
