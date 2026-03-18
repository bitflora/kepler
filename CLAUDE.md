# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A project that calculates when it's "Taco Tuesday" on different planets:
- **Web visualization** (`index.html`): Interactive animated solar system with per-planet Taco Tuesday status

Ignore everything but the html.

## Architecture

### Web Visualization (`index.html`)

- Self-contained single file (HTML/CSS/JS, no dependencies)
- Planets orbit the sun with CSS/JS animation; speed is controlled via a slider
- Hovering a planet shows a tooltip with current day and time until next Taco Tuesday
- Planets display a 🌮 and golden glow when it's Tuesday
- A date input allows simulating past/future dates
- Taco Tuesday logic mirrors the Rust CLI: modular arithmetic on timestamps relative to J2000.0
