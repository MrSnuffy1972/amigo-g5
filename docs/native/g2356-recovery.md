# Game 2356 (spy vs spy (1989)(first star software)[cr yakoul]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02356). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 45 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 114 raw-word lines |
| Functions named and described | pass, 650 of 650 named (524 author symbols), 650 of 650 described (44 by rule, 119 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6056 of 6056 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 23343 of 23375 instructions identical in start and length, 2 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 477296 |
| code | 99686 |
| data | 233146 |
| padding | 10096 |
| unused | 80896 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Spy vs Spy (1989)(First Star Software)[cr Yakoul].adf:ArcticAntics | 2 | 8981 | 231 |
| setmap | 13 | 627 | 33 |
| type | 2 | 363 | 12 |
| cd | 2 | 294 | 5 |
| spy1prg | 2 | 209 | 8 |
| Spy vs Spy (1989)(First Star Software)[cr Yakoul].adf:TheIslandCaper | 4 | 1267 | 71 |
| disk-validator | 1 | 386 | 1 |
| Spy vs Spy (1989)(First Star Software)[cr Yakoul].adf:spy3titles | 2 | 1899 | 107 |
| USA0 | 1 | 3 | 1 |
| Spy vs Spy (1989)(First Star Software)[cr Yakoul].adf:spy2 | 3 | 9260 | 179 |
| Spy vs Spy (1989)(First Star Software)[cr Yakoul].adf:bootblock | 1 | 86 | 2 |

## Cases handled

- spy vs spy (1989)(first star software)[cr yakoul]/Spy vs Spy (1989)(First Star Software)[cr Yakoul].adf:d/spy1prg: PowerPacker 2.3 (xfd) stream found by ancient scan (61896 -> 128880 bytes) but only 0 of 60 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- spy vs spy (1989)(first star software)[cr yakoul]/Spy vs Spy (1989)(First Star Software)[cr Yakoul].adf:d/spy1prg: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Spy vs Spy (1989)(First Star Software)[cr Yakoul].adf:ArcticAntics

- `START_CODE`: Jump stub: passes control straight on to START.
- `WAIT_FOR_RASTER`: Waits for the vertical retrace by polling the raster position register until it reaches a specific value, then checks that it hasn't exceeded another value before returning.
- `ATARI_COPY`: Copies 16000 source words into four planes 4000 words apart, taking consecutive words from the source in groups of four and placing one word of each group in each plane.
- `COLOUR_COPY`: Copies sixteen source words into the global block, each shifted left by one bit.
- `LOAD_FILE`: Opens dos.library through the exec library and stores its base and the value returned by the library's minus 30 vector.
- `SETUP_SCREEN`: Sets the current and alternate screen pointers, copies 56 words of plane pointers into the second copper list, and programs the display window, data fetch, modulo, control and copper registers.
- `WHICH_PLANES`: Fills four plane pointer entries, each storing the low and high halves of an address that advances by 8000 bytes from the previous plane.
- `BLIT`: Waits for the blitter to finish, then writes saved register values to the first-word and last-word masks, channel pointers, modulo values, control and size registers.
- `WAIT_BLIT`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `SWAP_SCREEN`: The decompiled body only waits for the blitter and then loops forever.
- `CLEAR_SCREEN`: Zeroes 32000 bytes starting at the address in the argument register, writing sixteen bytes per step.
- `MYINT2`: Calls the key matrix update routine, waits through a fixed delay loop, clears bit six of the shared byte and then writes the acknowledge value 8.
- `ADD_MATRIX`: Converts the raw keyboard byte to a key number by shifting it right one bit and inverting the low seven bits.
- `DOWN_STROKE`: Clears one bit, selected by the low three bits of the argument, in the byte at the base address plus the word offset.
- `INIT_KEY`: Sets four longwords of the key matrix to all ones.
- `INKEY`: Tests the key matrix bit for the key number in the argument.
- `SCAN_JOY`: Decodes the joystick port data of both ports into left and up values, using the fire button bits for the fire values.
- `HANDLE_MOUSE`: Reads mouse movement data from two input ports and processes them through another routine, then sets global flags based on button states from a potentiometer register.
- `DO_MOUSE`: Updates a mouse record by decrementing its counter bytes toward zero.
- `DOM_RIGHT`: Marks one field with 20 when the first movement byte exceeds one.
- `DOM_DOWN`: Sets the byte at offset five to 20 when the movement value exceeds one, then stores the new reading word.
- `ALLOCATE_MEMORY`: Locates available memory blocks and stores the returned bases into three global variables for the onscreen, twoscreen, and background buffers.
- `CLEARD0`: Zeroes 32000 bytes starting at the address in the argument register, one longword per step.
- `FIND32K`: Allocates memory through exec.AllocMem and returns the result when it is non-zero.
- `DECO_PIC`: Loads a file, then copies sixteen words from the back buffer into a table at the start of the target.
- `jump_to_test`: Jump stub: passes control straight on to TEST.
- `TEST`: Disables task switching through the exec library, runs a seventeen-iteration wait for a status bit to clear and to set again, then re-enables task switching through the exec library.
- `START_SOUNDS`: Sets the sound master flag, fills 8192 bytes with values from a shift-and-add pseudo-random generator, and initialises three channel records with envelope and dummy pointers and their audio channel registers.
- `GO_SOUND`: Fills a sound channel record with a sample pointer, two word parameters, a cleared status word, a flag set to one and a second pointer.
- `HIGH_PING`: Calls the shared sound routine and returns.
- `MEDIUM_PING`: Calls a routine to play a medium-frequency sound effect.
- `LOW_PING`: Calls a routine to play a specific low-frequency sound effect.
- `DROP_PING`: Performs an indirect call by loading a function pointer from a data structure and jumping to it, serving as a dispatch mechanism for a handler stored in a data table.
- `TAKE_PING`: Calls a routine to populate a two-part record with a pointer, two values, and initialized counters.
- `MEDIUM_BEEP`: Triggers a routine that fills a record structure with a pointer, two values, and specific flags.
- `BOP`: Executes a routine to write a 32-bit pointer and two 16-bit values into a record while zeroing counters.
- `BANG`: Invokes a routine to initialize a channel record with a pointer, two values, and a flag set to one.
- `OTHER_SMACK`: Performs a call to a routine that populates a data structure with a pointer and two 16-bit values.
- `GRAVESOUND`: Requests the filling of a two-part record containing a pointer, two values, and zeroed counters.
- `BOOM`: Calls a routine to set up a record with a 32-bit pointer, two 16-bit values, and a flag.
- `BURY`: Plays a sound effect by calling a routine that configures a sound channel.
- `SPLASH`: This routine invokes the sound system and then returns without performing any other action.
- `RUMBLE`: Calls a sound routine and returns without performing any further operations.
- `FEET`: Calls a sound routine and then returns without doing anything else.
- `PICKY`: It invokes a sound routine and then returns without performing any other operation.
- `SAWY`: Calls a sound routine and then returns without performing any other action.
- `KISS`: Calls the sound routine and then returns without doing anything else.
- `SUB`: It calls GO_SOUND and then returns.
- `HANDLE_SOUNDS`: Runs the sound request handler once, then up to three more times, repeating only while the previous call left the zero flag set.
- `HANDLE_REQUEST`: Looks for a pending request in four slots and returns minus one when none is pending.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `NEW_SOUND`: Queues a sound request identified by the low ten bits of its argument.
- `MYINT6`: Checks a hardware interrupt flag, calls a mouse handler routine, and manages sound processing by updating music timing and handling audio envelope changes.
- `process_music`: Updates music timing and handles sound envelope changes for multiple audio channels, calling various sound routines before clearing an interrupt request flag and returning.
- `CHANGE_ENVELOPE`: Advances one envelope channel.
- `END_OF_ENVY`: Loads the next envelope segment from the stream, following loop markers, then applies its increments to the channel's volume and period.
- `continue_envelope_segment_load`: Entry point into the envelope segment loader.
- `finish_channel_request_state`: Clears a status bit when the channel's counter is zero, then clears or masks the request word it points to.
- `ALLOW_AGAIN`: Clears the top six bits of the word at the address in the argument register, then writes the channel's period, volume and control values to the channel registers.
- `X8912`: Updates two sound channels from their state records.
- `DO_ENVEL1`: Resets the envelope fields of one channel record when its pending flag is set.
- `XNOISE1`: Points the noise channel at a random buffer with a length of 4096, sets a temporary waveform offset from the record, and updates the envelope fields or the volume of that channel.
- `XDONE1`: Writes one channel's wave, frequency, envelope and volume values from its record, choosing a wave table, frequency table or random buffer from the mode flags.
- `DO_ENVEL2`: When the record's pending byte is set, it writes a table value selected by the argument to the channel period, resets the envelope fields and clears the pending byte.
- `XNOISE2`: Sets a temporary waveform offset from the record and either resets the envelope fields or sets the envelope level from the record's value.
- `XDONE2`: Sets the pending byte of the channel record to zero and returns.
- `PSGW`: Stores the low byte of the second argument in the temporary music buffer at the first argument's offset plus two.
- `INITMUSC`: Sets the music-on flag and calls the byte-store routine three times.
- `PLAYMUS`: When music is on, plays the channel routine twice, restoring the saved delay and note pointers between the two calls.
- `store_two_music_bytes`: Calls the byte-store routine twice and returns.
- `PLAY_CHANNEL`: Steps one music channel.
- `PLAY2`: Reads the next note entry, following an indirect pointer when its first word is negative.
- `set_flag_and_store_three`: Sets the flag word to all ones and calls the byte-store routine three times.
- `clear_flag_and_store_bytes`: When the first argument is zero it calls the byte-store routine and clears the stored counter.
- `START`: Initializes memory, keys, and sound, sets up the screens, then loops forever calling the game routine while checking an abort flag.
- `READALL`: Runs a fixed sequence of picture reads, grab and buffer-save calls that load the picture files named in the program text into memory, then clears a block.
- `MYINT3`: Increments a counter, clears the interrupt request variable and writes 112 to the interrupt request register, then returns.
- `GRAB`: Calls the buffer save routine four times and returns.
- `GRABBIG`: Calls another routine three times in a loop, decrementing a counter until it reaches its limit.
- `GRAB1`: Calls the buffer save routine once and returns.
- `GRABOBJ`: Calls the buffer save routine twenty times and returns.
- `GRABLAND`: Small routine of 2 instructions that copies 20 to d3, then subtracts 1 from d3.
- `GRABLAND2`: Calls the short buffer store routine once for each step of a countdown from the argument's value through zero, then returns.
- `GRABMAP`: Runs RSAVE_BUFF three times in a row, decrementing a counter from 3 to 0 and repeating until it reaches zero.
- `PLAYINIT`: Resets the play state variables, copies the level's map and terrain tables into working memory using the level number, and clears the trap and snow lists.
- `PLAYINIT0`: Clears the menu, nudge and automatic flags, copies the level's map and terrain tables, and resets the rocket, trap, snow and timer variables.
- `GETA4`: Small routine of 3 instructions that clears d1, then copies (a4)+ to d1, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs three instructions (eori.b, cmpi.b, btst.l) and then falls through into the code that follows, with no return of its own.
- `WHICH3`: Sets several flag bits in three separate byte locations pointed to by registers, ORing the values 5, 9 and 5 into each.
- `WHICH4`: It sets bits 0 and 2 of the byte pointed to by the A6 register, leaving the other bits unchanged, then returns to its caller via the handler pointer.
- `WHICH5`: Sets bits zero and two in one byte and bits zero and three in another, then calls the buffer save routine for each nonzero entry of a 76-entry height table.
- `WHICH6`: Sets bits zero and three in one byte, then calls the buffer save routine for each nonzero entry of a 76-entry height table.
- `WHICH7`: Sets a bit in a byte at a fixed offset, then loops over a table of short values, calling another routine whenever an entry is nonzero, until a counter reaches its limit.
- `READ_G`: Walks the item height table one word at a time for a fixed count of entries and calls the buffer save routine for each nonzero height.
- `DRAW_NEW_SEL`: Performs the same clear, line, sprite and screen-swap sequence as the frame interrupt redraw, and is called from the parameter setup routine to repaint the selection screen.
- `PARMS`: Resets sound, demo and auto-play flags, copies the second screen buffer over the first, then polls triggers and joystick movement.
- `DO_GAME`: Initialises timers and counters, then repeatedly spawns a new hatch object into a free slot chosen with random values, checks the abort flag, toggles an animation value, and updates both players' busy state, map positions and window before moving and redrawing the scene.
- `READPIX`: Decodes a picture, copies it into the Atari buffer, and then copies its colour data.
- `STIT`: Repeatedly finds a random free table slot, stores a random word into it, and advances a counter until the requested count reaches zero.
- `STUFFSPY`: Chooses random map positions for both spies, scaling each coordinate to the map grid and storing the window offsets.
- `RNDER`: Updates the three-word random state with additive mixing and a rotate-and-xor step, leaving the new state in place.
- `RNDER2`: Advances the random state once through the generator and passes the register result back to the caller.
- `GETRAND`: Generates random offsets within a small range and repeats until the chosen table entry is empty, then returns that entry's address.
- `SETXY`: Converts a value taken relative to a base address into a word index and divides it by 96, storing the quotient and remainder as two coordinates.
- `CLEAROUTCRAP`: Sets the four clock digit counters and the two fuel counters to the all-ones sentinel value.
- `GOERROR`: Restores CPU registers from the stack, sets the vertical blank vector, resets the palette and a screen buffer, prints text and hex values, then enters an infinite loop.
- `BLOOP`: Jump stub: passes control straight on to BLOOP.
- `OPEIGHT`: Calls the four-value hex print routine twice in sequence to emit eight values.
- `OPFOUR`: Repeats a hex output followed by a text output until a count held in the register reaches zero, producing four value and label pairs.
- `OPHEX`: Repeatedly emits one hex item through the item printer until the remaining count reaches zero.
- `OPTEXTZ`: Emits each character of a zero-terminated string through the item printer until the terminator is reached.
- `emit_text_until_nul`: Duplicate entry into the zero-terminated text loop: it prints characters through the item printer until a zero byte.
- `print_text_tail_entry`: Prints characters through the item printer until a zero byte and then continues into the zero-terminated text routine.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a7)+ to d0, then returns.
- `OPITEM`: Copies an eight-row character glyph from the character table into four screen bitplanes, skipping the blank character.
- `return_register_value`: Returns the value currently held in a register without modifying it.
- `ILLEG`: Jump stub: passes control straight on to GOERROR.
- `SETUPZ`: Stores the illegal-instruction handler into three vector slots so that faults during start-up are routed to that handler.
- `DRAWSPY`: Tests whether spy 1 or spy 2 lies in the current window cell, computes the screen plot position, and draws the body and hand sprites, which are drawn through the hand routine.
- `DRAWHANDS`: Selects the hand state and screen offset for spy 1 or spy 2 and adjusts the position for the spy's current cell.
- `DRAWBUTTONS`: Draws seven button icons by calling the single button drawer seven times.
- `ONEBUTTON`: Draws one button icon into the second screen, choosing the empty or full glyph from a flag, and then calls the sprite routine.
- `SWAPSCREEN`: Jump stub: passes control straight on to SWAP_SCREEN.
- `DRAWTIME`: For each of the four clock digits, compares it with the previously drawn digit and redraws the character only when it has changed.
- `HLINE`: Passes the current register values straight to the horizontal line routine and returns.
- `METER`: Clamps each spy's energy between zero and its maximum, then draws the meter segments when the clamped value is neither empty nor full.
- `FLASH`: Draws two sprites, and on alternating counter phases calls the flash routine twice to draw the flashing status sprites.
- `FLASH_ONE`: Clears a flash counter and, depending on a state value, draws up to two sprites while incrementing the counter in the flashing states.
- `COMPARETWICE`: Compares the input word against a stored two-word pair and updates the pair only when it differs.
- `SPRITER_MOD`: Selects the spy state for spy 1 or spy 2 and, when no rocket is active, draws the sprite in two parts if it is wider than the limit and the hand state is set.
- `RSPRITER`: Shifts a sprite's mask and four source rows by the horizontal offset, inverts the combined mask and ANDs it into the four screen bitplanes over the sprite's width and height.
- `RBACKER`: Clears the target cell in the four bitplanes, then ORs shifted background source rows into the screen with a per-row bit offset.
- `continue_background_composite`: Continues the background compositing loop by stepping through rows and columns, clearing the next column when a column finishes and ORing shifted source rows into the screen.
- `RSAVE_BUFF`: Copies a rectangle of four screen bitplanes into the save buffer, storing the planes for each word in order.
- `RDRAW_BUFF`: Copies a rectangle of four saved bitplane words from the buffer back onto the screen, reversing the save copy.
- `RSTUFF_BUFF`: Writes a size header for a rectangle of the given width and height, then packs each cell into the buffer as the OR of its four bitplanes followed by the individual planes.
- `RSTUFF_BUFF_SHORT`: Writes a size header for a rectangle, then copies each of the four bitplane words of every cell into the buffer in order without combining them.
- `RSPRITER_AM`: Programs the blitter's channel pointers, modulos, masks and size, then starts it for a masked sprite copy, waiting for each blit to finish.
- `SPRITER`: Thin wrapper that calls the masked sprite routine with the current register values and returns.
- `SPRITER_AM`: Thin wrapper that calls the blitter sprite routine with the current register values and returns.
- `BACKER`: Thin wrapper that calls the background compositing routine with the current register values and returns.
- `MINTERMS`: Advances the snow timing and slab selection and then calls the snow drawing routine for each spy window that is not inside an igloo.
- `RSNOWING`: Advances the snow timing and slab selection, then draws snow into each spy window outside an igloo.
- `DOSNOW`: Draws one column of snow rows across the four bitplanes, choosing the snow pattern from the snow counter and combining each row with AND or OR masks.
- `PUTCHAR`: Calculates character position in a display buffer using shift operations on register values, then calls another routine to process the next character.
- `NEXTCHAR`: Looks up the character cell in the font table from the low nibble of the input and copies eight rows into four bitplanes at the target.
- `CALCAD`: Consists of fifteen instructions but performs no functional operations or data manipulations before returning.
- `PLOTPOINT`: Clears the pixel bit in each of the four bitplanes at the target and then sets the bits given by the four colour words.
- `LEFT_RIGHT`: Tests the leftward direction flag and the low bit of the step value, but returns without any effect in either case.
- `UP_DOWN`: Tests the direction flag and returns without any effect.
- `DRAW_LINE`: Computes the horizontal and vertical distances and their directions, then plots points along the line, stepping with an error accumulator that moves vertically or horizontally at each step.
- `HORLINE`: Finds the starting point and fills a horizontal span of the four bitplanes, setting or clearing bits to match each colour, with a boundary path for partial words.
- `clear_span_bitplane_rows`: Clears the first bitplane at the start of a span, then sets or clears the four planes across the remaining columns and moves to the next block when a block fills.
- `fill_span_second_plane`: Continues the span fill starting from the second bitplane, writing plane bits for each column and moving to the next block after sixteen columns.
- `fill_span_third_plane`: Continues the span fill starting from the third bitplane, writing plane bits for each column and moving to the next block after sixteen columns.
- `fill_span_fourth_plane`: Starts the span fill at the fourth bitplane, writing bits across the columns and tail-branching to the horizontal span routine when the span ends.
- `GOT_BOUNDARY`: Writes whole sixteen-pixel words of solid colour across all four planes, then handles any leftover pixels with a bit mask.
- `write_span_block_words`: Sets whole words of the span to the colour pattern across the planes in blocks of sixteen pixels, then finishes the remaining pixels bit by bit.
- `write_span_plane_two_words`: Writes whole words of the span from the second plane in blocks of sixteen pixels, then finishes the remaining pixels bit by bit.
- `write_span_plane_three_words`: Writes whole words of the span from the third plane in blocks of sixteen pixels, then finishes the remaining pixels bit by bit.
- `write_span_plane_four_words`: Writes whole words of the span from the fourth plane in blocks of sixteen pixels and then tail-branches to the full word writer for the remaining pixels.
- `LATER`: Sets or clears one pixel column at a time across all four planes using a rotating bit mask until the count runs out.
- `write_rotating_span_bits`: Rotates the mask one pixel per column and writes the colour bits into the planes, continuing the span with the next colour word.
- `write_rotating_span_plane_two`: Rotates the mask one pixel per column and writes colour bits into the second plane, continuing the span with the next colour word.
- `write_rotating_span_plane_three`: Rotates the mask one pixel per column and writes colour bits into the third plane, continuing the span with the next colour word.
- `write_rotating_span_plane_four`: Clears the fourth plane word at the target, then writes rotating colour bits across the four planes for each remaining column.
- `POINT`: Combines the register inputs and returns them masked to keep only the bits used for a pixel position and bit index.
- `GET_RANDOM`: Updates the three-word random state with additions and a subtraction, then rotates a value by its low bits and folds it back into the state.
- `READ_TRIGS`: Scans the joysticks, then sets each player's trigger word from the fire button for that player's control mode, reading the keyboard when the mode calls for it.
- `MOVE`: Moves each spy according to its control mode: a human player reads the joystick and keyboard, while a computer player uses the brain or random movement.
- `set_spy_direction_four`: Sets the spy direction to four and stores the value two through the pointer it is given.
- `set_spy_direction_three`: Sets the spy direction to three and stores the value one through the pointer it is given.
- `run_spy_action_sequence`: Sets the spy direction and a state value, then runs a long sequence of routines covering busy handling, meeting checks, trap deletion, footstep counting, sound, random choices and state setting.
- `CHECKMEET`: Aligns both windows to the same row band, then, when the spies share a band and neither is inside an igloo, checks their horizontal distance to decide whether they meet or separate the windows.
- `update_spy_window_2`: Calculates window coordinates for spy characters based on map positions, updates global variables, and adjusts window positioning when spies are close together or separated.
- `update_spy_window`: Adjusts window coordinates for spy characters based on map positions and window size, updating global variables while checking boundary conditions and calling additional routines for window adjustments.
- `adjust_window_for_spy_two`: Continues the window adjustment toward spy two, counting the steps, and separates the windows when spy two's window is no longer valid.
- `separate_windows_toward_spy_two`: Adjusts the shared window toward spy two in steps of two, clearing the buffer markers and separating the windows when they no longer overlap.
- `reset_spy_one_window`: Clears the buffer markers, sets the window to spy one's position with a left margin and a minimum edge, updates the temporary positions, and returns through the meeting exit routine.
- `CHECKMEETEND`: Clears the two buffer markers when the meeting counter is greater than one, then returns.
- `SETTEMPS`: Copies the window and map position of spy one or spy two into the shared temporary position variables, depending on the input.
- `DIRFIX`: Chooses the animation buffer for a spy from its direction and advances that spy's frame index, wrapping at four.
- `TEST_OPTIONS`: Counts down a special delay and runs the option test routine whenever the delay has reached zero.
- `TEST_M`: When no key is pressed, reloads the special delay and toggles a test flag, setting or clearing three audio volume words and the related state words.
- `JOYMOVE`: Scans the joystick and option state, waits through the key and delay loops, and sets the abort flag when a key is pressed.
- `SETWINDOW`: Copies the window position for the active spy into the shared window position variables.
- `GETWINDOW`: Copies the active spy window position into the window two variables or the window one variables, depending on the active spy and the input.
- `LITTLE_DELAY`: Counts a fixed value down to zero as a short pause and returns its input word unchanged.
- `DRAWMOVE`: Performs the same redraw sequence as the status update: flash, landscape and objects for each window, buttons, clock, energy cooldown, meters, snowball, snow and lines.
- `RLINES`: When the clock minutes are zero, advances the snow line counter and draws the snow line columns into the second screen for each spy that is outside an igloo.
- `MOVE_SNOWBALL`: Advances each active snowball entry in the snowball list by its stored step, checking the map tile for blocking flags and the spy positions, and updates the entry in place.
- `DRAWLAND`: Sets the background pointer and bounds for window one or window two, then blits the landscape slabs through the slab routine, waiting for the blitter between steps.
- `ONE_SLAB`: Computes a source address from a row index and offset, then programs the blitter with that address, sizes and masks to copy one landscape slab, waiting for each blit to finish.
- `DRAWSLAB`: Calls the single slab blit routine and returns.
- `FIRE_IN_HERE`: The decompiled body is empty apart from a return, so this routine has no visible behaviour and acts as an empty stub.
- `DRAWOBJ`: Sets the object window origin for the active window, then loops over the object grid, drawing object cells through the spy object drawer and the sprite routine.
- `DRAWSPYOBJ`: Sets the origin for spy window one or two, loops over a grid of eleven columns and four rows drawing object cells, and draws each spy through the spy drawing routine.
- `BUSY`: Shows the same busy-state handler body for the chosen spy, checking auto, igloo and dead flags and calling the joystick move routine when the trigger and timer allow.
- `BUSY4_A`: Selects spy one or spy two, copies its map position into the shared spy position, and clears its hand state.
- `DELETE_TRAP`: Searches the trap list for the given address, restoring the saved value and clearing the entry when it is found, or clearing the target word when it is not.
- `ADD_TRAP`: Checks the spy's auto flag and the target word, scanning forward or backward for a suitable location, then records the trap address in the first free slot of the trap list.
- `DOSETXY`: Calls the coordinate split routine only when the direction code is one of eight recognised values, and returns without action for any other code.
- `BUSY5_0`: Shares the map width selection body with the counter routine: it reads the joystick, then sets the map sprite width for the chosen index and shows the map.
- `draw_trail_segment_line`: Computes the horizontal and vertical deltas between two stored trail points, positions the spy through the position routine, draws a horizontal line segment, and then sets the state.
- `reset_spy_flash_state`: Clears both spies' flash words, then clears the chosen spy's state counter and flash word and returns.
- `initialize_spy_state`: Initializes spy state by setting up a global variable to a specific value, then branches to another routine after storing the value in a memory location.
- `update_spy_state`: Updates spy state based on input and game conditions, handling menu navigation and item usage.
- `BUSY6_4`: Clears the state counter and the flash word of the chosen spy, one for spy one and one for spy two, and returns.
- `SETSAFE`: Sets the safe flag of spy one or spy two to four, as selected by the input.
- `KILLDOUBLE`: Stores the frame counter minus twenty as the last trigger time for the chosen spy, which the delay table reads to reject repeat triggers.
- `GETPOS`: Converts a map column into a screen X position by dividing by four and then by ten and scaling, and converts a map row into a screen Y, storing both in the screen position variables.
- `GETPOS1`: Converts a cell column and row into a screen X position and Y position by scaling and offsetting them.
- `SETSTATE`: Stores the given value as the state counter of spy one or spy two, and clears that spy's flash word when the value is zero.
- `GETBTIME`: Checks if a value in a register is zero and returns early when it is.
- `SHOWMAP`: Sets the map sprite position and width, selects the map image for the current level, and draws it through the sprite routine.
- `ABSD2`: This routine is empty and performs no operations before returning.
- `CLEANTRAIL`: Sets the six trail entries for spy one and spy two to the all-ones value, clearing their recorded trail positions.
- `SETUPA2`: Tests the input word against zero and returns with no effect in the visible body.
- `SETUPA2D`: Tests the input word against zero and returns with no effect in the visible body.
- `SETUPA2E`: Tests the input word against zero and returns with no effect in the visible part.
- `GETMODE`: Runs the title and mode selection loop: it counts a demo timer from 500, redraws the background, sprites and menu items each pass, swaps the screens, and reads the keyboard to choose a game mode.
- `DRAWITEMS`: For each menu item, computes the line endpoints from the item index and calls the line routine four times to draw the item's box.
- `SELECTITEM`: Empty routine: returns to the caller immediately without doing anything.
- `DRAWLINE`: Thin wrapper that calls the line drawing routine and returns, so menu code can draw a segment between the current point registers.
- `RCLEARBLOCK`: Writes a block of the screen with the first bitplane set and the other three cleared, row by row.
- `BRAINMOVE`: Chooses the computer spy's next move: it searches for a mound, fires when the fire routine applies, and picks a direction from random values, setting the state for the chosen action.
- `GETD0D1`: Sets the spy direction to three, four, one or two from the first of four direction bits that is set in the input, or to zero if none is set.
- `BRAINBUSY1`: When the state word at the pointer is zero, resets the frame index for spy one or spy two and stores a fixed state code in that word.
- `reset_spy_two_frame_state`: Clears spy two's frame index and stores the fixed state code at the pointer it is given.
- `RANDMOVE`: When the spy is not inside an igloo and a random roll matches a fixed value, picks a random direction from two further random bits and sets the spy direction.
- `FIND_A_MOUND`: Scans a small block of map entries around the spy's position, looking for a mound tile, and records the matching position.

