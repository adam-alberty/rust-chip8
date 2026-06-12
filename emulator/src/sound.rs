use rodio::{
    MixerDeviceSink, Player,
    source::{Source, SquareWave},
};

pub fn create_beep_sink() -> (Player, MixerDeviceSink) {
    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

    let player = rodio::Player::connect_new(&handle.mixer());
    let source = SquareWave::new(1000.0).amplify(0.25).repeat_infinite();

    player.append(source);

    (player, handle)
}
