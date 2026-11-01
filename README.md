<div align="center">

<img src="icon.png" width="160" height="160" alt="Rustium Logo" />

# Rustium

**The Sodium optimization engine rewritten in native Rust for Horizon Minecraft.**

[![Horizon](https://img.shields.io/badge/Horizon-Native_Mod-orange.svg)](https://github.com/horizon-rust/horizon)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/Version-0.1.0-green.svg)]()

</div>

---

## ⚡ Sodium-Inspired Optimizations

* **Built-in Entity Culling (Sodium Entity Culling in Rust)**:
  * Uses fast SIMD software occlusion culling to skip drawing entities hidden behind opaque blocks or outside the camera frustum.
  * Massive FPS boost around mob farms, villager halls, item frames, and dense entity clusters.
  * Toggleable on/off anytime in Options > Mods > Rustium or via `/rustium culling`.
* **UI & HUD Framerate Decoupling**:
  * Decouples the 2D interface (F3 debug screen, hotbar, hearts, containers) from the 3D terrain rendering pipeline.
  * Stops costly per-frame vertex allocations for static 2D UI elements.
  * Switch between Uncapped, 60 FPS, 30 FPS, or 15 FPS.
* **Toggleable FPS & Frame Time Diagnostics**:
  * **Simple Mode**: Minimal live FPS display (`FPS: 144`).
  * **Extended Mode**: Real-time telemetry showing live FPS, minimum, maximum, and 1% low frame times.
  * **Toggle Key**: Press **`F9`** in-game to cycle (`Simple` → `Extended` → `OFF`).
* **Zero GC & Memory Overhead**:
  * 100% native Rust, compiled directly into the client via compile-time AST mixins without garbage collection pauses.

---

## 🎮 In-Game Controls & Commands

| Action | Control |
|---|---|
| **Toggle FPS Display** | Press **`F9`** |
| **Settings Menu** | Go to `Options` → `Mods` → `Rustium` → `Settings` |
| **Toggle Entity Culling** | `/rustium culling` *(or `/rustium culling on\|off`)* |
| **Status Command** | `/rustium` |
| **Switch FPS Mode** | `/rustium fps <simple\|extended\|off>` |
| **Set UI Framerate** | `/rustium rate <fps>` *(e.g. 30, 60, or 0 for uncapped)* |
| **Set HUD Corner** | `/rustium corner <tr\|tl\|br\|bl>` |
| **Reset Defaults** | `/rustium reset` |

---

## 📦 Installation

1. Download `rustium-0.1.0.hnmod`.
2. Place it in your Horizon mods directory:
   * Windows: `%APPDATA%\horizon\horizon-mods\`
3. Open the Horizon Launcher and launch the game.
