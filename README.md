<div align="center">

<img src="icon.png" width="160" height="160" alt="Rustium Logo" />

# Rustium

**Native high-performance optimization mod for Minecraft / Horizon Rust.**

[![Horizon](https://img.shields.io/badge/Horizon-Native_Mod-orange.svg)](https://github.com/horizon-rust/horizon)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/Version-0.1.0-green.svg)]()

</div>

---

## ⚡ Features

* **Toggleable FPS Counter**:
  * **Simple Mode**: Clean, minimal live FPS display (`FPS: 144`).
  * **Extended Mode**: Real-time telemetry showing live FPS, minimum (lowest in 5s), maximum, and 1% low framerate (`FPS: 144 | Min: 45 | Max: 180 | 1%: 70`).
  * **Toggle Key**: Press **`F9`** in-game to cycle (`Simple` → `Extended` → `OFF`).
* **UI & HUD Framerate Throttling**:
  * Decouples the 2D interface (F3 debug screen, player hotbar, hearts, hunger, inventory & container menus) from the 3D rendering pipeline.
  * Allows the 3D world to render at maximum uncapped framerates while saving GPU/CPU resources on static UI redraws.
  * Presets: **Uncapped**, **60 FPS**, **30 FPS**, and **15 FPS**.
* **Zero Overhead**: Written in 100% native Rust using compile-time AST mixins without runtime reflection or GC pauses.

---

## 🎮 In-Game Controls & Commands

| Action | Control |
|---|---|
| **Toggle FPS Display** | Press **`F9`** |
| **Settings Menu** | Go to `Options` → `Mods` → `Rustium` → `Settings` |
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
