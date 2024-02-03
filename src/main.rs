use kart_apple_gl::core::application::KartApple;

mod app_scaff;

fn main() {
    unsafe {
        KartApple::start(
            app_scaff::PhysicsScaffold::new(),
            500, 500, "window"
        )
    }
}
