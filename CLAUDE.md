# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a 2D platformer game built with **Godot 4.4** using **Rust** for game logic through the godot-rust (gdext) library. The project follows a standard Godot project structure with Rust code compiled as a dynamic library.

## Architecture

### Rust Components
- **Extension Library**: `rust/src/lib.rs` - Main entry point for Godot extension
- **Game Logic**: 
  - `player/` - Player character controller with movement (walk, jump, double jump) and shooting
  - `enemy/` - Basic enemy AI with walking behavior and death state
  - `level/` - Level management including coin collection system
- **Physics**: Uses Godot's built-in CharacterBody2D physics with custom gravity handling

### Godot Scene Structure
- **Scenes**: `godot/` directory contains all game scenes
  - `player/player.tscn` - Player character with animations and audio
  - `enemy/enemy.tscn` - Enemy character with sprite and animations
  - `level/level.tscn` - Main game level with platforms, coins, and boundaries
  - `game.tscn` - Root game scene

## Development Commands

### Build and Run
```bash
# Build Rust code and start Godot
./run.ps1 restart

# Start Godot without rebuilding
./run.ps1 start

# Stop running Godot instance
./run.ps1 stop
```

### Manual Build
```bash
# Build Rust extension
cd rust && cargo build

# Run Godot from godot/ directory
godot --path ./godot
```

### Key Configuration
- **Gravity**: 2100.0 (set in project.godot)
- **Physics**: 120 ticks per second
- **Window**: 800x480 (scaled to 1600x960)
- **Layers**: Player(1), Enemies(2), Coins(3), Platforms(4), Ground(5)

### Input Bindings
- **move_left**: Left arrow key
- **move_right**: Right arrow key  
- **jump**: Up arrow key
- **shoot**: Space bar

## Development Notes
- Rust code compiles to `target/debug/crate.dll` (Windows) which Godot loads dynamically
- All game entities use Godot's node system with Rust implementing the logic
- Animation states are controlled via AnimationPlayer nodes in scenes
- Audio effects are triggered through AudioStreamPlayer2D nodes