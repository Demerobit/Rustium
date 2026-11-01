// Rustium — runtime module
// Added to the game as crate::game::rustium.
// Also acts as the Cargo library so `cargo test` runs without a game.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::time::Duration;

pub const ID: &str = "rustium";
pub const VERSION: &str = "0.1.0";

// ── Rebindable key names ───────────────────────────────────────────────────────
pub const KEY_TOGGLE_FPS: &str = "toggle_fps";

// ── Settings keys ─────────────────────────────────────────────────────────────
pub const SETTING_FPS_MODE:   &str = "fps_mode";   // "off" | "simple" | "extended"
pub const SETTING_HUD_RATE:   &str = "hud_rate";   // "1" | "5" | "10" | "20" | "30" | "60"
pub const SETTING_HUD_CORNER: &str = "hud_corner"; // "tr" | "tl" | "br" | "bl"

// ── FPS Display Mode ─────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum FpsMode {
    Off,
    #[default]
    Simple,
    Extended,
}

impl FpsMode {
    pub fn as_str(self) -> &'static str {
        match self {
            FpsMode::Off      => "off",
            FpsMode::Simple   => "simple",
            FpsMode::Extended => "extended",
        }
    }

    pub fn display(self) -> &'static str {
        match self {
            FpsMode::Off      => "OFF",
            FpsMode::Simple   => "Simple (Live only)",
            FpsMode::Extended => "Extended (Live, Min, Max, 1% Low)",
        }
    }

    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "off" | "hide" | "hidden" => Some(FpsMode::Off),
            "simple" | "live"         => Some(FpsMode::Simple),
            "extended" | "full" | "ext" => Some(FpsMode::Extended),
            _ => None,
        }
    }

    pub fn next(self) -> Self {
        match self {
            FpsMode::Simple   => FpsMode::Extended,
            FpsMode::Extended => FpsMode::Off,
            FpsMode::Off      => FpsMode::Simple,
        }
    }
}

// ── HUD corner ────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HudCorner {
    TopRight,
    TopLeft,
    BotRight,
    BotLeft,
}

impl HudCorner {
    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s.trim() {
            "tr" => Some(HudCorner::TopRight),
            "tl" => Some(HudCorner::TopLeft),
            "br" => Some(HudCorner::BotRight),
            "bl" => Some(HudCorner::BotLeft),
            _    => None,
        }
    }
}

// ── Cached HUD strings ────────────────────────────────────────────────────────

#[derive(Default, Clone)]
pub struct HudCache {
    pub mode:  FpsMode,
    pub line1: String, // "FPS: 144" or "FPS: 144 | Min: 45 | Max: 180 | 1%: 70"
}

// ── Rolling Max FPS tracker (5-second window matching Horizon) ───────────────

struct FrameSample {
    time_secs: f64,
    fps: u32,
}

// ── Thread-local state ────────────────────────────────────────────────────────

thread_local! {
    static FRAME_COUNT: Cell<u64> = const { Cell::new(0) };
    static TOTAL_TIME:    Cell<f64> = const { Cell::new(0.0) };
    static LAST_HUD_TIME: Cell<f64> = const { Cell::new(0.0) };
    static SAMPLES:       RefCell<VecDeque<FrameSample>> = RefCell::new(VecDeque::new());
    static HUD_CACHE:   RefCell<HudCache> = RefCell::new(HudCache {
        mode: FpsMode::Simple,
        line1: String::new(),
    });
}

// ── Frame counter & timing ────────────────────────────────────────────────────

pub fn bump_frame(elapsed: Duration) -> u64 {
    let dt = elapsed.as_secs_f64();
    let current_time = TOTAL_TIME.with(|t| {
        let n = t.get() + dt;
        t.set(n);
        n
    });

    let inst_fps = if dt > 0.0001 {
        (1.0 / dt).round().min(9999.0) as u32
    } else {
        0
    };

    if inst_fps > 0 {
        SAMPLES.with(|q| {
            let mut q = q.borrow_mut();
            q.push_back(FrameSample {
                time_secs: current_time,
                fps: inst_fps,
            });
            // Keep 5 seconds window
            while q.front().is_some_and(|s| current_time - s.time_secs > 5.0) {
                q.pop_front();
            }
        });
    }

    FRAME_COUNT.with(|c| {
        let n = c.get().wrapping_add(1);
        c.set(n);
        n
    })
}

