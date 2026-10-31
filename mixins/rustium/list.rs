// The mod's row in Options > Mods.
fn rustium_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::rustium as m;

    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("Rustium {}", m::VERSION),
        description: "Toggleable FPS display with Simple & Extended modes (Live, Min, Max, 1% Low).".into(),
        keys: vec![
            [m::KEY_TOGGLE_FPS.into(), "Toggle FPS display".into(), "key.keyboard.f9".into()],
        ],
    });
}
