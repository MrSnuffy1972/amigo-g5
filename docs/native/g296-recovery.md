# Game 296 (Global Thermonuclear Warfare): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen. The source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (one OFS disk image, 901120 bytes) | 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | identical: all 12 program hunks (bytes and relocations) and the boot block |
| Functions named and described | 116 of 116 named (author symbols), 116 of 116 described |
| Executed addresses (reference run) in recovered, described code | 675 of 675 (100.00%) |
| Independent decoder cross-check (Ghidra against capstone) | 2659 of 2660 instructions identical in start and length, 0 mismatches |

## Bytes by class (whole disk)

| class | bytes | percent |
|---|---|---|
| code | 11716 | 1.300 |
| data | 145653 | 16.164 |
| asset | 506740 | 56.234 |
| padding | 1491 | 0.165 |
| unused | 235520 | 26.136 |

Files on the disk: `GTW` (the program, 12 hunks: 4 code, 8 data), three world maps (`GTW-Map1..3.gfx`, assets) and `s/startup-sequence`. The boot block holds the standard AmigaDOS 2.0 install boot code (one routine, recovered). There is no packing, no copy protection and no track loader: the game loads through AmigaDOS. The program carries its full symbol table, so every routine keeps the author's name.

Limit: the reference run covers only the intro and title (about 400 frames). The other routines are proved reachable statically, from the entry point, the button-handler tables and the interrupt vector.

## Functions


### Boot block

- `bootentry`: Standard AmigaDOS 2.0 install boot code, called by the Kickstart strap with a6 = ExecBase.

### Program hunk 0 (main, game, ProTracker replayer)