### setmap

- `start_program_and_open_libraries`: Program entry point: sets up the library and task globals, opens the DOS library and the current directory, then calls the helpers that start the program.
- `close_libraries_and_reply`: Cleanup path that closes the libraries, forbids multitasking, replies to the message and raises an alert when an error code is set.
- `invoke_two_library_functions`: Calls two library functions through pointers read from fixed offsets of a base register, one after the other, and returns.
- `open_dos_library_or_fail`: Opens the DOS library; if that fails it sets an error code of one hundred and runs the alert and cleanup path, otherwise it returns the library handle.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_helpers_with_global_flags`: Conditionally calls a helper with global values, passing the address of a global when a flag is set, then calls it again with a fixed global and the incoming parameter.
- `store_parameters_and_call_helpers`: Stores its two parameters in two global variables, then calls a helper with fixed arguments that reference a global and constants, followed by a second helper call with the constant twenty.
- `load_keymap_for_console`: Start-up routine that opens the console, icon and keymap resources and loads the keymap named in the tool types or the command line, reporting an error when a step fails.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_head`: Library stub: loads its arguments and the exec library base, calls AddHead and returns its result to the caller.
- `call_find_name`: Library stub: loads its arguments and the exec library base, calls FindName and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_alloc_signal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `call_free_signal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `call_put_msg`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_open_device`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `call_close_device`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `call_open_resource`: Library stub: loads its arguments and the exec library base, calls OpenResource and returns its result to the caller.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `pass_param_with_stack_slot`: Calls a helper with a global value, its parameter, and the address of an eight-byte stack slot.
- `call_indirect_table_entry`: Calls one function through a pointer read from a global variable at a fixed offset, then returns with no other operations.
- `measure_and_pass_stack_string`: Calls a function through a global pointer, measures the length of a 140-byte stack string up to its terminator, and passes the string and its length to another routine when the length is nonzero.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_words_then_call`: Copies sixteen long words from a table at a fixed offset into a buffer when the buffer pointer is non-null, then calls through a stored code pointer.
- `initialize_structure`: Initializes several fields in a data structure pointed to by a1, clears specific memory locations, and sets up a counter in a global variable.
- `jump_to_dispatch`: Unconditionally branches to another routine using a relative word offset and does nothing else.
- `set_global_value`: Sets a global variable to negative one and stores that value at an offset from another global variable's address before branching to a different routine.
- `set_another_value`: Sets a global variable to negative one and stores that value at an offset from a global variable's address before branching to a different routine.
- `process_data_with_jump`: Loads data from a table using an index, processes the loaded value, and stores the result in a global structure before jumping to another routine based on the processed value.
- `dispatch_by_value`: Loads a byte from memory, compares it against specific constants, and branches to different routines based on the comparison results.
- `update_and_execute`: Increments a counter in a structure, performs several conditional checks and assignments based on values in memory, and executes a series of function calls through pointers in registers before branching to another routine.
- `initialize_and_execute`: Initializes a global variable at offset 616 of the structure in a2 with the value 1, then performs a series of function calls through a5 using different offsets from a2, updating values at various offsets and checking conditions before branching to another routine.
- `jump_to_handler`: Immediately jumps to another location in the code without performing any other operations.
- `execute_indirect_call`: Performs two indirect function calls using pointers stored in registers, then jumps to another location through a pointer in a6.

### cd

- `find_largest_list_entry`: Walks a linked list of entries, keeping the largest value found, after a library call sets up the list.
- `copy_words_then_call`: Copies a fixed number of words from one memory location to another while the destination pointer is not null, then invokes a routine through a stored pointer.
- `xor_and_shift_bytes`: XORs a byte value with a bit mask derived from a register, then shifts and stores the result in a global structure.
- `load_and_process_2`: Loads data from a table using an index, processes the loaded value, and stores the result in a global structure before branching to another routine.
- `load_and_process`: Loads data from a table using an index from a global structure, processes the loaded value, and stores the result in another global structure.

### spy1prg

- `load_hunk_executable_segments`: Loader routine that allocates and frees memory through the exec library, compares hunk type codes while processing records, and relocates the loaded segments.
- `empty_return_program_stub`: The decompiled body is empty apart from a return, so this routine has no visible behaviour and is an empty stub in the program.
- `count_down_delay_loop`: Decrements a count in the register until it reaches zero, a busy-wait delay.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (lsl.l) and then falls through into the code that follows, with no return of its own.
- `copy_byte_runs_from_codes`: Repeatedly calls a helper for codes and copies byte runs between a source and a destination buffer, adjusting the pointers and counts from the returned values until the terminator is reached.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_character_loop`: Loops a count of times decrementing a byte and wrapping it to the space character when it reaches zero.
- `stub_overlapping_bad_instructions`: Decompiler output shows overlapping bad instruction data, so this is not reliable code.

