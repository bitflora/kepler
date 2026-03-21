# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A project that plays the Music of Spheres described by Kepler's Harmony of the World.
- **Web visualization** (`index.html`): Interactive animated solar system with per-planet music

Images to be used in the html can be found in the images subdirectory.

## Architecture

### Web Visualization (`index.html`)

- Self-contained single file (HTML/CSS/JS, no dependencies)
- Planets orbit the sun with CSS/JS animation; speed is controlled via a slider
