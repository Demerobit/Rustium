// Runs every frame — tracks elapsed time and FPS.
fn rustium_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use crate::game::rustium as m;
    if !game.native_on(m::ID) { return; }
    m::bump_frame(elapsed);
}