pub fn current_frame() -> u64 {
    FRAME_COUNT.with(|c| c.get())
}

pub fn current_max_fps() -> u32 {
    SAMPLES.with(|q| {
        let q = q.borrow();
        q.iter().map(|s| s.fps).max().unwrap_or(0)
    })
}

/// Returns true when the HUD text should be rebuilt based on target update rate (Hz / times per sec).
/// If hz is 0, returns true every frame (Real-time).
pub fn should_rebuild_hud(hz: u32) -> bool {
    if hz == 0 {
        return true;
    }
    let interval = 1.0 / (hz as f64);
    let now = TOTAL_TIME.with(|t| t.get());
    LAST_HUD_TIME.with(|last| {
        if now - last.get() >= interval {
            last.set(now);
            true
        } else {
            false
        }
    })
}

// ── HUD cache helpers ─────────────────────────────────────────────────────────

pub fn rebuild_hud(live: u32, min: u32, max: u32, low1: u32, mode: FpsMode) {
    HUD_CACHE.with(|h| {
        let mut c = h.borrow_mut();
        c.mode = mode;
        match mode {
            FpsMode::Off => {
                c.line1.clear();
            }
            FpsMode::Simple => {
                c.line1 = format!("FPS: {live}");
            }
            FpsMode::Extended => {
                let display_max = max.max(live);
                let display_min = if min > 0 { min } else { low1.min(live) };
                c.line1 = format!("FPS: {live} | Min: {display_min} | Max: {display_max} | 1%: {low1}");
            }
        }
    });
}

pub fn hud_snapshot() -> HudCache {
    HUD_CACHE.with(|h| h.borrow().clone())
}

// ── Real HUD Batch Throttling (Limits F3, Hotbar, Menus & UI FPS) ──────────

#[cfg(not(feature = "standalone"))]
thread_local! {
    static LAST_HUD_UPDATE: Cell<Option<std::time::Instant>> = const { Cell::new(None) };
    static CACHED_HUD_BATCH: RefCell<Option<horizon_client::hud::HudBatch>> = const { RefCell::new(None) };
    static CACHED_HUD_SIZE: Cell<[u32; 2]> = const { Cell::new([0, 0]) };
}

#[cfg(not(feature = "standalone"))]
pub fn save_hud_batch(batch: &horizon_client::hud::HudBatch) {
    LAST_HUD_UPDATE.with(|c| c.set(Some(std::time::Instant::now())));
    CACHED_HUD_SIZE.with(|c| c.set([batch.width as u32, batch.height as u32]));
    CACHED_HUD_BATCH.with(|b| {
        *b.borrow_mut() = Some(horizon_client::hud::HudBatch {
            vertices: batch.vertices.clone(),
            scale: batch.scale,
            width: batch.width,
            height: batch.height,
            patches: batch.patches.clone(),
            blends: batch.blends.clone(),
            end_portal: batch.end_portal,
            glint_alpha: batch.glint_alpha,
            blur_at: batch.blur_at,
            inline_profiles: batch.inline_profiles.clone(),
        });
    });
}

#[cfg(not(feature = "standalone"))]
pub fn try_reuse_hud_batch(rate_hz: u32, size: [u32; 2]) -> Option<horizon_client::hud::HudBatch> {
    if rate_hz == 0 {
        return None;
    }
    let interval = Duration::from_secs_f64(1.0 / (rate_hz as f64));
    let last = LAST_HUD_UPDATE.with(|c| c.get())?;
    if last.elapsed() < interval {
        let cached_size = CACHED_HUD_SIZE.with(|c| c.get());
        if cached_size == size {
            return CACHED_HUD_BATCH.with(|b| {
                b.borrow().as_ref().map(|cached| horizon_client::hud::HudBatch {
                    vertices: cached.vertices.clone(),
                    scale: cached.scale,
                    width: cached.width,
                    height: cached.height,
                    patches: cached.patches.clone(),
                    blends: cached.blends.clone(),
                    end_portal: cached.end_portal,
                    glint_alpha: cached.glint_alpha,
                    blur_at: cached.blur_at,
                    inline_profiles: cached.inline_profiles.clone(),
                })
            });
        }
    }
    None
}

