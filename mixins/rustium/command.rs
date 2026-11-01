// Handles the /rustium command suite.
fn rustium_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::rustium as m;

    // Strip prefix — leave other commands alone
    let Some(rest) = command.strip_prefix("rustium") else { return; };
    // Ensure it is exactly "rustium" or "rustium <subcommand>" (not "rustiumXYZ")
    if !rest.is_empty() && !rest.starts_with(' ') { return; }

    *ran = true;
    if !game.native_on(m::ID) {
        game.native_chat("Rustium is disabled. Enable it in Options > Mods.");
        return;
    }

    match m::parse_cmd(rest) {
        m::Cmd::Status => {
            let mode = game.native_setting(m::ID, m::SETTING_FPS_MODE)
                .as_deref()
                .and_then(m::FpsMode::from_str_opt)
                .unwrap_or(m::FpsMode::Simple);
            let (live, min, low1) = game.native_frame_rate();
            let max = m::current_max_fps();
            let rate: u32 = game.native_setting(m::ID, m::SETTING_HUD_RATE)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            let ui_fps = if rate == 0 { "Uncapped".into() } else { format!("{rate} FPS") };
            game.native_chat(&format!(
                "Rustium {ver} | Mode: {m} | UI Limit: {ui_fps} | Live: {live} | Min: {min} | Max: {max} | 1%: {low1}",
                ver = m::VERSION,
                m   = mode.display(),
            ));
        }

        m::Cmd::SetFpsMode(mode) => {
            game.native_set_setting(m::ID, m::SETTING_FPS_MODE, mode.as_str());
            game.native_chat(&format!("Rustium FPS mode set to: {}", mode.display()));
        }

        m::Cmd::CycleFpsMode => {
            let mode = game.native_setting(m::ID, m::SETTING_FPS_MODE)
                .as_deref()
                .and_then(m::FpsMode::from_str_opt)
                .unwrap_or(m::FpsMode::Simple);
            let next = mode.next();
            game.native_set_setting(m::ID, m::SETTING_FPS_MODE, next.as_str());
            game.native_chat(&format!("Rustium FPS: {}", next.display()));
        }

        m::Cmd::SetRate(rate) => {
            game.native_set_setting(m::ID, m::SETTING_HUD_RATE, &rate.to_string());
            let label = if rate == 0 {
                "Uncapped (Real-time)".to_string()
            } else {
                format!("{rate} FPS")
            };
            game.native_chat(&format!("Rustium UI & HUD framerate limit: {label}"));
        }

        m::Cmd::SetCorner(c) => {
            game.native_set_setting(m::ID, m::SETTING_HUD_CORNER, &c);
            game.native_chat(&format!("Rustium HUD position set to: {c}"));
        }

        m::Cmd::Reset => {
            for key in [
                m::SETTING_FPS_MODE,
                m::SETTING_HUD_RATE,
                m::SETTING_HUD_CORNER,
            ] {
                game.native_set_setting(m::ID, key, "");
            }
            game.native_chat("Rustium: Settings reset to defaults.");
        }

        m::Cmd::Help => {
            for line in [
                "§eRustium commands:",
                "  /rustium                  — show live stats & mode",
                "  /rustium fps              — cycle mode (simple / extended / off)",
                "  /rustium fps <simple|extended|off> — set mode directly",
                "  /rustium rate <fps>       — set UI/HUD FPS limit (e.g. 30, 60, or 0 for uncapped)",
                "  /rustium corner <tr|tl|br|bl> — set HUD corner",
                "  /rustium reset            — restore defaults",
            ] {
                game.native_chat(line);
            }
        }

        m::Cmd::Unknown(err) => {
            game.native_chat(&format!("§cRustium: {err}"));
        }
    }
}
