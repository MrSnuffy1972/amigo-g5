# Game 296 Global Thermonuclear Warfare: R1 summary (no game bytes)

Full report: Ryzen `/srv/fast/amigo/native/00296/README.md`. State map: `state-map.json` in the same folder. Assets: `/srv/bulk/amigo-native/00296/manifest.json`, in the proposed format `native-asset-pack/0.1-proposed`.

## Structure

- **Players:** two-player hot-seat, USA against USSR. There is no AI.
- **Screen flow:** intro fade, then title (logo reveal plus title music, click to continue), then options (subs, silos, cities, factory rate, trails, map 1-3, Default, Instructions with 6 pages, Play), then game, then war over (winner, reason, both scores, click). After that it returns to the intro.
- **Timing:** everything is frame-locked at 50 Hz with no other clock.
- **Display:** a 320x50 HUD on top and a 320x206 view below it onto a 576x206 world map. The map scrolls when the pointer sits at the left or right edge (4 px per frame, 256 px maximum). The world wraps horizontally for missiles.

## Rules

- **Objects:** each side has silos (3 missiles each), subs (3 missiles, at sea, never resupplied), factories (one per two silos; every N turns each gives +1 missile to its two silos, capped at 3) and cities.
- **Placement:** objects are placed at random on own-side land, or on sea for subs. One city per side is a secret HQ.
- **A turn:** the player selects one of their launchers on the map, sets elevation (0-90; arrows or presets 0/30/60/90) and heading (0-359; arrows or presets N/E/S/W), then presses LAUNCH, PEACE or clear-trails.
- **Flight:** ballistic. Each frame, altitude gains sin(e) and gravity grows by a fixed step; ground speed is about 4·sin(h)·cos(e) px per frame. The maximum range is about 240 px at 45 degrees. The missile is drawn as a rising arc. A missile that leaves the top or bottom of the map is lost.
- **Impact:** a blocking mushroom animation (about 1.2 s) and a white background flash. Every object of either side within 6 px is destroyed.
- **Scoring:** the victim's opponent scores 400 - 50·distance.
- **Winning:** a side wins by hitting the enemy HQ, by destroying all enemy silos and subs, or when the enemy has no missiles left. Each win adds +500. PEACE ends the game as a draw. ESC quits.
- **Sound effects:** select, launch, explosion on landing, a higher-pitched explosion per object hit, and a dry click when firing empty. Music plays only on the title and options screens.

## Asset names (manifest)

- **Palette:** main (32 colours).
- **Images:** panel, photos (4: silo, sub, factory, city), ammo (4), side (USA/USSR flag), title, options, instructions.
- **Sprite sheets:** mushroom (12 frames, with mask), pieces (16 map markers: normal, selected and destroyed per type, with mask), pointer.
- **Fonts:** alpha12 (8x13), mini (4x5).
- **Maps:** map1-3 (576x206; colour 27 = US land, 26 = USSR land, 29 = sea).
- **Samples:** sfx0-4.
- **Music:** title (.mod).
- **Text:** instructions_text, names, ui_strings.

## State for E2

The state map covers the scores, the selected launcher and its elevation, heading and ammo, the missile position, altitude and fall, the flags (win, exit, trails, music), the scroll, the pointer, the options, and two tables of 70 objects with 42-byte records (type, x, y, missiles, factory timer, stored aim, name). All addresses are runtime addresses from the G5 ref run.
