// Renders buttons for the Rustium settings page in Options > Mods > Rustium > Settings.
fn rustium_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::rustium as m;

    if mod_id != m::ID {
        return;
    }

    // 0: FPS Display Mode (Simple, Extended, Off)
    let mode = game
        .native_setting(m::ID, m::SETTING_FPS_MODE)
        .as_deref()
        .and_then(m::FpsMode::from_str_opt)
        .unwrap_or(m::FpsMode::Simple);
    buttons.push(format!("FPS Display: {}", mode.display()));

    // 1: UI & HUD FPS Throttle (Limits F3, Hotbar, Menus & UI FPS)
    let rate_hz: u32 = game
        .native_setting(m::ID, m::SETTING_HUD_RATE)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let rate_label = match rate_hz {
        15 => "15 FPS (Max GPU savings)".to_string(),
        30 => "30 FPS (Smooth UI)".to_string(),
        60 => "60 FPS (Fast UI)".to_string(),
        0  => "Uncapped (Real-time)".to_string(),
        n  => format!("{n} FPS"),
    };
    buttons.push(format!("UI & HUD FPS: {rate_label}"));

    // 2: HUD Position
    let corner_str = game
        .native_setting(m::ID, m::SETTING_HUD_CORNER)
        .unwrap_or_else(|| "tr".into());
    let corner_label = match corner_str.as_str() {
        "tl" => "Top-Left",
        "br" => "Bottom-Right",
        "bl" => "Bottom-Left",
        _    => "Top-Right",
    };
    buttons.push(format!("HUD Position: {corner_label}"));
}
