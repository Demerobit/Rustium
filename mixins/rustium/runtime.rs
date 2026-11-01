// Rustium — runtime module
// Added to the game as crate::game::rustium.

use std::cell::Cell;
use std::time::Duration;

pub const ID: &str = "rustium";
pub const VERSION: &str = "0.1.0";

// ── Thread-local timing ───────────────────────────────────────────────────────

thread_local! {
    static TOTAL_TIME: Cell<f64> = const { Cell::new(0.0) };
    static LIVE_FPS:   Cell<u32> = const { Cell::new(0) };
}

pub fn bump_frame(elapsed: Duration) {
    let dt = elapsed.as_secs_f64();
    TOTAL_TIME.with(|t| t.set(t.get() + dt));

    let fps = if dt > 0.0001 {
        (1.0 / dt).round().min(9999.0) as u32
    } else {
        0
    };
    if fps > 0 {
        LIVE_FPS.with(|f| f.set(fps));
    }
}

pub fn live_fps() -> u32 {
    LIVE_FPS.with(|f| f.get())
}