- `program`: Program entry point (hunk 0 offset 0).
- `initialise`: One-time start-up: sets the CIA-A LED bit (power LED dim, audio filter off), clears the CIA-B drive-0 select and motor bits, builds the display (initbitmap), enables the pointer sprite, sets option defaults (oncevars), opens dos.library, installs the vertical-blank handler and loads the first map file.
- `exit`: Shutdown, the reverse of initialise: closes dos.library, turns sprites off, restores the system copper list and frees the bitmap (exitbitmap), removes the interrupt vector and turns the filter LED back on.
- `main`: Top-level loop: shows the intro fade once, then forever runs the title screen, the options screen and one game.
- `mt_init`: ProTracker replayer initialisation: stores the module address, scans the song position table for the highest pattern to find where the sample data starts, fills mt_SampleStarts with the 31 sample addresses, and resets speed (6), counter, song position and pattern position.
- `mt_end`: Silences the four Paula channels (volume 0) and turns their DMA off; used to stop the music when leaving the options screen and as the tail of mt_init.
- `mt_music`: ProTracker per-frame player, called from the vertical-blank handler while musicflag is set.
- `mt_NoNewAllChannels`: Runs mt_CheckEfx for each of the four channels on ticks between rows; its second entry mt_GetNewNote fetches the current pattern row and plays it on each channel through mt_PlayVoice.
- `mt_PlayVoice`: Plays one channel's note for the current row: decodes sample number, period and effect, sets sample start/length/loop and volume, handles finetune, tone-portamento set-up and the retrigger DMA wait, then advances the pattern position and song position (pattern break, position jump, end of song).
- `mt_CheckEfx`: Per-tick effect dispatcher for one channel: updates funk repeat, then branches on the effect command to arpeggio, portamento, tone portamento, vibrato, volume slide or the extended E commands; with no effect it rewrites the channel period.
- `mt_PerNop`: Writes the channel's current period to Paula; the entry is shared with the arpeggio (effect 0) and fine/normal portamento up and down (effects 1, 2, E1, E2) handlers that follow it.
- `mt_SetTonePorta`: Tone portamento set-up (effect 3 on a new note): finds the target period in the period table for the channel's finetune and records the slide direction; also holds mt_ClearTonePorta and the per-tick mt_TonePortamento speed update.
- `mt_TonePortNoChange`: Performs one tick of tone portamento toward the target period (with optional glissando rounding to semitones), and starts the vibrato (effect 4) handler that follows.
- `mt_Vibrato2`: Applies one vibrato step using the sine, ramp or square waveform, and contains the combined tone-portamento/vibrato plus volume-slide handlers (effects 5, 6) and tremolo (effect 7) on the channel volume.
- `mt_CheckMoreEfx`: Effects evaluated once per row: sample offset (9), position jump (B), set volume (C), pattern break (D), set speed (F) and the dispatcher for the extended E commands (filter, glissando and vibrato control).
- `mt_SetFineTune`: Extended ProTracker E commands: set finetune, pattern loop, tremolo control, retrigger note, fine volume up and down, note cut, note delay, pattern delay and invert loop (funk).
- `mt_UpdateFunk`: Invert-loop (EF funk) update: at the speed from mt_FunkTable it steps through the looped part of the sample, inverting one byte at a time.
- `game`: One complete war: initstuff, then alternates the American turn (usaloop) and the Soviet turn (ussrloop), each followed by the missile flight (icbmloop), the win test (usawon/ussrwon) and factory resupply.
- `clearsc`: HUD 'clear' button handler: redraws the clean map into the display bitmap and redraws all markers, which removes the missile trails.
- `warover`: War-over screen: clears the map area, draws the frame graphic, prints 'the war is over', the winner line, the two-line reason and both sides' scores (5 digits via convert), then waits for a click.
- `peace`: HUD PEACE button handler: ends the war at once as a draw by setting exitflag and winflag, with winside 'both sides declare peace' and an empty reason.
- `usawon`: After an American shot, counts the Soviet silos and submarines still alive and their missiles.
- `ussrwon`: After a Soviet shot, the mirror of usawon: tests the American silos and submarines and missiles and declares a Soviet win with reason3 or reason2 plus 500 points.
- `initstuff`: Per-game set-up: sets the interrupt enables, resets the variables, clears both object tables, copies the clean map to the screen, places all objects (setup), picks each side's secret HQ city, draws the markers and fills the HUD.
- `checkhit`: Explosion hit test over all 140 objects: for each live object within distance 6 (squroot of the squared distance) it plays the hit sound, scores 400 - 50*distance for the victim's opponent, declares a win with reason1 if it was that side's HQ, draws the destroyed marker and marks the record dead.
- `factories`: Resupply after a shot for the side whose table is in a2: every live factory counts down, and on reaching zero is reset to shipspeed and gives each of its two silos one missile, up to 3.
- `explode`: Missile landing explosion: computes the blit address and shift of the impact point, saves the map under the cloud into mushbg with the blitter, then runs the mushroom animation.
- `mushroom`: Plays the 12-frame mushroom cloud: every 5 vertical blanks it cookie-cut blits the next 32x9 frame with its mask over the saved background, until the last frame.
- `icbmloop`: Missile flight loop, one step per frame: scrolls if needed, draws the trail segment (printicbm), moves the missile (moveicbm1/moveicbm2) and applies gravity, until exitflag is set by landing or leaving the map.
- `moveicbm1`: Vertical part of a flight step: adds sin(elevation) x1024 to the altitude and sets the ground speed to cos(elevation) x1024 / 2 (via the sine and cosine tables).
- `moveicbm2`: Horizontal part of a flight step: moves the missile x by sin(heading) * speed >> 7 and y by -cos(heading) * speed >> 7 in 10-bit fixed point.
- `gravity`: Subtracts the fall speed from the altitude and increases the fall speed by gravrate.
- `printicbm`: Draws the newest trail segment: converts the fixed-point position and altitude to screen coordinates, wraps and clips them (clip1/clip2) and, if the point is on the map, draws a line in colour 31 on all 5 planes with blitline; then stores the current position as the previous one.
- `clip1`: Wraps the missile x around the 576-pixel world and ends the flight (exitflag) if y leaves the map at the top or bottom, redrawing the map when trails are off.
- `clip2`: Empty clipping stage (just returns); kept by the author as a placeholder.
- `showturn`: Draws the side flag (USA or USSR frame of sidegfx) into the HUD to show whose turn it is.
- `usaloop`: The American turn: if the last-used launcher is dead it selects the first live silo or sub, fills the HUD, then once per frame scrolls, blinks the selected marker and handles the HUD buttons and map clicks until a missile is fired or peace is declared (or ESC is pressed).
- `ussrloop`: The Soviet turn, identical to usaloop for the ussrpics table and lastussr, without the ESC test.
- `fired`: HUD LAUNCH handler: with no missiles it plays the empty click; otherwise it plays the launch sound, takes one missile, stores elevation and heading in the record, starts the missile at the launcher position with zero altitude and fall, updates the HUD and sets exitflag to end the turn.
- `siloclix`: Map click handler: on a new left-button press below the HUD it looks for one of the current side's objects within 3 pixels of the pointer, plays the select sound and makes it the current launcher (showinfo).
- `flashsilo`: Blinks the selected launcher's marker: counts the 20-frame flash cycle and re-blits the marker frame for the current phase (markframes_a/markframes_b) if the object is alive.
- `showinfo`: Selects a new current object (a1): stores the old launcher's elevation and heading back in its record, loads the new one's missiles, elevation and heading, prints its type caption and name, its photo, missile count and the aim values.
- `prtammo`: Draws the missile-count picture (0 to 3 missiles, ammogfx frame) for the current launcher into the HUD.
- `vbgint`: Level-3 vertical-blank interrupt handler: acknowledges the interrupt, rewrites the map bitplane pointers and fine scroll, moves the pointer sprite, runs the background flash and mouse reading, then plays the music (mt_music) or silences finished sound effects (soundoff).
- `inchead`: Heading + button handler: increases the heading by 1 degree, wrapping 359 to 0, and reprints the aim values.
- `dechead`: Heading - button handler: decreases the heading by 1 degree, wrapping 0 to 359, and reprints the aim values.
- `incelev`: Elevation + button handler: increases the elevation by 1 up to 90 degrees and reprints the aim values.
- `decelev`: Elevation - button handler: decreases the elevation by 1 down to 0 and reprints the aim values.
- `sethead`: Heading preset buttons: picks 0, 90, 180 or 270 degrees from headpresets according to which 10-pixel cell the pointer is in, and reprints the aim values.
- `setelev`: Elevation preset buttons: picks 0, 30, 60 or 90 degrees from elevpresets according to the pointer cell, and reprints the aim values.
- `prtelev`: Prints the current elevation and heading as decimal numbers into the HUD with the 8x13 font.
- `scroll`: Edge scrolling: while the pointer is held at the right or left screen edge it moves the map 4 pixels per frame (fine scroll in scwait, coarse 16-pixel steps in mapaddr) within the 256-pixel range, and shifts every marker's screen x via onscreen.
- `onscreen`: Adds the scroll delta in d0 to the screen x of every live object in both tables.
- `pointwait`: Writes the fine-scroll value into the copper list's BPLCON1 move for both playfields.
- `initvars`: Resets the per-game variables: scores, scroll, flags, aim values, flash and animation counters, the last/current launcher pointers, the map address and both HQ pointers.

