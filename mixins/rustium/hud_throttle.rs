fn rustium_hud_throttle(
    game: &mut crate::game::Game<'_>,
    size: [u32; 2],
) -> core::ops::ControlFlow<HudBatch> {
    use crate::game::rustium as m;

    if !game.native_on(m::ID) {
        return core::ops::ControlFlow::Continue(());
    }

    let rate_hz: u32 = game
        .native_setting(m::ID, m::SETTING_HUD_RATE)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    // 0 = Real-time / full uncapped FPS (no throttling)
    if rate_hz == 0 {
        return core::ops::ControlFlow::Continue(());
    }

    if let Some(batch) = m::try_reuse_hud_batch(rate_hz, size) {
        core::ops::ControlFlow::Break(batch)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
