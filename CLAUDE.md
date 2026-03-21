# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A project that plays the Music of Spheres described by Kepler's Harmony of the World.
- **Web visualization** (`index.html`): Interactive animated solar system with per-planet music

Images to be used in the html can be found in the images subdirectory.

## Kepler's Music
https://chatgpt.com/s/t_69bb7baf75388191beff8a893f618e8b

Kepler is not describing one fixed tune. He says the planets move through a continuum of pitches, so the notes printed on the staff are schematic; the exact things to extract are each planet’s extreme notes, while the intermediate notes just show the glide between them.

So the clean modern-note summary is:
- Saturn — G to B natural; as a simple discrete approximation: G-A-B-A-G.
- Jupiter — G to B-flat; approximately G-A-B♭-A-G.
- Mars — in Kepler’s common-system/modal reading, F to C; approximately F-G-A-B♭-C-B♭-A-G-F.
- Earth — E-F-E. Kepler states it explicitly as “MI, FA, MI.”
- Venus — essentially a single E; if forced into repeated discrete notes, E-E-E. Kepler says Venus stays approximately in unison and does not even span the smallest consonant interval.
- Mercury — starts on A and spans the widest range. Kepler gives Mercury A as its starting pitch, and scholarly summaries of his figure read Mercury as covering a minor tenth; in note-names that is A up to C, passing through the intervening notes A-B-C-D-E-F-G-A-B-C, then back down.

So, in shortest form: Saturn G-B, Jupiter G-B♭, Mars F-C, Earth E-F-E, Venus E, Mercury A-C.

## Architecture

### Web Visualization (`index.html`)

- Self-contained single file (HTML/CSS/JS, no dependencies)
- Planets orbit the sun with CSS/JS animation; speed is controlled via a slider
