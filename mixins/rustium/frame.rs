// Runs every frame. Bumps the frame counter and tracks timing.
fn rustium_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use crate::game::rustium as m;
    if !game.native_on(m::ID) { return; }

    m::bump_frame(elapsed);
}