#[cfg(feature = "standalone")]
pub fn save_hud_batch<T>(_batch: &T) {}
#[cfg(feature = "standalone")]
pub fn try_reuse_hud_batch<T>(_rate_hz: u32, _size: [u32; 2]) -> Option<T> { None }

// ── Command parser ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Eq)]
pub enum Cmd {
    Status,
    SetFpsMode(FpsMode),
    CycleFpsMode,
    SetRate(u64),
    SetCorner(String),
    Reset,
    Help,
    Unknown(String),
}

pub fn parse_cmd(raw: &str) -> Cmd {
    let args: Vec<&str> = raw.split_whitespace().collect();
    match args.as_slice() {
        [] => Cmd::Status,
        ["help"] => Cmd::Help,
        ["reset"] => Cmd::Reset,
        ["fps"] | ["toggle"] => Cmd::CycleFpsMode,
        ["fps", mode] => match FpsMode::from_str_opt(mode) {
            Some(m) => Cmd::SetFpsMode(m),
            None => Cmd::Unknown(format!("Unknown FPS mode: '{mode}'. Use simple, extended, or off.")),
        },
        ["rate", num] => match num.parse::<u64>() {
            Ok(n) if n >= 1 && n <= 120 => Cmd::SetRate(n),
            _ => Cmd::Unknown("Rate must be between 1 and 120 frames.".into()),
        },
        ["corner", pos] => match *pos {
            "tr" | "tl" | "br" | "bl" => Cmd::SetCorner((*pos).to_string()),
            _ => Cmd::Unknown("Corner must be tr, tl, br, or bl.".into()),
        },
        _ => Cmd::Unknown(format!("Unknown command. Type '/rustium help' for options.")),
    }
}

// ── Unit Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fps_mode_cycle() {
        assert_eq!(FpsMode::Simple.next(), FpsMode::Extended);
        assert_eq!(FpsMode::Extended.next(), FpsMode::Off);
        assert_eq!(FpsMode::Off.next(), FpsMode::Simple);
    }

    #[test]
    fn fps_mode_parse() {
        assert_eq!(FpsMode::from_str_opt("simple"),   Some(FpsMode::Simple));
        assert_eq!(FpsMode::from_str_opt("LIVE"),     Some(FpsMode::Simple));
        assert_eq!(FpsMode::from_str_opt("extended"), Some(FpsMode::Extended));
        assert_eq!(FpsMode::from_str_opt("off"),      Some(FpsMode::Off));
        assert_eq!(FpsMode::from_str_opt("invalid"),  None);
    }

    #[test]
    fn parse_status_and_commands() {
        assert_eq!(parse_cmd(""),              Cmd::Status);
        assert_eq!(parse_cmd("fps"),           Cmd::CycleFpsMode);
        assert_eq!(parse_cmd("fps simple"),    Cmd::SetFpsMode(FpsMode::Simple));
        assert_eq!(parse_cmd("fps extended"),  Cmd::SetFpsMode(FpsMode::Extended));
        assert_eq!(parse_cmd("fps off"),       Cmd::SetFpsMode(FpsMode::Off));
        assert_eq!(parse_cmd("rate 1"),        Cmd::SetRate(1));
        assert_eq!(parse_cmd("rate 10"),       Cmd::SetRate(10));
        assert_eq!(parse_cmd("corner tl"),     Cmd::SetCorner("tl".into()));
        assert_eq!(parse_cmd("reset"),         Cmd::Reset);
        assert_eq!(parse_cmd("help"),          Cmd::Help);
    }

    #[test]
    fn hud_formatting() {
        rebuild_hud(120, 45, 160, 60, FpsMode::Simple);
        let s = hud_snapshot();
        assert_eq!(s.line1, "FPS: 120");

        rebuild_hud(120, 45, 160, 60, FpsMode::Extended);
        let s2 = hud_snapshot();
        assert_eq!(s2.line1, "FPS: 120 | Min: 45 | Max: 160 | 1%: 60");

        rebuild_hud(120, 45, 160, 60, FpsMode::Off);
        let s3 = hud_snapshot();
        assert_eq!(s3.line1, "");
    }
}
