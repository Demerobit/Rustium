// Reacts to key presses: toggle FPS display mode (Simple -> Extended -> Off).
fn rustium_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::rustium as m;

    if !game.native_on(m::ID) || !pressed { return; }

    // Toggle FPS mode key (default: F9)
    let toggle_key = game.native_mod_key(m::ID, m::KEY_TOGGLE_FPS, "key.keyboard.f9");
    if key == toggle_key {
        let current = game.native_setting(m::ID, m::SETTING_FPS_MODE)
            .as_deref()
            .and_then(m::FpsMode::from_str_opt)
            .unwrap_or(m::FpsMode::Simple);

        let next = current.next();
        game.native_set_setting(m::ID, m::SETTING_FPS_MODE, next.as_str());
        game.native_chat(&format!("Rustium FPS: {}", next.display()));
    }
}