### Program hunk 1 (graphics, sound, text and input helpers)

- `fadetowhite`: Fades the 32-colour palette up to white, one step per colour component every frame pair.
- `fadetocols`: Fades the palette at a1 toward the target palette at a0 one step per component per frame pair until all 32 entries match, copying to the hardware each step.
- `playsfx`: Starts sound effect d0 from sfxlist on the next channel in round-robin order (3, 2, 1, 0): sets sample pointer, length, period and volume after a short wait and enables that channel's DMA.
- `soundoff`: Called each frame when no music plays: for every channel whose audio interrupt has fired it points the channel at the silent nosound sample, so one-shot effects do not loop.
- `loadmap`: Options LOAD handler and start-up loader: reads the selected map file (mapfnmpt) into mapgfx via loadfile.
- `loadfile`: Loads a whole file with dos.library: Open, Seek to the end and back to get its length, Read into the buffer and Close, then clears the CIA-B drive select and motor bits again.
- `write12`: Prints a NUL-terminated string with the 8x13 font into plane 0 of the display bitmap (360-byte rows), folding lower case to upper and mapping '.' and ',' to their glyphs; with d7 set it prints one character per frame.
- `squroot`: Integer square root of d0 by testing from the top bit down; used for the explosion distance.
- `clix2`: Left-button button-table handler: on a new press it searches the {x1,y1,x2,y2,handler} table in a0 for the box under the pointer and calls its handler.
- `flash`: Background flash after an explosion: every 2 frames lowers COLOR00 by flashdec until it reaches black.
- `blitline`: Draws a line from (d0,d1) to (d2,d3) in one bitplane at a2 with the blitter's line mode, choosing the octant from lineoctants and setting the Bresenham terms.
- `writemini`: Prints a string with the 4x5 mini font, two characters per byte (nibble shift), at the row stride in d7; used for HUD captions and instructions text.
- `printslab`: Plain blitter copy of a rectangle: d0 words wide, d1 rows times d2 planes high, from a0 to a1 with destination modulo from the row stride d3.
- `blitaddr`: Converts a pixel x in d0 to a byte offset (x/16*2) and a blitter shift value ((x and 15) << 12).
- `printblock`: Cookie-cut blit of one 16x6 marker frame (mask then data, 120 bytes apart) at a0 onto the world bitmap at a1 with shift d1, for all 5 planes.
- `point`: Reads the 5-plane colour index of world pixel (d0,d1) from the display bitmap; used to place objects on land of the right colour or on sea.
- `random`: 16-bit linear congruential random number generator, returning a value modulo the requested range.
- `write`: Prints a string with the 8x13 font into the HUD panel (200-byte rows); used for the elevation and heading numbers.
- `convert`: Converts the number in d0 to d1 decimal ASCII digits at a0 by repeated subtraction of powers of ten.
- `clix`: Right-button button-table handler with auto-repeat: while the button is held it calls the handler of the box under the pointer every 2 frames.
- `spron`: Waits for a raster line below the display and enables sprite DMA so the pointer shows.
- `sproff`: Waits for a raster line below the display and disables sprite DMA.
- `mousemove`: Reads the JOY0DAT mouse counters, adds the signed deltas since last frame to pointx and pointy (labels negx/negy/onx/ony handle sign and limits) and clamps the pointer to the screen.
- `printpoint`: Writes pointx and pointy into the pointer sprite's control words (start/stop position with the high bits) so it follows the mouse.
- `opendos`: Opens dos.library (exec OpenLibrary) and stores the base in dosbase.
- `closedos`: Closes dos.library (exec CloseLibrary).
- `delay`: Waits a given number of frames by watching the raster beam.
- `copycols`: Copies 32 colours from a1 to the hardware colour registers.

