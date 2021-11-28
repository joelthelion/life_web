use macroquad::prelude::*;
use oorandom::Rand32;

mod biot;
mod biot_collection;

use biot_collection::BiotCollection;

#[macroquad::main("Life")]
async fn main() {

    let mut seed: [u8; 8] = [0; 8];
    getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
    let mut rng = Rand32::new(u64::from_ne_bytes(seed));
    let mut biots = BiotCollection::new(600, &mut rng);

    loop {
        biots.step(&mut rng);
        clear_background(Color::new(0.,0.,0.1,1.0));
        biots.draw();
        draw_text(&format!("FPS: {}, biots: {}", get_fps(), biots.len()),
            screen_width()-200., screen_height()-5.,
            18.,
            LIGHTGRAY);
        next_frame().await
    }
}
