use crate::{
    brightnessctl,
    spawn_the_thing::spawn_the_thing,
};

#[derive(clap::Parser, Debug)]
/// Display current brightness (probably after changing it)
pub struct Brightness {}

pub fn brightness(_args: Brightness) {
    spawn_the_thing(
        brightnessctl::get_percent(),
        "1;36m",
        "2000",
    );
}
