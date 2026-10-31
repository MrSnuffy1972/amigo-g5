# Game 296 native: Global Thermonuclear Warfare

Added by native stage 1 (31 Oct 2026). Global Thermonuclear Warfare (Mark Sheeky, Scorpius Software, 1994) is a
two-player hot-seat artillery game: America and Russia take turns to pick a silo or submarine, set elevation and
heading and fire one missile across a scrolling world map at the other side's silos, submarines, factories and
cities. Hitting the hidden headquarters city, or leaving the enemy with no launchers or no missiles, wins; both
sides can declare peace. Options set the map (three), the number of each kind of object, the factory rate and
missile trails; there are six pages of instructions and a ProTracker title tune.

The native version is `rust/games/g296` on `rust/native_engine` (see [G5-NATIVE.md](../G5-NATIVE.md), "Added
by native stage 1"). It is written from the recovered source as ordinary typed Rust: no guest memory, no 68000
registers, no Kickstart, no chip registers or copper, blitter or bitplane emulation. All assets come from the
asset pack at run time (extracted outside the repo; nothing of the game is in git).

## What was ported

Every screen and code path: the intro fade, the title reveal and its music, the options screen (all buttons,
including defaults and the map choice), the instructions pages, a new war's set-up (objects placed on land or sea
of the right colour, hidden headquarters), both sides' turns (launcher selection by click, elevation and heading
buttons with right-button repeat, map scrolling at the screen edges, the HUD photo, name, ammunition and turn flag),
the missile flight (climb, heading, gravity, wrap round the world, trails and the clear button), hits and the
mushroom cloud, factory resupply, every win reason and peace, the war-over screen and the return to the title.
The sound effects and the music play as the original did, including its quirks (every effect plays once, because
the original's repeat test can never be true). 50 Hz PAL timing is kept tick for tick, including the frames the
original lost to slow drawing and to music rows.

Protection removed: none (the game has none). Cracktro: none.

## Function map summary

116 recovered routines, 116/116 accounted for: 96 ported, 20 deliberately not ported. The full map (routine,
range, native function, reason) is kept outside the repo with the asset pack.

Ported: program initialise main mt_init mt_end mt_music mt_NoNewAllChannels mt_PlayVoice mt_CheckEfx mt_PerNop mt_SetTonePorta mt_TonePortNoChange mt_Vibrato2 mt_CheckMoreEfx mt_SetFineTune mt_UpdateFunk game clearsc warover peace usawon ussrwon initstuff checkhit factories explode mushroom icbmloop moveicbm1 moveicbm2 gravity printicbm clip1 clip2 showturn usaloop ussrloop fired siloclix flashsilo showinfo prtammo vbgint inchead dechead incelev decelev sethead setelev prtelev scroll onscreen initvars fadetowhite fadetocols playsfx soundoff loadmap write12 squroot clix2 flash writemini point random write convert clix mousemove clearsilo prtsilo esthq setup oncevars printmap clearmap intro titlescreen optsscrn instructs waitclik addsub subsub addmap submap addsilo subsilo addcity subcity addship subship trails defaults prtvals prtcon exitopt.

Not ported, because the engine does their job (OS start-up and shutdown, the boot block, interrupt set-up, beam
waits, copper, sprite and blitter drivers, DOS file reads): bootentry exit pointwait loadfile blitline printslab blitaddr printblock spron sproff printpoint opendos closedos delay copycols startint stopint initbitmap exitbitmap pointmap.

## Verification

Reference: the 68000 reference harness running the original (1 MB chip as the game asks), driven by the same
scripted mouse input as the native headless runner, 1832 ticks covering intro, title, options, instructions, a
war with three hits and a headquarters win, the war-over screen and the return to the title. The frozen G5
translated build was used as a second reference for its first 237 ticks (where it stops on untranslated code).

| Check | Result | Tolerance |
|---|---|---|
| Named state (18 values: phase, pointer, scroll, elevation, heading, missiles, scores, flags, missile x/y/altitude, flash, seed ...) | 17 exact on every tick; the RNG seed differs for 12 ticks inside set-up only, same values after | exact, except set-up internals |
| Events (music start, scores, win) | identical ticks: music 71, 279, 1624; scores at 740, 1085, 1250; HQ win 1408 | exact |
| HUD picture (rows 0 to 49) | 1824/1832 frames identical, 1830 within one frame | within 1 frame |
| World map (from the reference's memory, trails excluded) | 91/168 sampled ticks identical; the rest are the original's progressive clears at 200, 280, 1410 and 30 to 77 pixels near the first missile's path (world x 36 to 86, y 36 to 74) | see note |
| Sound | envelope correlation 0.994, effect onsets on the same ticks | correlation above 0.99 |
| Frozen G5 build (ticks 0 to 236) | pointer, music and cities exact; HUD 235/237 identical | as above |

Note: the reference harness's blitter line mode draws trails wrongly (it treats one octant bit as y-major), so
trail pixels are counted apart and the residual map pixels beside the first trail are most likely that defect
writing into the map planes; they have not been proven one way or the other.

Play tests: desktop (Linux, SDL2) and the NVIDIA Shield (`org.amigo.g5.g00296.nativegame`, installed beside the
translated build). Screenshots checked by eye: pre-boot menu, title, options, war, aiming, flight, Russia's turn,
war over and back to the title. Pressing start or tapping PLAY starts the game (a tap shorter than one tick is
latched); no freeze or quit after the start-up screen. The Shield draws 60.0 fps with the game at 50 ticks a second.