### Spy vs Spy (1989)(First Star Software)[cr Yakoul].adf:TheIslandCaper

- `jump_to_init`: Jump stub: passes control straight on to INIT.
- `E_EXPLODE`: Plays the explosion sound: marks the sound channel, starts the sound system, runs ten cycles of dive and explode sequences while waiting on the audio flags, then restores the saved interrupt.
- `E_DIVE`: Runs the same sound sequence as the explosion routine: ten dive and explode cycles with audio waits, then restores the saved interrupt.
- `TEST`: Runs the same ten dive and explode sound cycles as the explosion routine, bracketed by interrupt handling calls.
- `START_SOUNDS`: Sets the sound master enable and music pointers to zero, fills a random area of sound data with values from the random generator, programs the audio channels and interrupts, then initialises the music and installs the sound interrupt handler.
- `SHOOT`: Starts the shoot sound on audio channel three by setting its volume and sample pointer to the random sound area, its length and period, clearing its envelope counters, and pointing the envelope at the shot routine.
- `SHOOT2`: Sets the length of the channel three sample to the standard value.
- `EXPLODE`: Sets up audio channel three for an explosion: clears the volume and sample counters, sets the sample pointer to the random sound area with a period of three thousand, and points the envelope handler at the explosion routine.
- `DIVE`: Sets up audio channel two for a dive sound: sets the volume, uses the triangle waveform table with a period of 2130, resets its counters, and points the envelope handler at the dive routine.
- `MYINT6`: Interrupt handler for the sound system: when sound is enabled it counts a music delay, plays the next music step and updates each active channel's envelope, then acknowledges the interrupt.
- `play_music_step_and_envelopes`: Resets the music delay, plays the next music step when the music switch is on, updates the active channel envelopes, and acknowledges the audio interrupt request.
- `CHANGE_ENVELOPE`: Advances one channel's envelope: when its segment counter has run out it loads the next three-word segment, otherwise it decrements the counter and adds the step values to the volume and period, then writes the volume.
- `X8912`: Selects the sound source for one channel from its mode bits and sets its audio pointer, then branches to the envelope, noise or completion routines as the mode requires.
- `DO_ENVEL1`: When the trigger flag is set, loads the starting value from the envelope table, resets the segment counter and volume, then calls the envelope completion routine.
- `XNOISE1`: Sets up a noise sample for one channel: points the audio pointer at the random sound area with a length of 4096, sets the period from the music byte, and resets the channel counters when the mode byte equals sixteen.
- `XDONE1`: Finishes playback on channel one: unless both of the channel's flag bits are set, it loads the sample pointer from the wave length table and the period, then branches to the envelope or noise completion routines.
- `DO_ENVEL2`: When the trigger flag is set, loads the envelope start value from the table, resets the segment counter and volume, then clears the flag and finishes through the completion routine.
- `XNOISE2`: Sets up the noise sample for the second channel from the music data, pointing at the random sound area with a length of 4096, and resets the counters when its mode byte equals sixteen before branching to the finish routine.
- `XDONE2`: Clears the trigger flag of the channel, ending its envelope start.
- `PSGW`: Writes the data byte to the sound chip register whose number is given, through the register array.
- `INITMUSC`: Sets the sound chip mixer register to its initial value, writes that register three times, then turns both voices off.
- `PLAYMUS`: Plays the music on channel zero and channel one in turn: it selects each channel, switches its voice on, and sends its output.
- `VOICEISOF`: Turns a voice off by clearing its gate flag and resetting its sequence counters, then steps to the next sequence entry when the counter runs out.
- `NEXTSEQ`: Advances the music sequence: when the counter runs out it moves the pointer on by three bytes, then loads the note, duration and volume fields from the next sequence entry.
- `VOICISON`: Decrements the note timer for a voice; when the timer expires it loads the next control byte into the interrupt register and, if the voice is active, advances to the next note.
- `NEXTNOTE`: Counts down the note on a voice and, when the note's control byte is positive, gates the voice off and advances to the next note through the note routine.
- `NEXTNOTO`: Advances a voice to its next note: steps the note pointer by three until it reaches the end marker, advances the sequence when needed, then loads the duration from the duration list and continues the song.
- `CONTSONG`: Loads the next note's duration from the duration list; for short values it returns, otherwise it gates the voice, checks the pitch byte, and calculates and stores the note value.
- `calculate_channel_note_value`: Gates the voice and, when the pitch byte is set, checks the channel and calculates the note value, storing it along with the channel's second value.
- `store_calculated_note_value`: Runs the channel check and note calculation, then stores the resulting note and the channel value into the voice state.
- `DEGATEIT`: Clears the gate flag of the voice, ending its note gate.
- `CHECK2`: For channel zero, checks the channel's mode bits, updates the mixer register, writes the sound register, sets a noise or tone flag, and branches to the snare or output routine as needed.
- `SNAREA`: Sets the mixer for the snare sound, writes three sound chip registers, and records a volume value of sixteen for the channel before branching to the channel routine.
- `CHANNELB`: Chooses one of two PSG mixer masks for channel B according to the status bits (bits 4 to 6) of the channel record, writes it to the mixer register and sets the record's volume field to 16, or to 12 in one of the branches.
- `SNAREB`: Sets a fixed PSG mixer mask for channel B, writes 16 to the channel record's volume field and issues several PSG writes.
- `CALCNOTE`: Calls NOTEMODS and takes the low three bits of the upper nibble of the result as a count.
- `OUTVC1`: Calls a sound output routine and then returns.
- `OUTVC2`: It invokes the sound output routine and then returns without modifying any data.
- `SNDOUT`: Calls the PSG write routine PSGW three times in a row and returns.
- `NOTEMODS`: Adjusts a note byte by a step held in another register: steps of 0 to 7 add to the low nibble and carry into the upper bits, while larger values subtract.
- `NOTEMOD5`: Negates the note byte, adds it to a second register and, when no carry occurs, corrects the sum by four and the saved byte by sixteen.
- `NOTEMOD6`: Writes IRQA with the low nibble of the note byte combined with bits 4 to 6 of the saved byte, with no arithmetic.
- `XXMOD5`: Negates the low three bits of the note byte, adds the result to a second register and applies the same correction as NOTEMOD5 when no carry occurs.
- `XXMOD1`: Writes IRQA with the note byte ORed with bits 4 to 6 of the saved byte and returns.
- `RESET`: Issues three PSG write calls in sequence and returns, with no other register work in this routine.
- `INIT`: Allocates memory, starts the sound system, sets up and clears the screen and decodes the picture, then copies it to the back buffers and copies the colour table.
- `BLIT_ME_HONEY`: Loops over 4000 words, ANDing each source word with a mask word and ORing the result into the destination on four bitplanes spaced 4000 words apart.
- `PIXEL_PIECE`: Calls BLIT_ME_HONEY_PIECE sixteen times in a counted loop and returns.
- `BLIT_ME_HONEY_PIECE`: For each of twenty columns, clears the destination bits covered by a mask word and then ORs the masked source bits into four bitplanes spaced 4000 words apart, advancing the mask one word per column.
- `MOVE_THE_JETS`: Copies a full screen buffer from the first buffer to the second, then sweeps two jet sprites across the screen in turn with SHOW_JET, blowing a hole every eight steps.
- `DO_RESTORE_OF_JET`: Copies a three-longword-wide strip over 32 rows from the second screen buffer back into the first, for each of four bitplanes.
- `SHOW_JET`: Waits for the raster, clears the master enable flag and restores the background strip, then takes the next animation frame from a table that wraps every four frames.
- `WAIT_FOR_RASTER`: Polls a 17-bit beam counter read from a global value, which is updated from the VPOSR register, until it falls inside a window that starts at the input value and is about 2816 counts wide.
- `BLOW_A_HOLE`: Advances a hole index modulo eight and, when a horizontal position offset by 96 is below 310, calls MOVE_A_HOLE twice.
- `MOVE_A_HOLE`: Over a five-column block, clears the bits under a source pattern in four bitplanes spaced 4000 words apart and refills them from the background.
- `ATARI_COPY`: Loops over 4000 words, taking four consecutive source words and writing each one to a different bitplane, the planes being spaced 4000 words apart.
- `COLOUR_COPY`: Converts sixteen source words into a palette table by shifting each left one bit and setting the lowest bit of each three 4-bit colour channels.
- `LOAD_FILE`: Opens the DOS library to get its base address, then opens a file through it to obtain a handle.
- `SETUP_SCREEN`: Sets the two screen buffer pointers, copies the plane pointer table into the second copper list and writes the display window, data fetch, bitplane control and modulo registers.
- `WHICH_PLANES`: Writes the high and low words of four bitplane pointers into a table, each plane starting 8000 bytes after the previous one, beginning at the address given in the input register.
- `BLIT_A_SPY`: Sets a blitter control value, then runs BLIT once and BLIT2 three times.
- `BLIT`: Writes the blitter mask, modulo and control registers from global values, then calls WAIT_BLIT until the blitter has finished.
- `BLIT2`: Writes the blitter pointer registers and the blitter size register from global values, then waits for the blitter to finish.
- `WAIT_BLIT`: Loops while bit 6 of the DMA control read-back is set, and returns once the blitter reports that it is idle.
- `SWAP_SCREEN`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `CLEAR_SCREEN`: Zeroes 2000 blocks of four longwords, 32000 bytes in total, which is the size of a four-plane screen with 8000 bytes per plane.
- `ALLOCATE_MEMORY`: Calls the 32K memory allocator six times and stores each result as the load screen, the two screen buffers, the title, jet and credits buffers.
- `RESCIND_MEMORY`: Calls the 32K memory release routine six times, undoing the six allocations made by ALLOCATE_MEMORY.
- `FIND32K`: Calls the system memory allocation function for a 32K block and returns the pointer when the request succeeds.
- `LOSE32K`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `DECO_PIC`: Loads a file, copies sixteen words from the start of the loaded data, then unpacks run-length packed picture data into four bitplanes of 8000 bytes each.

