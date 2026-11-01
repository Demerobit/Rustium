fn rustium_entity_culling(
    game: &mut crate::game::Game<'_>,
    camera: glam::DVec3,
) -> core::ops::ControlFlow<()> {
    use crate::game::rustium as m;

    if !game.native_on(m::ID) {
        return core::ops::ControlFlow::Continue(());
    }

    let enabled = game
        .native_setting(m::ID, m::SETTING_ENTITY_CULLING)
        .as_deref() != Some("false");

    if !enabled {
        game.entity_culler = None;
        return core::ops::ControlFlow::Break(());
    }

    game.pack_shadow_casters = match (&game.renderer, &game.pack_ids) {
        (Some(r), Some(_)) => r.pack_shadow_casters(),
        _ => (false, false),
    };

    if game.player.is_some() && !game.spectator() {
        game.entity_culler = Some(horizon_client::entity_culling::Culler::new(camera.to_array(), 8_192));
    } else {
        game.entity_culler = None;
    }

    core::ops::ControlFlow::Break(())
}
