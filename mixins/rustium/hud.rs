// Draws the Rustium toggleable FPS overlay on the HUD,
// and saves the finished HUD batch to the cache for HUD framerate throttling.
fn rustium_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::rustium as m;
    use horizon_client::extras::Corner;

    if !game.native_on(m::ID) { return; }

    // Must be playing (in a world, no screen over it)
    if !game.native_playing() { return; }

    // Check FPS mode (Off, Simple, Extended)
    let mode_str = game.native_setting(m::ID, m::SETTING_FPS_MODE);
    let mode = mode_str.as_deref()
        .and_then(m::FpsMode::from_str_opt)
        .unwrap_or(m::FpsMode::Simple);

    if mode != m::FpsMode::Off {
        let (live, min, low1) = game.native_frame_rate();
        let max = m::current_max_fps();
        m::rebuild_hud(live, min, max, low1, mode);

        let cache = m::hud_snapshot();
        if !cache.line1.is_empty() {
            let corner = game.native_setting(m::ID, m::SETTING_HUD_CORNER)
                .and_then(|s| m::HudCorner::from_str_opt(&s))
                .unwrap_or(m::HudCorner::TopRight);

            let h_corner = match corner {
                m::HudCorner::TopRight => Corner::TopRight,
                m::HudCorner::TopLeft  => Corner::TopLeft,
                m::HudCorner::BotRight => Corner::BottomRight,
                m::HudCorner::BotLeft  => Corner::BottomLeft,
            };

            let line = &cache.line1;
            let line_h  = 10.0_f32;
            let padding = 3.0_f32;
            let box_h   = line_h + padding * 2.0;
            let box_w   = game.native_text_width(line) + padding * 2.0;

            let (bx, by) = corners.place(h_corner, box_w, box_h);
            game.native_fill(batch, [bx, by, box_w, box_h], 0x7800_0000);
            game.native_text(batch, line, [bx + padding, by + padding], 0xFF_FFFF, true);
        }
    }

    // Save batch for HUD framerate throttling
    m::save_hud_batch(batch);
}