### disk-validator

- `compare_bitmap_bits_and_count`: For each block in a range, tests one bit in two bitmap word arrays and counts the positions where the two bits differ.

### Spy vs Spy (1989)(First Star Software)[cr Yakoul].adf:spy3titles

- `START_CODE`: Allocates memory, starts the sound system, sets up the screen and loads the data, then waits in a counted delay loop.
- `WAIT_FOR_RASTER`: Synchronizes execution to a specific raster line by comparing a target value with the current vertical position register, looping until the condition is met.
- `ATARI_COPY`: Loops over 4000 words, taking four consecutive source words and writing each one to a different bitplane, the planes being spaced 4000 words apart.
- `COLOUR_COPY`: Copies sixteen source words into a table, shifting each one left by one bit.
- `LOAD_FILE`: Opens the DOS library through the system exec base, stores the returned library base in a global, then makes two further library calls on that base.
- `SETUP_SCREEN`: Sets the screen buffer pointers, fills an eight-entry table with two fixed words, and copies 51 words of plane pointer data into the second copper list.
- `WHICH_PLANES`: Splits a 32-bit address into its high and low 16-bit halves and stores each half into consecutive entries of a pointer table, advancing the address by 8000 bytes and repeating four times.
- `BLIT`: Waits for the blitter to become idle, then loads the blitter mask, modulo, pointer and size registers from global values.
- `WAIT_BLIT`: Spins in a loop polling a hardware register until a specific bit clears, synchronizing execution with the beam position.
- `SWAP_SCREEN`: Waits for the blitter to become idle, then enters an endless loop.
- `CLEAR_SCREEN`: Writes zero into 2000 groups of four 32-bit words starting at the given pointer, clearing 32000 bytes.
- `ALLOCATE_MEMORY`: Stores the addresses of three screen buffers, SCREENA, SCREENB and SCREENC, into the one-screen, two-screen and back-buffer globals and returns.
- `FIND32K`: Calls the system allocation function for a 32K block and returns when it succeeds.
- `WHOS_REMOVED_THE_RAM_CHIPS`: Contains only an endless loop with no other work.
- `DECO_PIC`: Copies sixteen 16-bit values into a destination table, then decodes three 8000-byte bitplanes from a stream of literal bytes and marker-introduced runs, writing each byte with interleaved spacing.
- `jump_to_test_2`: Jump stub: passes control straight on to TEST.
- `TEST`: Waits for a status bit to clear and then set in a short loop, then disables task switching, writes to a hardware register, and re-enables task switching.
- `START_SOUNDS`: Fills an 8192-byte table with values from a pseudo-random generator, then resets the music switch and the voice records.
- `GO_SOUND`: Stores the input pointer and the input values into a sound record, sets a global to 255 and marks the record as active with a count of one.
- `HIGH_PING`: Calls GO_SOUND and returns, so it simply passes the sound parameters already set up in the registers to the common sound path.
- `MEDIUM_PING`: Invokes a routine to play a sound and then returns without performing any additional work.
- `LOW_PING`: This routine invokes a sound routine and then returns.
- `DROP_PING`: This routine performs an indirect call by loading a function pointer from a data structure and jumping to it.
- `TAKE_PING`: Invokes the sound routine and then returns without modifying any data.
- `MEDIUM_BEEP`: This routine invokes the sound subsystem and then returns without modifying any data.
- `BOP`: It invokes a sound routine and then returns without altering any data.
- `BANG`: It invokes a routine to initialize a channel record with a pointer, two values, and a flag set to one.
- `OTHER_SMACK`: Invokes a sound routine that populates a data structure with a pointer and two 16-bit values, then returns.
- `GRAVESOUND`: It invokes a sound routine and then returns without modifying any data.
- `BOOM`: It triggers a sound via a sound routine and then returns.
- `BURY`: This routine invokes the sound subsystem and then returns without modifying any data.
- `SPLASH`: It invokes a sound routine and then returns without altering any data.
- `RUMBLE`: Calls GO_SOUND and returns, so it passes the sound parameters already set up in the registers to the common sound path.
- `FEET`: This routine invokes a sound routine and then returns without modifying any data.
- `PICKY`: PICKY is a routine that triggers a sound effect by invoking the sound subsystem.
- `SAWY`: Calls a sound routine and then returns without modifying any data.
- `KISS`: Initializes a sound channel by writing a pointer and specific 16-bit values into a data record, then triggers the sound routine.
- `SUB`: It invokes the sound routine and then returns without altering any data.
- `HANDLE_SOUNDS`: Calls HANDLE_REQUEST up to four times in a row, repeating each call only while the zero flag left by the previous call is set.
- `HANDLE_REQUEST`: Checks four request slots in order, skipping any slot whose value is zero or negative, and returns minus one if none holds a request.
- `return_no_request_status`: Returns the fixed failure value minus one and does nothing else.
- `NEW_SOUND`: Queues a sound request by storing its code in a free request slot, or by setting a flag on an existing matching slot.
- `MYINT6`: Runs on each interrupt tick while master enable is set: every fourth tick it calls PLAYMUS and X8912 when the music switch is on.
- `service_music_tick_unguarded`: Advances one music tick without the master enable check: it resets the delay to three, plays the next music step with PLAYMUS and X8912 when the music switch is on, updates the voice envelopes and runs the sound request handler.
- `CHANGE_ENVELOPE`: Advances one voice's volume envelope.
- `END_OF_ENVY`: Walks an envelope table until it reaches an entry whose first word is non-negative, handling control codes on the way.
- `scan_envelope_for_next_step`: Walks the envelope table forward, skipping control words, until it finds an entry whose first word is non-negative.
- `clear_voice_flags_and_output`: Clears the high bit of a voice flag word when its counter is zero, masks a volume word, and copies the voice's output words.
- `ALLOW_AGAIN`: Clears the top bit of a voice control word, copies two output words for the voice and writes a global status value.
- `X8912`: Updates the Amiga audio channels from their music state records.
- `DO_ENVEL1`: When an envelope request flag is set, writes the envelope's first value to the channel volume and clears the channel state.
- `XNOISE1`: Sets the channel's noise state from its music value and points it at the random-value table.
- `XDONE1`: Writes the frequency, volume and loop registers of an audio channel from its state, applying the envelope when the mode bits require it.
- `DO_ENVEL2`: When an envelope request flag is set, writes the first envelope value to the second channel's volume and resets its state.
- `XNOISE2`: Points the noise source at the random-value table, sets the noise state from the channel's music value and writes the channel's noise registers.
- `XDONE2`: Clears the envelope request flag of the second audio channel state and returns.
- `PSGW`: Stores one byte into the sound chip register selected by the index value and returns.
- `INITMUSC`: Turns music on, writes three PSG registers through PSGW, points the two music channels at their note tables and clears their delay and flag values.
- `PLAYMUS`: If music is on, plays one step on each of two music channels, keeping each channel's pointer and delay between calls.
- `write_two_psg_registers`: Calls PSGW twice and returns.
- `PLAY_CHANNEL`: Plays one step of a music channel.
- `PLAY2`: Follows a note pointer and sets the channel A or channel B flag from the note's second word.
- `activate_channel_b_note`: Sets the channel B flag to minus one and writes three PSG registers.
- `clear_note_flags_on_channel`: Writes one PSG register, clears the channel B flag, and when the channel selector is zero also writes a register and clears the channel A flag.
- `START`: Allocates memory, starts the sound system, sets up the screen buffers and reads the picture data, then waits in a counted delay loop.
- `READALL`: Calls READPIX three times and RSTUFF_BUFF between those calls, which together read the background and object picture files.
- `MYINT3`: Calls TINTERRUPT, clears the interrupt request flag and writes the interrupt acknowledge mask to the global value.
- `READPIX`: Invokes three routines in sequence: a decode routine, a copy routine, and a colour copy routine.
- `RNDER`: Updates three random-generator state words using additions, a rotate and an exclusive-or with a fixed constant.
- `RNDER2`: Calls RNDER to advance the random generator and returns the new state value.
- `SWAPSCREEN`: Waits for the blitter to become idle, then enters an endless loop.
- `GOERROR`: Error handler that copies the saved register values to a stack area, copies a palette, clears the first screen buffer and writes the register names and their values as text.
- `BLOOP`: Contains only an endless loop that branches to itself, so the program stops at this point.
- `OPEIGHT`: Calls OPFOUR twice, which emits eight items in two groups of four.
- `OPFOUR`: Repeats a value output step and a text output step while a counter is nonzero.
- `OPHEX`: Calls OPITEM repeatedly while a counter is nonzero, outputting one character item per pass.
- `OPTEXTZ`: Calls OPITEM for each character of a zero-terminated text until the terminator is reached, then returns the register value.
- `emit_remaining_text_items`: Calls OPITEM repeatedly until the text pointer reaches a zero byte.
- `emit_text_items_then_restart`: Calls OPITEM repeatedly until the text pointer reaches a zero byte, then tail-branches into OPTEXTZ.
- `return_without_output`: Returns immediately with no work.
- `OPITEM`: Copies one eight-row glyph from a font table into four bitplanes of the screen, moving one 40-byte column per row.
- `return_unchanged_value`: Returns without doing any work.
- `ILLEG`: Handles an illegal instruction exception: it stores the register values to a stack area, copies the palette, clears the first screen buffer, then branches into GOERROR to show the register dump.
- `SETUPZ`: Stores the address of ILLEG into three global vector slots, which installs the illegal-instruction handler.
- `RSPRITER`: Processes glyph data to render a sprite by combining bit planes with rotation and masking operations, then calls another routine to continue processing.
- `or_glyph_rows_into_planes`: ORs source words into four bitplanes row by row, advancing the source by four words per pass.
- `RSAVE_BUFF`: Copies screen words from four bitplanes into a buffer, storing four words per entry, over a block of rows.
- `RDRAW_BUFF`: Writes buffer words back into four bitplanes, splitting four words per entry, over the same row block and 20-word row spacing as the matching save routine.
- `RSTUFF_BUFF`: Writes a header into the buffer holding a size word, a column count plus one and a row count.
- `RSTUFF_BUFF_ST`: Writes a size, column and row header into the buffer, then copies a rectangle of words from the four screen planes using a wider row stride than the matching store routine.
- `RSTUFF_BUFF_SHORT`: Writes a size, column and row header into the buffer, then copies a rectangle of words from one screen plane without the four-plane interleaving used by the longer variants.
- `RSPRITER_AM`: Sets the blitter pointers, modulo, masks and control registers from the sprite's position and size, then waits for the blit to finish.
- `MINTERMS`: Sets a bit in a mask byte and builds the palette, then clears both screen buffers and swaps them.
- `TITLES`: Builds the palette, clears both screen buffers and swaps them.
- `TINTERRUPT`: Increments a tick counter, and once every five ticks calls ALTER_PALETTE to step the palette fade.
- `reset_palette_fade_timer`: Sets the palette fade timer to four and calls ALTER_PALETTE. TINTERRUPT branches here when its timer reaches zero.
- `MAKE_PALETTE_D0`: Stores the input value as the fade value, then fills sixteen colour words from the palette table at that step.
- `ALTER_PALETTE`: When a fade is active, moves the fade value up or down by one.
- `increment_fade_and_rebuild`: Adds one to the fade value, clears the fade flag at the upper limit of 21, and rebuilds the palette.
- `FADE_IN`: Copies 8000 longwords from the back buffer to the second screen buffer, then calls SWAPSCREEN.
- `INTRO_SEQUENCE`: Resets the animation counters and sub-sprite state, then queues a sound.
- `MOVE_SUB`: Calls RSPRITER_AM twice and returns.
- `DO_PENGUIN`: Calls ONE_PENGY four times in sequence and returns.
- `ONE_PENGY`: Moves one penguin sprite by its velocity and reverses its direction at two horizontal limits.
- `RSPRITER_LIMITED`: Calls RSPRITER_AM and returns its register value.
- `PA`: It toggles one bit of a byte pointed to by a register, then subtracts a constant from a short value at that address three times, toggling the same bit again at the end.
- `NEW_COUNTER`: Pseudo-code could not be produced, so only the listing facts are known: a single instruction whose address INTRO_SEQUENCE uses in an exclusive-or and a test.

