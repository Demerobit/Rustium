// Handles button clicks on the Rustium settings page.
fn rustium_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::rustium as m;

    if mod_id != m::ID {
        return;
    }

    match button {
        // 0: Cycle FPS Display Mode
        0 => {
            let mode = game
                .native_setting(m::ID, m::SETTING_FPS_MODE)
                .as_deref()
                .and_then(m::FpsMode::from_str_opt)
                .unwrap_or(m::FpsMode::Simple);
            let next = mode.next();
            game.native_set_setting(m::ID, m::SETTING_FPS_MODE, next.as_str());
        }

        // 1: Toggle Entity Culling (Sodium-style occlusion)
        1 => {
            let current_on = game
                .native_setting(m::ID, m::SETTING_ENTITY_CULLING)
                .as_deref() != Some("false");
            let next_val = if current_on { "false" } else { "true" };
            game.native_set_setting(m::ID, m::SETTING_ENTITY_CULLING, next_val);
        }

        // 2: Cycle UI & HUD FPS Throttle (Uncapped -> 30 FPS -> 60 FPS -> 15 FPS -> Uncapped)
        2 => {
            let current: u32 = game
                .native_setting(m::ID, m::SETTING_HUD_RATE)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            let next = match current {
                0  => 30,
                30 => 60,
                60 => 15,
                15 => 0,
                _  => 0,
            };
            game.native_set_setting(m::ID, m::SETTING_HUD_RATE, &next.to_string());
        }

        // 3: Cycle Corner
        3 => {
            let current = game
                .native_setting(m::ID, m::SETTING_HUD_CORNER)
                .unwrap_or_else(|| "tr".into());
            let next = match current.as_str() {
                "tr" => "tl",
                "tl" => "bl",
                "bl" => "br",
                _    => "tr",
            };
            game.native_set_setting(m::ID, m::SETTING_HUD_CORNER, next);
        }

        _ => {}
    }
}