### Program hunk 3 (objects, display set-up)

- `clearsilo`: Zeroes both object tables (usapics and ussrpics, 2 x 70 records).
- `prtsilo`: Draws the marker of every live object in both tables onto the world bitmap.
- `esthq`: Chooses each side's secret headquarters: a random city record index for the USA and for the USSR, added to the first-city pointers set by setup.
- `setup`: Places all objects for both sides: for each of the 8 types (silos, subs, factories, cities per side) it takes the count and name list from setuptypes, retries random positions until the map pixel has the type's colour, then fills the record (position, blit address and shift, missiles, factory countdown and supplied silos, name).
- `oncevars`: Sets the option defaults once at start-up (10 silos, 5 factories, 3 subs, 14 cities, factory rate 2, map 1, gravity 17), clears music and pointer state, and seeds the random generator from the CIA-B time-of-day counter.
- `startint`: Saves the level-3 autovector and points it at vbgint, then enables the vertical-blank interrupt.
- `stopint`: Restores the saved level-3 autovector.
- `initbitmap`: Builds the display: saves the system copper list, allocates the chip-memory bitmap, fills and starts the copper list, loads the palette and sets the display registers for 5 planes.
- `exitbitmap`: Restores the system display: re-enables sprites, points the copper back at graphics.library's list, frees the bitmap, closes graphics.library and calls Permit.
- `pointmap`: Writes the current map address (mapaddr, 5 planes 72 bytes apart) into the map part of the copper list.
- `printmap`: Copies the clean map from mapgfx into the display bitmap (74160 bytes), erasing markers and trails.
- `clearmap`: Clears the display bitmap's map area to colour 0 and resets the scroll position.

### Program hunk 8 (title, options and instructions screens)

- `intro`: Intro: fades the palette to white, clears the map and fades back to the game palette.
- `titlescreen`: Title screen: starts the title music, clears the screen and reveals the 240x82 logo one row per frame (82 frames), then waits for a mouse click.
- `optsscrn`: Options screen: draws the frame, the title and the current values, then each frame handles left clicks on the option buttons until PLAY sets exitflag; then stops the music.
- `instructs`: Instructions: draws the frame and title, then shows 6 pages of 16 lines from instext with the mini font, waiting for a click after each page, and returns to the options screen.
- `waitclik`: Waits for a new press of the left mouse button (edge-triggered with waitclik_latch).
- `addsub`: Options: one more submarine per side, up to 10.
- `subsub`: Options: one fewer submarine per side, down to 1.
- `addmap`: Options: next map (up to map three) and advances the file name pointer; LOAD must be pressed to load it.
- `submap`: Options: previous map (down to map one) and moves the file name pointer back.
- `addsilo`: Options: two more silos (up to 20) and one more factory per side.
- `subsilo`: Options: two fewer silos (down to 2) and one fewer factory per side.
- `addcity`: Options: one more city per side, up to 30.
- `subcity`: Options: one fewer city per side, down to 1.
- `addship`: Options: slower factory resupply, one more turn between deliveries, up to 8.
- `subship`: Options: faster factory resupply, down to 1 turn.
- `trails`: Options: toggles missile trails on and off (trailflag) and reprints the values.
- `defaults`: Options DEFAULT button: restores 3 subs, 10 silos, 5 factories, rate 2 and 14 cities and reprints the values.
- `prtvals`: Prints all option values on the options screen: the four numbers (prtcon), trails ON/OFF and the map name.
- `prtcon`: Prints one 2-digit option number at a screen position (convert then write12).
- `exitopt`: Options PLAY button: sets exitflag so the options loop ends and the game starts.

## Copy protection found

- none found: no routine touches disk hardware or trackdisk, no protection text is referenced, and every file loads through AmigaDOS (checked with the generic evidence rules, 1 Nov 2026).