### USA0

- `enter_program_stub`: Pseudo-code could not be produced.

### Spy vs Spy (1989)(First Star Software)[cr Yakoul].adf:spy2

- `START_CODE`: Allocates memory, initialises the key matrix, starts the sound system and saves the current interrupt pointer.
- `WAIT_FOR_RASTER`: Waits for the vertical retrace by polling the raster position register until it reaches a specific value, then checks if it exceeds another value before returning.
- `ATARI_COPY`: Loops over 4000 words, taking four consecutive source words and writing each one to a different bitplane, the planes being spaced 4000 words apart.
- `COLOUR_COPY`: Copies sixteen 16-bit values from a source pointer into a global table, doubling each value on the way.
- `LOAD_FILE`: Loads the DOS library via the system base and stores the returned library base in a global, then invokes two further library calls on that base.
- `SETUP_SCREEN`: Sets the screen buffer pointers, fills an eight-entry table with two fixed words, and copies 55 words of plane pointer data into the second copper list.
- `WHICH_PLANES`: Fills four entries of a pointer table, each storing a 32-bit address split into high and low 16-bit halves, with each address 8000 bytes after the previous one.
- `BLIT`: Calls a helper, then writes its return value and several register inputs, including two all-ones words, into consecutive global slots before returning.
- `WAIT_BLIT`: Waits in a loop until a bit in the interrupt status register clears, then returns.
- `SWAP_SCREEN`: Waits for the vertical blank and then enters an infinite loop that never returns, using the copper to switch between two screen base addresses.
- `CLEAR_SCREEN`: It writes four consecutive zero 32-bit words to the memory block pointed to by A0, then advances the pointer by four longs, repeating this until 2000 groups have been cleared.
- `MYINT2`: Runs the keyboard interrupt: it updates the key matrix through ADD_MATRIX, waits in a short delay loop, clears one control bit and writes the interrupt acknowledge value.
- `ADD_MATRIX`: Inverts and shifts a keyboard code, then maps it to a matrix byte and bit.
- `DOWN_STROKE`: Clears one bit in the key matrix at the offset held in a register and returns.
- `INIT_KEY`: Sets four key matrix words to all ones, so that no key reads as pressed.
- `INKEY`: Returns minus one when the key's bit in the key matrix is clear, and zero when it is set.
- `SCAN_JOY`: Reads the fire button and direction bits from the joystick and mouse port registers into state words for ports 0 and 1.
- `HANDLE_MOUSE`: Calls DO_MOUSE twice to update the mouse movement, then sets two last-mouse state words to minus one when their button bits read clear.
- `DO_MOUSE`: Reads a four-byte counter from a state record and steps it toward zero one byte at a time, rotating the byte order between steps.
- `ALLOCATE_MEMORY`: Finds three available 32K memory blocks and stores each one into separate global variables that hold the single-screen, double-screen, and background buffers.
- `CLEARD0`: Zeroes 8000 longwords starting at the input pointer, which clears one 32000-byte screen block.
- `FIND32K`: Calls the system allocation function for a 32K block and returns when it succeeds.
- `DECO_PIC`: It copies sixteen 16-bit values into a destination table, then decodes three 8000-byte bitplanes from a stream of literal bytes and marker-introduced runs, writing each byte with interleaved spacing.
- `jump_to_test_3`: Jump stub: passes control straight on to TEST.
- `TEST`: It triggers a sound effect, then repeatedly plays music notes while waiting for a status flag to flip on and off.
- `START_SOUNDS`: Fills an 8192-byte buffer with values from a shift-and-add generator, sets the length registers of four audio channels to 32 words, and writes per-channel records with pointers to that buffer, then clears interrupt and DMA state and initializes the music.
- `GO_SOUND`: It stores a 32-bit pointer and two 16-bit values into a two-part record, then zeroes the counters and sets a flag to one.
- `HIGH_PING`: Sound handler whose body only calls GO_SOUND and returns.
- `MEDIUM_PING`: This routine invokes GO_SOUND and then performs a direct recursive call to itself before returning.
- `LOW_PING`: It invokes a sound routine and then returns.
- `DROP_PING`: This routine performs an indirect call by loading a function pointer from a data structure and jumping to it, serving as a dispatch mechanism for a handler stored in a data table.
- `TAKE_PING`: This routine calls a sound routine and then returns.
- `MEDIUM_BEEP`: It invokes the sound routine and then returns.
- `BANG`: It invokes a routine to initialize a channel record with a pointer, two values, and a flag set to one.
- `OTHER_SMACK`: Plays a sound effect by invoking the audio routine.
- `GRAVESOUND`: It invokes a sound routine and then returns.
- `BOOM`: Triggers a sound effect by calling the sound routine and then returns.
- `BURY`: Invokes the sound routine to produce an audio effect.
- `SPLASH`: It invokes a sound routine and then returns without altering any data.
- `RUMBLE`: Sound handler whose body only calls GO_SOUND and returns.
- `FEET`: It invokes a sound routine and then returns without altering any data.
- `KISS`: Sound handler whose body only calls GO_SOUND and returns.
- `SUB`: Calls a sound routine and returns without performing any other action.
- `HANDLE_SOUNDS`: Calls HANDLE_REQUEST up to four times in sequence, moving to the next call only while the zero flag from the previous one is set.
- `HANDLE_REQUEST`: Takes the first non-zero of four pending request words, or returns an error value if all four are zero.
- `NEW_SOUND`: Matches a sound request's ID against four stored request slots and marks the matching slot, or updates the stored current value when the flag bit is clear.
- `MYINT6`: Polls the mouse, advances the music delay counter and plays the next music step, updates each active volume envelope, processes queued sound requests, and then clears the interrupt request.
- `CHANGE_ENVELOPE`: Steps a voice's volume envelope by reading entries from the sequence its record points to.
- `X8912`: Selects a wave table, wave length and frequency for each of the first three audio channels from lookup tables, or a noise area when the mode bits say so.
- `PSGW`: Stores the low byte of the second data register into the music buffer at an offset two past the index held in the first register.
- `INITMUSC`: Sets register 7 to 56, sends three register writes through PSGW, and calls VOICEISOF twice.
- `PLAYMUS`: Selects channel 0, advances its voice and sends it out through OUTVC1, then does the same for channel 1 through OUTVC2.
- `VOICEISOF`: Clears the voice's gate, then scans its sequence table for the next entry and stops when the index passes the end marker.
- `NEXTSEQ`: Advances a voice's position in its sequence table and reads the next entry's duration, offset value and level.
- `VOICISON`: Counts down a voice's note timer and, when it expires, advances to the next note.
- `DEGATEIT`: Clears the gate byte at offset 13 of the voice record.
- `CHECK2`: Picks register 7 settings for the current channel from the channel selector and two status bits in the voice record.
- `CALCNOTE`: Calls NOTEMODS to get a modified note byte and takes its high nibble.
- `OUTVC1`: Forwards to SNDOUT to write the sound registers for the first voice.
- `OUTVC2`: Forwards to SNDOUT to write the sound registers for the second voice.
- `SNDOUT`: Sends three register bytes through PSGW in a row, with no arguments visible in the decompile.
- `NOTEMODS`: Adjusts a note byte by a small signed offset taken from a second input, carrying into the high nibble when the low nibble wraps.
- `RESET`: Calls the register write routine PSGW three times in a row.
- `START`: Top-level start routine that allocates memory, sets up keys, sounds and screens, then calls READALL and PLAYINIT before entering DO_GAME. It swaps screens and installs the interrupt handlers MYINT2, MYINT3 and SAVE_INT2 along the way.
- `READALL`: Clears the first screen buffer, then makes calls to READPIX and long runs of GRAB and RSAVE_BUFF that copy picture data.
- `MYINT3`: Increments a global counter, clears a busy flag, and resets another global variable before returning from the interrupt.
- `GRAB`: Calls RSAVE_BUFF four times in a row, counting a register down through four iterations.
- `GRABOBJ`: Calls RSTUFF_BUFF in a loop of 20 iterations.
- `GRABOBJ1`: Calls RSTUFF_BUFF repeatedly, counting down a register it receives on entry until the count goes below zero.
- `GRABLAND`: Calls RSTUFF_BUFF_SHORT in a loop of 20 iterations.
- `GRABMAP`: Calls RSAVE_BUFF repeatedly, counting down a register it receives on entry until the count goes below zero.
- `PLAYINIT`: Sets the menu and flash state variables for both players and the refresh counter.
- `PLAYINIT0`: Sets the automatic-control flags for the two players from the player count, then places both players' starting positions from the map table for the current level.
- `GETA4`: Loads the address of a global variable into register A4 and returns without performing any further action.
- `XGET_STICK`: Uses the current level to index two position tables, copying the X and Y entries into the XSTICK1 and YSTICK1 variables.
- `PARMS`: Menu and setup loop that reads trigger and joystick input and draws the selection.
- `DO_GAME`: Main game-frame routine: it clears the outcome timers, updates the busy flags and window positions, and moves and draws the spy.
- `READPIX`: Runs a picture decode and copy routine, then sets two global variables to zero and a large value before returning.
- `STUFFIT`: Repeats until its count register reaches zero.
- `STUFFSPY`: Picks a random position for each spy and converts map positions to window coordinates.
- `SETUP_VOLCANO`: Stores three fixed values, 1971, 1424 and 1397, into COL1A, COL2A and COL3A and mirrors them into their second copies.
- `COPY_COLOURS`: Copies four colour values into global variables, choosing between two sets depending on whether the incoming D0 register value is even or odd.
- `RNDER`: Calls GET_RANDOM once and returns.
- `RNDER2`: Calls GET_RANDOM and returns its result as a 64-bit value combined with the D0 half.
- `GETRAND`: Repeatedly picks random slab coordinates from RNDER calls until the chosen cell reads as zero, then returns a pointer to that cell.
- `SETXY`: Adds a fixed offset to its input, then divides by 96 to store the quotient as X and keeps the remainder as Y. It is called from BUSY, BUSY4_A, PLAYINIT0 and STUFFSPY.
- `CLEAROUTCRAP`: Sets six variables for the time and fuel readouts to minus one.
- `DRAWSPY`: Draws the spy sprite for each player at its window position when that player's map cell matches the current window, then calls DRAWHANDS for the hands.
- `DRAWHANDS`: Draws a hand sprite for a player when that player's counter, depth and swamp values are within limits.
- `DRAWBUTTONS`: Calls ONEBUTTON six times in a row, one for each button position.
- `ONEBUTTON`: Draws one item button sprite on the second screen, using the ITEMON buffer when the flag is set and ITEMOFF otherwise.
- `SWAPSCREEN`: Waits for the blitter to finish, then jumps to SWAP_SCREEN. The decompile shows an endless loop after the wait, which is the jump shown as a loop and not real behaviour.
- `DRAWTIME`: Calls COMPARETWICE on each of the four time digits, ten minutes, one minute, ten seconds and one second.
- `HLINE`: Calls HORLINE to draw a horizontal line and returns.
- `METER`: Restores one unit of energy on a timer when the player's state allows it, and sets the refresh flag.
- `FLASH`: Chooses between two drawing paths based on the counter's low bit and the upper bits of the chosen player's hand value.
- `DRAWPIECE`: Sets the minterms to 2544, draws a sprite with RSPRITER_AM, then sets them to 4066.
- `DRAWSEL`: When the selection flag is set, draws the four selection characters with PUTCHAR. The selected index comes from a counter bit, and the matching character is skipped.
- `COMPARETWICE`: Compares its input with a stored word and, when they match, also checks the next word.
- `RSPRITER`: Draws a sprite into the screen buffer one row at a time, with the row and column offsets taken from the input registers and the sprite data read from the buffer address.
- `RBACKER`: Draws rows of background data into the screen buffer at a position given by the input registers, reading the source through a pointer.
- `copy_four_plane_words`: Copies four words per iteration from the source into the screen buffer at offsets 4000 bytes apart, one per plane.
- `RSAVE_BUFF`: Copies a block of rows from the screen buffer into a save buffer, using row and column offsets from the input registers.
- `RDRAW_BUFF`: Copies rows from the buffer into the screen at a position given by the input registers.
- `RSTUFF_BUFF`: Computes a block size as the product of two input counts and stores it doubled in the first buffer words.
- `RSTUFF_BUFF_SHORT`: Computes a block size as the product of two input counts and stores it doubled in the first buffer words, using the short-word form of the fill.
- `RSPRITER_AM`: Sets a global to 33856, then programs the blitter's source, destination, modulo and size registers for a sprite copy.
- `SPRITER`: Calls RSPRITER to draw a sprite and returns.
- `SPRITER_AM`: Calls RSPRITER_AM to blit a sprite and returns.
- `BACKER`: Calls RBACKER to draw the background rows and returns.
- `MINTERMS`: The recorded length is 1 instruction, and the body sets one bit in a byte and then calls NEXTCHAR. MINTERMS is used as a plain data variable by its callers, so the body is not this entry's own code.
- `PUTCHAR`: Calls NEXTCHAR to draw one character and returns.
- `NEXTCHAR`: Draws one eight-row character from the font table, chosen by the low four bits of the input.
- `CALCAD`: The routine executes fifteen instructions but performs no functional operations or data manipulations before returning.
- `PLOTPOINT`: Clears the masked bits and then sets the new bits in four words of the screen buffer, one word per plane, 4000 bytes apart.
- `LEFT_RIGHT`: Tests the go-left flag and the low bit of an input, but both branches return with no change shown.
- `UP_DOWN`: Reads the go-left flag, but both branches return with no change shown.
- `DRAW_LINE`: Works out the horizontal and vertical distances between two points and sets the direction flag.
- `HORLINE`: Draws a horizontal run of points between two x positions, calling POINT for each group of pixels along the way.
- `POINT`: Joins the high word of its first input with the low word of its second input, then masks the result to keep the high word and the low four bits.
- `GET_RANDOM`: Updates three random-state words with adds, subtracts and a bit rotation, then returns the new value through its registers.
- `READ_TRIGS`: Scans the joystick, then sets the fire-trigger flags for player one from the keyboard or fire inputs, depending on the player's mode.
- `MOVE`: Moves a spy each frame, choosing its direction from the joystick, the computer player's routine or a random move.
- `set_spy_direction_four_2`: Sets the spy direction to 4 and stores 2 through the pointer register, then returns.
- `set_spy_direction_three_2`: Sets the spy direction to 3 and stores 1 through the pointer register.
- `CHECKMEET`: Sets each player's window row from the map row, then compares the two windows.
- `CHECKMEETEND`: When its count input is above one, sets BUF1Y and BUF2Y to 65535, then returns.
- `SETTEMPS`: Copies the window and map position of player one or player two into the shared spy position variables.
- `DIRFIX`: When its input is nonzero, chooses the player two frame buffer from the spy direction value.
- `TEST_OPTIONS`: Counts down the option timer on each call, and when it is already zero calls TEST_M to handle the option check.
- `TEST_M`: Calls INKEY, and when the result is false resets the option timer to 20.
- `JOYMOVE`: Scans the joystick and keyboard, waiting in loops with a short delay until the keyboard check returns true.
- `SETWINDOW`: Copies the player two window position into the spy window variables when SPYWIN is 2 or the input is nonzero (and SPYWIN is not 1).
- `GETWINDOW`: Copies the spy window position back into the player two or player one window variables, using the same choice rule as set_spy_window_position.
- `LITTLE_DELAY`: Loops counting a 16-bit variable down through a long wrap-around count, then returns its input unchanged.
- `DRAWMOVE`: Draws one movement frame: the fuel readout, the flashing piece, the selection, the land and the objects.
- `DRAWLAND`: Reads the buffer edge positions for the two players and selects the matching back buffer.
- `DRAWLAND2_0`: Starts a blit using the mode word 33856, waits for it, and draws the land slabs.
- `DRAWLAND3_0`: Starts a blit using the mode word 33856, waits for it, and draws the land slabs.
- `ONE_SLAB`: Computes a slab address from a row index scaled by 520 and programs the blitter for a single slab, with masks and sizes.
- `DRAWSLAB`: Draws one slab with blit_one_slab, and draws it a second time when the slab index is zero or inside a range and the address is aligned to eight bytes.
- `DRAWBACK`: Copies the object buffer into the selected back buffer, starting at an offset from the X position and using the width value.
- `DRAWOBJ`: Draws objects by setting the buffer, screen and width values and calling SPRITER_AM for each one.
- `DRAWLAND5_2`: Draws the RTCOVER sprite at a fixed position on the second screen.
- `DRAWROPES`: Calls DRAW_LINE for each player whose rope-line flag is set, and clears that flag after drawing.
- `DRAWFUEL`: Compares the chosen player's fuel value with its last value through COMPARETWICE. It returns early when the fuel value is zero, and otherwise draws the gauge with HORLINE. It is called from DRAWMOVE.
- `CLEAR_WINDOW`: Programs the blitter to clear a window region of the second screen, then clears a second region 8000 bytes further on.
- `BUSY`: Large per-frame routine for the two players.
- `BUSY4_A`: Chooses a player's map position and spy position, then sets the state and calls SETXY and STUFFIT to place the objects.
- `DELETE_TRAP`: Scans a list of 100 trap entries for the given pointer.
- `ADD_TRAP`: Adds a trap entry when its input byte is nonzero, choosing the player's automatic flag by the input.
- `clear_player_delay_state`: Clears both flash counters, moves the pending DELAY1 value into DELAY0 or DELAY1 according to the input, and clears the matching player's counter.
- `BUSY6_4`: Moves the pending DELAY1 value into DELAY0 or DELAY1 according to the input, and clears the matching player's counter.
- `SETSAFE`: Sets the safe value for player one or player two to 4, chosen by whether the input is zero.
- `KILLDOUBLE`: Sets the time marker for player one or player two to the counter minus 20, chosen by whether the input is zero.
- `GETPOS`: Small routine of 2 instructions that shifts d1 right by 6, then shifts d2 right by 4.
- `GETPOS1`: Sets the X position to the first input times 16 plus 114, and stores a Y position from the second input plus eight times the third.
- `SETSTATE`: Moves the pending DELAY1 value into DELAY0 or DELAY1 according to the input, then sets the matching player's counter to the value in its register.
- `GETBTIME`: It reads a word from a global variable and stores it back to the same location, effectively copying the value into register D0.
- `SHOWMAP`: Draws the map for the current level as a sprite at a fixed position on the second screen.
- `ABSD2`: Contains no instructions and simply returns to the caller without modifying any data.
- `DRAWROPE`: Chooses the plot position of player one or player two according to its input, then draws the rope segment from that point.
- `CLEANTRAIL`: Sets each of the six entries in the two trail arrays to 65535.
- `SUBCLOUDS`: Draws a cloud sprite at a position taken from its inputs, doubling the x offset when the second input is above six.
- `COPYBACK`: Copies the first screen buffer into the second, several words per pass, over about 2000 passes.
- `PARADROP`: Computes a count from the demo value, and when it exceeds one draws the sprite twice with RSPRITER. It then runs a short delay loop.
- `GETMODE`: Mode-selection routine that clears the trails, draws the item list, and draws the sprites for both players.
- `DRAWITEMS`: Loops over the menu items, setting the x limits PX1 and PX2 for each and drawing a line for it with DRAWLINE. It is called from GETMODE.
- `SELECTITEM`: The body is a bare return with no work, so the routine does nothing.
- `DRAWLINE`: Calls DRAW_LINE to draw one line and returns.
- `RSPRITERBOTH`: Calls RSPRITER to draw a sprite, then branches into RSPRITER again with the second screen's values.
- `RDRAW_BOTH`: Calls RDRAW_BUFF to draw the buffer, then branches into RDRAW_BUFF again for the second screen.
- `RCLEARBLOCK`: Clears a block of bytes in the screen buffer, starting at an offset from the input registers and running for a count taken from the input.
- `BRAINMOVE`: Chooses the next move for the computer-controlled spy from random values, the stick positions in XSTICK1 and YSTICK1, and a mound search.
- `GETD0D1`: Sets the spy direction from the first set bit of its input, checking bits 0 to 3 in turn for directions 3, 4, 1 and 2.
- `BRAINBUSY1`: When the word at the pointer is zero, clears the frame index of player one or player two according to the input, then sets the word to 16389.
- `reset_player_two_frame`: Clears the player two frame index and sets the word at the pointer to 16389.
- `RANDMOVE`: Uses random bits from RNDER to pick a step with x and y changes of minus one, zero or one.
- `FIND_A_MOUND`: Searches neighbouring map cells through two position pointers, reading a byte table for each cell.

### Spy vs Spy (1989)(First Star Software)[cr Yakoul].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `compute_bootblock_checksum`: Stores a fixed value at offset 42, sums 23 words starting at offset 34, and stores the complement of the sum at offset 82.
