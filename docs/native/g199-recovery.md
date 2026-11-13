# Game 199 (ProSoccer 2190): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00199). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905680 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 309 of 309 named (276 author symbols), 309 of 309 described (22 by rule, 27 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3151 of 3156 (99.84%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8366 of 8374 instructions identical in start and length, 2 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 262608 |
| code | 42792 |
| data | 142080 |
| padding | 1496 |
| unused | 456704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| ProSoccer 2190.adf:SKR_VAR | 1 | 154 | 1 |
| ProSoccer 2190.adf:prosoccer | 6 | 7887 | 279 |
| ProSoccer 2190.adf:echo | 2 | 117 | 2 |
| ProSoccer 2190.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 58 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 138 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |

## Cases handled

- ProSoccer 2190/ProSoccer 2190.adf:LOGO_VVP.CPX: Exploder Style Data File (xfd)-packed, unpacked with ancient (4216 -> 13080 bytes)
- 30 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ProSoccer 2190/ProSoccer 2190.adf:SKR_VAR: TryIt Cruncher 1.01 (xfd) stream found by ancient scan (14596 -> 23508 bytes) but only 0 of 8 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate, 1 jump target
- ProSoccer 2190/ProSoccer 2190.adf:SKR_VAR: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### ProSoccer 2190.adf:SKR_VAR

- `decompress_rle_bitplane_to_chipram`: Reads a packed byte stream through a5, expands run-length and back-reference tokens into a destination buffer, writes bitplane bytes backwards, and flashes COLOR00 with the bit counter while copying.

### ProSoccer 2190.adf:prosoccer

- `main`: Calls init_os, set_palette, seed, cls and palette_st_to_amiga, then jumps to pro_start to begin the game's season loop.
- `read_kb`: Loads the keyboard hardware register at absolute address 548 into d0 and falls through to the following routine.
- `clear_kb`: Writes zero to the last key variable at absolute address 548, clearing the stored key, then returns to the caller.
- `init_os`: Opens intuition, graphics and dos libraries, stores their bases, reads the sync rate, adds the level 5 interrupt server, sets the vertical blank flag and calls the input, screen and fx interrupt initialisers.
- `reset_os`: If the vertical blank flag is set, removes the level 5 interrupt server and clears the flag, then calls the input and screen reset routines and closes the intuition and graphics libraries.
- `vertb`: Saves all registers, calls set_current_screen and joy_vbl with a0 pointing at the custom chip base, conditionally calls ami_clock unless time_held equals one, calls check_debug, increments vblflag and vblcount, restores registers and returns zero in d0.
- `vsync`: Clears d0 and the word at address 126, then tests that word and spins with dbra until it is nonzero, returning once the vertical blank flag has been set.
- `ami_clock`: Increments the jiffy byte, rolls it to zero at fifty and increments the seconds word, rolling it at sixty and incrementing the minutes word, then returns.
- `ed_ints`: Empty routine: returns to the caller immediately without doing anything.
- `init_mem`: Sets a global word to zero, calls a library routine to allocate 32704 bytes of memory, stores the resulting pointer in a global long, and sets a global word to one if the allocation succeeded.
- `reset_mem`: Checks the longword flag at address 606; if nonzero, calls exec FreeMem through a6 with a1 pointing at the pointer at 602 and size 32128, then restores a6 and returns.
- `getascii`: Masks d0 to seven bits, maps key codes 68, 65 and 70 to control values 13, 8 and 127, otherwise indexes chartab and returns the byte in d1, turning 63 into zero.
- `init_screen`: Calls intuition CloseWorkBench through a6, copies GfxBase field at offset 50 into global old_clist, then calls set_screen with a0 zeroed and returns.
- `reset_screen`: Calls intuition OpenWorkBench then RemakeDisplay through a6 using IntuitionBase, refreshing the display and returning to reset_os.
- `swap_screen`: Swaps the current and display screen pointers, toggles the screen number byte, sets the pending-swap word to one and clears the vbl flag word.
- `set_current_screen`: Tests the pending-swap word; if nonzero it clears it and loads the display screen pointer into a0, otherwise it returns immediately.
- `set_screen`: Writes four pairs of high and low words from a0 into the copper list, advancing a0 by 8000 each time, then stores the list pointer into GfxBase offsets 50 and 54.
- `set_palette`: Copies sixteen words from the memory pointed to by a0 into the palette area at a fixed low address, writing each word twice forty bytes apart, then returns.
- `blit_rect`: Empty routine: returns to the caller immediately without doing anything.
- `bm2`: Empty routine: returns to the caller immediately without doing anything.
- `fill_bitmap`: Empty routine: returns to the caller immediately without doing anything.
- `fh0`: Empty routine: returns to the caller immediately without doing anything.
- `fh1`: Empty routine: returns to the caller immediately without doing anything.
- `init_input`: Creates the input message port and standard IO request, opens input.device, stores the interrupt handler pointer and length in the IO block, and starts the input request.
- `reset_input`: If an input IO request exists, restores the interrupt handler, sends the abort command, closes input.device, deletes the standard IO request and the input message port.
- `input_handler`: Reads the event type byte at offset four from a0; type two branches to raw_mj, type one with a non-negative key word stores that key and branches to soak_event, otherwise returns via ret_event.
- `check_debug`: Reads the debug key word, dispatches on key codes to bring a screen to front or rebuild the copper list from bitmap or screen parameters, then returns.
- `CreatePort`: Allocates a signal and message port memory, fills port fields from d0 and a0, links the port list, and returns the port pointer in d0.
- `DeletePort`: If a0 is non-null, frees its signal bit, marks the port bytes at offsets 8 and 20 as invalid, then frees the port memory via exec.RemPort and returns.
- `CreateStdIO`: Allocates 48 bytes of exec memory for a message port, stores the caller's a0 pointer at offset 14, sets the port size at offset 18 and type byte 5 at offset 8, returning the port in a0.
- `DeleteStdIO`: If a0 is non-null, marks the port invalid at offsets 8, 20 and 24, then frees its memory with exec.FreeMem using the size stored at offset 18.
- `begin`: Calls reset_os, closes DOS library if open, forbids multitasking, replies the startup message when present, and returns the reply result.
- `toggle`: Pushes every data and address register, calls swap_screen then vsync, restores all registers and returns to the caller.
- `loop1`: Empty routine: returns to the caller immediately without doing anything.
- `blitsub`: Empty routine: returns to the caller immediately without doing anything.
- `waitblit`: Reads the blitter status byte at offset two from a1 and loops while bit six is set, returning once the blitter is idle.
- `joy_vbl`: Clears click and both what words, tests CIA fire bits and player/kickmeter/possession state to set click flags, then reads JOY0DAT and JOY1DAT to set direction bits in what and what+2.
- `init_mouse`: Empty routine: returns to the caller immediately without doing anything.
- `x_done`: Empty routine: returns to the caller immediately without doing anything.
- `exit_routine`: Empty routine: returns to the caller immediately without doing anything.
- `seed`: Copies the caller's return address from the top of the stack into the longword at absolute address 2350, then falls through to following code.
- `random`: Saves all registers, seeds the random_w longword from memory if zero, multiplies its low word by a fixed address, adds one, masks to nine bits, stores it back, restores registers and returns the value in d0.
- `load_file`: Saves all registers, clears the load error word, opens the filename with mode 1005, reads file_length bytes into destination_add, closes the handle, and jumps to error_handler on failure.
- `save_file`: Saves all registers, clears the load error word, opens the named file with mode 1006, writes the file length bytes from the destination buffer, closes the handle and restores registers; on failure it branches to the error handler.
- `palette_st_to_amiga`: Reads sixteen colour words from a0, masks and shifts each into Amiga 12-bit format, stores them in the palette buffer, then copies the palette to the hardware colour registers.
- `set_fx_interrupt`: Stores the address of snd_vector into the level 4 interrupt autovector at memory location 112, then returns to the caller.
- `snd_vector`: Saves d0 and a0, reads INTREQR, acknowledges the four sound interrupt bits in INTREQ, and branches to the matching sound channel handler before restoring registers and returning from the exception.
- `sv_s0`: Checks the channel 0 sound counter; if not positive, enables the level 2 interrupt in INTENA, clears the counter and acknowledges INTREQ, then branches back to the sound vector.
- `sv_s1`: Checks the channel 1 sound counter; if not positive, disables the disk DMA bit in DMACON, enables the level 3 interrupt in INTENA, clears the counter and acknowledges INTREQ before branching to the sound vector.
- `sv_s2`: Checks the channel 2 sound counter; if not positive, enables the level 4 interrupt in INTENA, clears the counter and acknowledges INTREQ, then branches back to the sound vector.
- `sv_s3`: Compares the longword sound counter at memory location 822 with zero; if greater, branches to the sound vector, otherwise clears bit 3 of DMACON, enables the interrupt at INTENA bit 10, clears the counter and acknowledges INTREQ bit 10.
- `palette_fadein`: Stores the target palette pointer, clears the temporary palette, then repeatedly masks the target colours with 444, 555 and 666 and finally copies them, calling palette, slowfade and holdfade between steps.
- `slowfade`: Loads d7 with 1000, d6 with 22 and d5 with 7, divides d5 by d6, then counts d7 down to zero before returning.
- `holdfade`: Loads d7 with 10000, d6 with 22 and d5 with 7, divides d5 by d6, then counts d7 down to zero before returning.
- `palette_fadeout`: Copies the target palette into temp_palette, then repeatedly masks each of sixteen colours with decreasing bit patterns, calling palette and slowfade between steps, and finally zeroes the palette and calls holdfade.
- `palette`: Saves all registers, loads the temp_palette address into a0, calls set_palette to install it, restores registers and returns.
- `de_crunch`: Saves all registers, loads source, destination and length from fixed memory into a1, a0 and d0, calls decompress, restores registers and returns.
- `new_pal`: Saves all data and address registers on the stack, moves a6 into a0, calls set_palette, restores the registers and returns.
- `initialize_game_state`: Performs several bitwise operations on global variables, clears the keyboard, clears the screen, calls multiple routines including a file loader and music starter, and enters a loop that plays seasons while incrementing a global counter.
- `pro_start`: Clears keyboard and screen, toggles display, loads the music file into sounds, sets palette, starts music, shows opening, then loops calling play_season and incrementing the season counter.
- `play_season`: Repeatedly plays weeks and prints the league until week reaches 22, clears the results area, sets up the final and European league matches, then plays or auto-resolves the final and runs end-of-year updates and trades.
- `play_week`: Clears results unless continuing, starts music, shows the fixture screen until a click or timer expires, then loops through matches calling setup, play or auto-result, league update and storage transfer.
- `play_`: Sets possession, shows fullscreen for non-A teams, calls setup_match, plays crowd effects, runs pre_match, releases the button, starts the match and stops sound effects.
- `terminate`: Call stub: calls clear_kb as a subroutine and returns to its own caller when it returns.
- `pro_palette`: Copies sixteen words from the source palette table into the palette buffer, then calls set_palette to install it in hardware.
- `VULTURE`: Clears the screen, runs the vulture animation routine, clears the screen again, then calls toggle before returning to the caller.
- `do_the_vulture`: Loads the vulture file into screen memory, decompresses it, clears the screen, runs the animation loop for one hundred sixty frames with palette fades, then clears and toggles.
- `vulture_it`: Calls the clear screen, vulture, video text, vulture text and product text routines in order, then calls an external routine and returns.
- `vance_the_vulture`: Sets image number, x and y globals, selects image one when the vulture time is between 120 and 130, then draws the image and returns.
- `video_text`: Sets image number and x, adds two to the vulture counter when time is between 40 and 88, sets y from 128 minus the counter, draws the image and returns.
- `vulture_text`: Sets image number 2 at x 6, adds 2 to vult_cnt when the vulture timer is between 40 and 88, sets image y to vult_cnt plus 32, then calls show_an_image.
- `prod_text`: Writes image x 9, y 155 and image number 4 to the display globals, then calls show_an_image only when the vulture timer exceeds 89.
- `show_an_image`: Computes a screen offset from image x and y, reads the image pointer and size from the image tables, then masks and ORs four bitplanes of the image into the current screen buffer.
- `decompress`: Reads a compressed stream backward from a0 plus d0, refilling bits via l16 and decoding literals and back-references via l17, writing bytes downward toward a1 and returning a status in d0.
- `l16`: Pops a longword from the stack pointed to by a0, XORs it into d5, then rotates d0 right through extend with the extend flag set, returning the shifted value.
- `l17`: Repeats d1 times: shifts d0 right, refills it from the stack via a0 and XORs into d5 when empty, and rotates the carry into d2, then returns.
- `medium_delay`: Calls small_delay nine times in a dbra loop using d2 as the counter, then returns to the caller.
- `small_delay`: Loads 10 into d0, calls tiny_delay repeatedly, decrementing d0 with dbra until it reaches -1, then returns.
- `tiny_delay`: Loads 11000 into d1, executes two nops, then decrements d1 with dbra until it reaches -1, then returns.
- `prt_cls_back`: Writes 640 to the pitch x position, then calls draw_pitch_backdrop, draw_tv, draw_tvlights and display_logo before returning.
- `trade`: Iterates team index from zero to fifteen, sets trade team and long-term storage pointers, skips teams whose byte at offset fourteen is 'A', and calls is_trader, prt_cls_back and toggle for each other team.
- `calc_player_cost`: Loops 32 times over player records at a0, copying byte at offset 44 to offset 46 and subtracting the sum of a1 offset 30 word and a2 offset 44 byte, advancing a0 by 64 each time.
- `is_trader`: Resets offer, copies trade pointers to display pointers, shows trader, exits if selection exceeds fifteen, otherwise runs team_coach, updates display pointers for the chosen team, recalculates cost and redisplays until selection exceeds fifteen.
- `display_trader`: Redraws the trade panel with card names, stats and prices, then reads the selection and click flags to move the cursor, switch firsts/seconds, or set up a buy or sell offer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `end_of_year`: Loops over 512 player records, increments age, resets and randomly adjusts skill bytes, clamps them between limits, and stores a computed overall rating.
- `opening`: Initialises the open timer to 200 and ball flag to zero, then repeatedly clears the screen, draws the logo, sets sprite and card variables in the mid-timer window, calls bigman and toggle, decrements the timer, and returns when the timer hits zero or click is set.
- `display_logo`: Sets sprite and card variables to draw seven logo blocks via prt_mlogo, advances the spinball and spin pointers with wrap checks, draws the spinball sprite via prt_proman, draws two more logo blocks, and clears d1 through d4 before returning.
- `prt_mlogo`: Calls prt_logo repeatedly, adding 16 to the sprite x position and 1 to the sprite variable while decrementing the card loop counter until it reaches zero, then returns.
- `prt_mman`: Draws four body sprites then four leg sprites by calling prt_proman, advancing the sprite x position by 16 each time and reading sprite indices from the body and legs tables.
- `bigman`: Advances the big-man animation pointer, wraps it to bm_work on a terminator, then calls the matching animation routine and finally prt_mman to draw the player.
- `m_norm`: Loads the animation buffer address into a0 and writes the normal pose frame indices zero through three and eight through eleven into consecutive words.
- `m_blink`: Loads the address 9776 into a0, then writes the values 4 and 5 to offsets 2 and 4 from that base, and returns.
- `m_smile`: Loads the address 9776 into a0, then writes the values 6 and 7 to offsets 2 and 4 from that base, and returns.
- `m_handball`: Loads the address 9776 into a0, then writes the values 12 and 13 to offsets 12 and 14 from that base, and returns.
- `m_spinball`: Writes 10 and 15 to two sprite control words at offset 2630, increments the handspin counter, wraps it from 18 back to 16, and stores it in the sprite frame word.
- `m_roar`: Writes the eight consecutive frame numbers 22 through 29 into the eight sprite control words starting at offset 2630, setting up the roar animation.
- `prt_options`: Draws the options cards and text, highlights the selected option, then reads input flags to move the selection or launch save/load, team coach, or graphics redraw actions.
- `save_and_load`: Draws the save/load card, prints the title and four file choices, highlights the selected entry, handles up/down and click input, then jumps to load_it or save_it.
- `team_coach`: Draws the coach/team card, prints team names and coach type letters, lets the player move the selection and click, then branches to name editing, player editing or coach type change.
- `correct_fixtures`: Scans the fixtures table comparing each pair's second byte with d0, swapping the two bytes when they match, then returns after processing all entries.
- `edit_box`: Draws the name entry card and cursor, then loops reading keyboard characters to insert, backspace or delete letters in the edit buffer until return is pressed.
- `drag_back`: Shifts characters in the edit buffer one position left from the cursor to the edit length, then writes a space at the end.
- `redraw_gl`: Draws the slim card and prints the game length title and four choice lines, highlights the current choice in blue, toggles input, adjusts the stored length on up or down, and loops until a click selects it.
- `auto_result`: Computes home and away strengths from team data, picks a score from the result table, adjusts it with random rolls by game length, then allocates goals, shots and tackles for both teams and resets the game timer.
- `allocate_goals_2s`: Loops d7 times, picks a random player index up to nine, and increments that player's goal and assist counters in the team data block at a0.
- `allocate_shots`: Loops d7 times, picks a random player index up to nine, and increments that player's shot counter at offset 50 in the team data block at a0.
- `allocate_shots_forward`: Loops d7+1 times, calls random4, adds 6 to the result, scales by 64 and increments the byte at offset 50 of the a0-based record, then returns.
- `allocate_tackles`: Loops d7+1 times, calls random16 until the result is at most 10, scales it by 64 and increments the byte at offset 49 of the a0-based record, then returns.
- `allocate_tackles_back`: Loops d7+1 times, calls random8, scales the result by 64 and increments the byte at offset 49 of the a0-based record, then returns.
- `calc_team_strength`: Loops eleven times adding three word fields at offsets 12, 18 and 34 from a0, advancing a0 by 64 each time, then divides the total by 33 and masks to a byte in d0.
- `which_type_of_match`: Clears the player count and watch flag, then checks the home and away team control bytes to set the watch flag for computer teams or count human-controlled sides.
- `clear_season`: Loops over sixteen team records clearing bytes and words at offsets 15 to 22, then numbers eight team and eight euro league entries, then resets 512 long-term records copying byte 4 into word 28 and zeroing fields.
- `sd_random`: Calls random16 repeatedly to randomly raise speed, control and tackle bytes of 16 long-term records, clamping each to 9, using ten then twenty iterations per record.
- `update_league`: Writes home and away scores as four characters into the text buffer, adds goals to both teams' for/against totals, increments games played, and updates win, loss or draw counters.
- `setup_teams`: Uses week and game indices to locate the fixture, copies both teams' 14-byte names and palette words into globals, calls set_palette, then copies each team's 2048-byte long-term storage into the home and away buffers.
- `transfer_back_to_storage`: Copies 2048 bytes from the home match buffer to the home long-term storage pointer, then 2048 bytes from the away match buffer to the away long-term storage pointer, and returns.
- `b_w_fix`: Writes four fixed palette words into the palette structure at offsets 20, 22, 24 and 26, then calls set_palette to apply them and returns.
- `setup_match`: Loops over 64 stored home player records setting byte 27 to 100, zeroing several fields and copying byte 4 into word 28, then calls transfer_names and transfer_data for home and away squads.
- `transfer_data`: For 11 players, copies bytes from a0 record fields into a1 match slots: byte 0 to word 18, byte 1 to word 28, byte 3 to word 34, byte 4 to byte 46 and word 12, byte 27 to byte 51, clears long 52, advancing both pointers by 64.
- `transfer_names`: Copies 10 bytes of name from offset 16 of each of 11 home records into a1, then copies 10 bytes from each of 4 away records into a1 after a 10-byte gap, advancing source by 64 per record.
- `pre_match`: Resets runon timer, ball and player data, mirrors pitch positions, then loops drawing pitch, doors, players, TV and big door sprites until click or timer reaches 70.
- `prt_big_door`: Writes sprite x positions 136, 152, 168 and 184 with sprite numbers incremented, calls prt_doors for each, then repeats a second row offset by 32 in y.
- `start_match`: Initializes player and ball data, scores and timers, then loops updating movement, drawing, clock, goals, quarters and keyboard player-mode input until the game ends.
- `prt_fixtures`: Clears the button, draws top and rest cards, prints the week's fixtures with team names and results, draws two manager portraits, then loops on toggle and click to change the selected option or exit.
- `prt_best`: Displays four successive top-ten tables (defence, offence, goalscorers, MVPs) by drawing the background, printing headers and calling fill_hall, waiting for a toggle between each page.
- `fill_hall`: Clears a workspace buffer, then iterates through a long-term storage table to perform an insertion sort on entries based on a byte-fix condition.
- `insert_sort`: Reads a score from a0 into a1 offset 36, stores 511 minus d7 at offset 38, then bubble-sorts nine groups of eight 4-byte entries in work_hall by comparing words and swapping pairs.
- `prt_best_back`: Sets sprite variable to 3, calls draw_topcard and draw_restcard, loads the white font pointer into the font color global, points a0 at the hall of fame text with coordinates 20 and 22, calls text_printer, then returns.
- `prt_final`: Draws the final result card: clears sprite variable, draws cards, prints final and the text in white and blue, loops over the current week's fixtures printing team names and scores in black and red, optionally prints the winner message, prints the two finalist names, draws two player sprites and calls toggle.
- `waitxxff`: Checks the click byte at memory location 8cc; if it equals one, calls the release_button routine, otherwise returns immediately without touching anything else.
- `prt_league`: Calculates points, sorts both league tables, sets sprite variable to 3, draws cards, prints the league headings and column titles in white, blue and red, prints both league tables via find_prt_league, calls toggle, waits for a click and releases the button.
- `calc_pts`: Loops over 16 team records of 32 bytes, storing points as four times wins plus draws at offset 22 and goal difference as goals-for minus goals-against at offset 28.
- `sort_league`: Builds a sort table from 8 team records, bubble-sorts it by points then goal difference, then writes each team's rank into offset 23 of its record.
- `find_prt_league`: Scans 8 team records for those matching the current league position, prints their name and five numeric fields via text_printer, and stores a display offset at record offset 30.
- `find_league`: Stores a0 into the league pointer, clears the team index and match counter, then loads the first team's league byte and compares it with the index, branching away if unequal.
- `advance_league_team_index`: Increments the team index, checks it against eight, adds eight to d2 or advances the league pointer by thirty-two and increments the match counter, returning when that counter reaches eight.
- `end_of_game_update`: Loops over ten home and ten away player records, clearing the man-of-the-match flag and incrementing the stored count when set, then updates both stored data blocks.
- `update_sddata`: Loops sixteen 64-byte player records from a0, adding bytes at offsets 10, 8 and 6 into words at 34, 36 and 38, and incrementing byte 12 when the long at 30 is nonzero.
- `tr_man_to_sd`: Copies words and bytes from the manager record at a0 into the sd slot at a1, then clears the three bytes at offsets 48, 50 and 49 of the source record.
- `fullscreen_`: Draws the full-screen team card, prints names and stats, toggles selection highlighting, handles up/down/click input, and swaps player records when demoting or promoting.
- `video_capture`: Resets the film write pointer when past its end, then copies three words from each of twenty-two player records and three ball words into the film buffer and advances the pointer.
- `video_show`: Resets the film read pointer when past its end, then copies three words from the film buffer into each of twenty-two player records and three ball words, advancing the pointer.
- `video_`: Loops twenty times calling video_show, pitch, player and TV drawing routines, flashing the REPLAY text every second frame, toggling the screen and delaying before returning.
- `refresh_reserves`: Loops over five 64-byte player records in two tables, incrementing byte 27 up to 100 and calling calc_new_speed2 for each updated record.
- `calc_new_speed2`: Copies byte 4 of the player record into word 28, then subtracts one for each of three stamina thresholds (70, 50, 20) that byte 27 exceeds.
- `timeout_message`: Checks player count and possession, decrements the home or away timeout counter, then draws the small card and prints the timeout messages, team name and remaining count before toggling, delaying, displaying and dropping in.
- `player_mode_change`: Sets sprite variable to 3, draws the small card, prints the one or two player mode text in blue and black, then toggles and delays before returning.
- `show_quarter`: Draws the small card, prints the end-of-quarter or end-of-game message chosen by mvp_score, computes the MVP, prints the MVP label, name and team in blue, black and red, then toggles and delays twice.
- `calc_mvp`: Scans 22 player records, scoring each from three stat bytes, keeps the highest scorer's data, name and team pointers, and flags the winner when the top score is 5.
- `goal_message`: Draws the small card, prints the scoring player's name and team name in blue and black, picks a goal message from score, position and player bytes, prints it in red, then toggles and delays.
- `delay`: Loads 120 into d7, then repeatedly calls vsync and hold_time, decrementing d7 until it reaches minus one, then returns.
- `delay_short`: Loads 16 into d7, then repeatedly calls vsync and hold_time, decrementing d7 until it reaches minus one, then returns.
- `text_printer_c`: Scans the string at a0 for a null or double space, counts characters, halves and multiplies by six, then calls text_printer with d1 set to 160 minus that width.
- `display_timeout`: Draws the timeout player card, fills card data pointers from possession, prints names and stats, then loops on joystick input to move the selection and show the choices card.
- `release_button`: Reads the click byte at memory location 2252 and branches back to itself until it becomes zero, then returns to the caller.
- `rebase`: Draws the grid and REBASE text, loops over eleven cards computing sprite coordinates from card data and calling prt_spr, then moves the selected player by the direction table, clamps x and y, stores them into player data, and repeats until click equals one.
- `show_positions`: Draws the grid and POSITIONS text, loops over eleven cards computing sprite coordinates from card data and calling prt_spr, draws the selected player sprite, toggles, and returns once click equals one.
- `draw_grid`: Clears the sprite index, then draws two rows of eight doors by repeatedly calling prt_doors while stepping the sprite index and x position, advancing the row y position between rows.
- `display_cover`: Draws the slim card and opponent name list, highlights the current cover choice, handles up/down input to move the selection, and on click stores the chosen opponent data pointer into the current player record.
- `tr_man_to_sub`: Copies player data into substitute structures, swaps 64 bytes of player and substitute data, then swaps 10 bytes of player and substitute names.
- `calculate_time`: For eleven card entries and four reserve card entries, divides each stored time by the game timer, multiplies by 100 and writes the percentage into the show_time array, using 100 when times match.
- `prt_col`: Stores a0 as the print pointer, then loops eleven times copying a word from the pointer into small_prt, calling text_printer, advancing the pointer by 64 and d2 by 8.
- `prt_col_res`: Stores a0 as the print pointer, then loops five times copying a byte from the pointer into small_prt, calling text_printer, advancing the pointer by 64 and d2 by 8.
- `prt_col_num_byte`: Loops eleven times, reading a byte from the pointer in a0, converting it to text via conv_num_text and text_printer, advancing the pointer by 64 and d2 by 8 each pass.
- `prt_col_num_byte_res11`: Stores the value 16 into the column loop counter at a fixed memory location, then branches unconditionally into the shared column-number printing routine.
- `prt_col_num_byte_res`: Loops five times, reading a byte from the pointer in a0, converting it to text via conv_num_text and text_printer, advancing the pointer by 64 and d2 by 8 each pass.
- `prt_col_num_byte_res11cost`: Loops sixteen times over a byte column, reads each byte through prt_data, picks the yes or no text pointer, calls text_printer, then advances prt_data by 64 and d2 by 8.
- `prt_col_num_word11`: Writes the value 16 to the sprite variable at memory location 824e, then branches unconditionally to the column number printing code.
- `prt_col_num_word`: Loops eleven times reading a word through prt_data, converts it with conv_num_text while preserving d1, prints the number text, then advances prt_data by 64 and d2 by 8.
- `prt_col_speed_word`: Loops eleven times over speed and print data tables, converts each value to text, sets font red when the speed byte differs from the printed word, calls the text printer, advances both pointers and d2, then restores black font.
- `prt_col_speed_word_res`: Loops five times over speed and print data tables, converts each value to text, sets font red when the speed byte differs from the printed word, calls the text printer, advances both pointers and d2, then restores black font.
- `prt_col_time`: Loops eleven times then five times, converting each word pointed to by the prt_data pointer into text via conv_num_text and text_printer, advancing the pointer by two and d2 by eight each pass.
- `small_prt`: Copies d0 into itself and falls through to the next routine, so it performs no useful work and returns nothing.
- `copy_phy_to_log`: Loads the physical screen pointer and logical screen pointer from absolute memory, then copies 8000 longwords from physical to logical memory before returning.
- `draw_topcard`: Sets sprite x and y variables, calls prt_spr, then loops 17 times advancing sprite x by 16 and calling prt_spr, finishing with one extra sprite draw.
- `draw_restcard`: Sets sprite variables and draws a bordered card grid of five or six rows, each row made of a left edge, sixteen middle tiles and a right edge, advancing the sprite y position by 24 per row.
- `draw_small_card`: Draws a small card frame with prt_spr, then prints home and away team names in white and both scores in black via text_printer and conv_num_text.
- `draw_slim_card`: Draws a slim card frame using slim_x and slim_y positions, calling prt_spr for each sprite row and column before returning.
- `update_timers`: Adds one to the interrupt timer at memory location 8c5c, resets it to zero when it reaches 30, and adds one to the long game timer at 3fba.
- `show_clock`: Reads the time, converts minutes and seconds to digits, builds the clock string, replaces a space with a zero, then prints it in white and restores black font color.
- `clock`: Loads the longword at offset 48 from the table addressed by a0 plus d3 into d0, then truncates d0 to its low word.
- `drop_in`: Copies base position into twenty-two player records with staggered offsets, mirrors ten opponent records around 1600, then sets the ball position, velocity and state fields.
- `new_player_control`: Checks game state word at 3fa8 and 3fa6, and if a5 field at offset 44 equals one, branches away; otherwise calls find_new_man and returns.
- `new_player_control_away`: Checks game state word at 3fa8 for two and 3fa6 for zero, and if a5 field at offset 44 equals one, branches away; otherwise calls find_new_man_away and returns.
- `player_intelligence`: Iterates through a table of player structures to update their target coordinates based on their role and the ball position.
- `big_boot`: Writes kickmeter and kick height values, sets a direction word to zero, four or minus four depending on the int timer, then calls the shot routine and returns.
- `shoot`: Writes kickmeter and kick height values, sets direction to three or minus three based on the ball y offset, stores the int timer as height when below sixteen, then calls the shot routine.
- `pass`: Reads three words from the pass table pointer into d0, d1 and d2, advances the pointer, wraps to the table start on a 255 marker, stores the kickmeter and height globals, negates height when the player side field exceeds 80, then calls no_shot_10.
- `player_intelligence_away`: Loops over ten away player records, sets each player's target x and y from the ball or role position, and for the controlled player either chases the ball, shoots, passes or boots depending on ball position, possession and timer globals.
- `ashoot`: Writes kickmeter 10 and height 3 to the shot globals, flips height to minus three when the player side field is above 80, copies the intelligence timer into the height global when below 16, then calls no_shot_10.
- `apass`: Reads three words from the apass script pointer into d0, d1 and d2, advances the pointer, wraps to the table start on a 255 marker, stores kickmeter and km_height values, negates height when the player's field side is below 81, then calls no_shot_10.
- `move_players`: Iterates through a list of player structures, updating a fitness value and a speed attribute based on a timer.
- `move_ball`: Snaps the ball to the controlled player plus a dribble table offset, checks the kickmeter, tests opposing players with ball_hit_check for a tackle, plays footsteps and squirts the ball loose.
- `squirt_ball`: Initialises kick meter and height, launches the ball upward with velocity, then integrates ball position, bounces off glass, floor and ceiling, and awards goals to home or away score.
- `forget_gravity`: Scans the twenty-two player records for a collision with the ball, and if the ball is slow enough, transfers player control to that man and sets possession side.
- `ball_hit_check`: Clears the collision flag, compares ball and player coordinates in d0-d3 within twelve units and ball height below thirty-three, then sets collision to one.
- `near_target`: Clears the collision flag, compares d2 against d0 and d3 against d1, and sets the flag to 1 or 3 when both differences are within ten; otherwise returns without change.
- `out_bounds_left`: Clears the outbounds flag, then sets it to 1 when the object's x position at offset 2 is left of the computed left boundary from offset 4.
- `out_bounds_right`: Sets the outbounds flag to 1 when the object's x position at offset 2 is at or beyond the computed right boundary from offset 4, otherwise leaves it unchanged.
- `move_player_joystick`: Checks player count and kickmeter state, updates kickmeter height and swerve from the joystick direction, then sets ball velocity and calls the kick effect.
- `no_shot_10`: Clears the kickmeter, sets ball height and direction from the player's facing and swerve values, zeroes player state and calls the kick sound effect.
- `move_player_joystick_away`: For two-player mode, updates the away player's control fields, adjusts kickmeter and km_height values from the joystick direction, and branches to kick or find-new-man code.
- `find_new_man`: Scans eleven player records, computes each one's Manhattan distance from the ball, and stores the closest record pointer and its distance in globals.
- `find_new_man_away`: Scans eleven away player records for the smallest Manhattan distance to the ball, stores the closest pointer, then applies joystick movement, pitch bounds and groan effects.
- `set_pitch_x`: Calculates a value based on a word at offset 2 of a structure pointed to by a global variable, subtracts 160, and stores the result in a global variable.
- `draw_players`: Advances head animation, walks the 22 player records updating poses and visibility, appends visible players and the ball to the print zone, sorts them by depth and calls prt_men.
- `prt_men`: Loops through a table of data based on a count in a global variable, loading three words per entry into global variables from an offset in a table.
- `draw_pitch_backdrop`: Sets line_length and line_spr_x, clamps pitch_x to a sixteen-pixel boundary, draws left or right end sprites, then calls the wall, pitch, yard-line and number drawing routines to render the field.
- `prt_left_end`: Copies 192 rows of eight-pixel column pairs from the left-end sprite data into the current screen buffer at four interleaved plane offsets, advancing source and destination each row.
- `prt_right_end`: Copies 192 rows of eight-pixel column pairs from the right-end sprite data into the current screen buffer offset by line_right, writing four interleaved plane words per row.
- `draw_tv`: Loads two camera display constants into d0 in turn and calls display_camera for each, then returns to its caller.
- `display_camera`: Sets sprite height to 16, advances the tvscan pointer and counter, wraps after 24 entries, then draws two TV sprites offset by pitch_x from d0.
- `draw_tvlights`: Sets sprite height to 16 and draws light sprites at four pitch_x-relative x positions, using three-light and two-light sprite groups.
- `prt_3lights`: Calls the TV sprite printer five times, setting the sprite variant to one through five and advancing the sprite x position by sixteen pixels before each call.
- `prt_2lights`: Calls the TV sprite printer three times, setting the sprite index to 1, 4 and 5 and advancing the sprite x position by 16 before each call.
- `draw_controlled_sign`: Reads the controlled home and away player positions, computes mirrored screen x when play goes right, sets sprite indices 58 and 57 and calls the sprite printer for each sign.
- `draw_kickmeter`: When the kickmeter flag equals one, draws five bar sprites at 16-pixel steps, then draws three marker sprites offset by the kickmeter and height values.
- `draw_numbers`: Writes sprite numbers and x positions into the sprite variables, subtracting pitch_x from each fixed x, then calls prt_spr eight times to draw the pitch numbers.
- `draw_30yard_line`: Compares the word at memory location 3fa4 against 161 and 480; if within range, sets d2 to 480 and branches to draw_50yard_line, otherwise branches there without changing d2.
- `draw_20yard_line`: Compares the word at memory location 3fa4 against 1 and 320; if within range, sets d2 to 320 and branches to draw_50yard_line, otherwise branches there without changing d2.
- `draw_80yard_line`: Compares the word at memory location 3fa4 with 961 and 1280; if within range, sets d2 to 1280 and branches to draw_50yard_line, otherwise branches there without changing d2.
- `draw_40yard_line`: Compares the word at memory location 3fa4 with 321 and 640; if within range, sets d2 to 640 and branches to draw_50yard_line, otherwise branches there without changing d2.
- `draw_60yard_line`: Compares the word at memory location 3fa4 with 641 and 960; if within range, sets d2 to 960 and branches to draw_50yard_line, otherwise branches there without changing d2.
- `draw_70yard_line`: Compares pitch_x against 801 and 1120; if outside that range it branches away, otherwise sets d2 to 1120 and branches to the 50-yard line drawing code.
- `draw_50yard_line`: For pitch_x between 481 and 800, computes a column offset and table index, then loops 160 times masking and ORing five words into four screen planes at 40-byte row spacing.
- `draw_top_wall`: Sets line length to 31, computes a source address from pitch_x alignment into the pitch data, stores it with zero y, then calls prt_line_spr to draw the top wall.
- `draw_play_pitch`: Sets line length to 31, then for five sprite pointers and y offsets (1280, 2560, 3840, 5120, 6400) stores them in globals and calls prt_line_spr to draw each pitch line.
- `prt_line_spr`: Reads four pixel words per row from the line sprite data and writes them into the current screen buffer at offsets 0, 8000, 16000 and 24000, advancing rows until both counters expire.
- `read_time`: Empty routine: returns to the caller immediately without doing anything.
- `zero_time`: Writes zero to four consecutive time-related words in memory, clearing the GEM time and minutes counters, then returns to the caller.
- `hold_time`: Empty routine: returns to the caller immediately without doing anything.
- `get_kb`: Calls clear_kb, then repeatedly calls read_kb until it returns nonzero, calls getascii, and returns the resulting word from d1 in d0.
- `random2`: Saves a0 to memory, calls random, loads the random word, masks it with one to return a single bit, restores a0 and returns.
- `random4`: Saves a0 to a global, calls the random generator, loads the result, masks it with 3 to return a value from 0 to 3, restores a0 and returns.
- `random8`: Saves a0 to a global, calls the random generator, loads the result, masks it with 7 to return a value from 0 to 7, restores a0 and returns.
- `random16`: Saves a0 to a global, calls the random generator, loads the result, masks it with 15 to return a value from 0 to 15, restores a0 and returns.
- `random100`: Saves a0 to a global, calls the random routine, masks the returned word with 127 and repeats until the value is at most 100, then restores a0 and returns.
- `text_printer`: Saves a0, a4, d1 and d2, then loops over the string at a0, writing each character plus its x and y position into the sprite variable buffer, advancing x by 6, calling prt_text, and restoring registers at the end.
- `cls`: Loads the screen pointer from absolute address 860, then writes zero to 8000 longwords starting there, clearing the display bitmap, and returns.
- `prt_text`: Loads a zero into d0, reads three words from a pc-relative table into d0-d2, sets a4 from low memory, d7 to 8, computes a3 from the table base plus d0 times 80, then branches to prt_spr.
- `prt_spr_tv`: Loads a zero into d0, reads three words from a pc-relative table into d0-d2, sets a4 from low memory, d7 to 32, indexes a word table by doubled d0 to fetch an offset, adds it to a sprite base in a3, then branches to prt_spr.
- `prt_man`: Clears d0, loads three words from the sprite variable table into d0-d2, reads the screen pointer into a4, sets d7 to 32, indexes the sprite offset table by d0 doubled, fetches a word offset, adds it to the man sprite data base in a3 and branches to the sprite blitter.
- `prt_doors`: Clears d0, loads three words from the sprite variable table into d0-d2, reads the screen pointer into a4, sets d7 to 32, indexes the sprite offset table by d0 doubled, fetches a word offset, adds it to the door sprite data base in a3 and branches to the sprite blitter.
- `prt_logo`: Zeroes d0-d3, loads three words from the sprite variable table into d0-d2, reads the current screen pointer into a4, sets d7 to 32, indexes the sprite offset table by d0 doubled, fetches a word offset, adds it to the logo sprite data base in a3 and branches to the sprite blitter.
- `prt_proman`: Loads sprite variables, indexes the sprite offset table and proman data, sets screen pointer and width, then branches to the shared sprite blitter.
- `prt_spr`: Reads sprite variables, computes source and screen addresses, clips horizontally and vertically, then masks and merges five words per row into four bitplanes.
- `conv_num_text`: Calls step1 to convert d0 into numtemp, then copies numtemp into number, replacing leading digit positions with spaces so the four-character result is right-aligned.
- `step1`: Loads the numtemp buffer address into a0, zero-extends d0 into d1 and jumps to the decimal digit conversion routine.
- `deci_4`: Divides d1 by one thousand, one hundred and ten in turn, calling the digit routine after each division to store each decimal digit.
- `digit`: Adds 48 to the low byte of d1 to form an ASCII digit, stores that byte at (a0) and increments a0, then clears d1 and swaps its halves before returning.
- `number`: Predecrements a0 twice, loading two longwords from the stack area into d0, then executes a zero word that decodes as ori.b #0,d0, indicating data rather than code.
- `save_it`: Checks the chosen slot number and, for slot zero, one or two, points the filename, destination and length globals at the matching fixture file and calls save_ to write it.
- `load_it`: Checks the chosen slot number and, for slot zero, one or two, points the filename, destination and length globals at the matching fixture file and calls load_ to read it.
- `error_handler`: Releases the button, copies physical to logical screen, sets blue font, prints two fail lines via text_printer, then toggles and waits for display.
- `save_`: Loads the base pointer from absolute address 4fd0, subtracts 15516 to form the relocated data pointer, stores it at 15512, then calls save_file and returns.
- `load_`: Calls load_file, rebuilds the base pointer by adding 15516 to the stored value at 15512, writes it to address 4fd0, calls setup_teams, sets the continue flag at 10a6 to one and returns.
- `start_music`: Loads the address 905e into a0 as the music module pointer, branches to the MU_startmusic wrapper and returns to the caller.
- `fade_music`: Writes 63 to the fade volume word, then busy-waits while that word stays positive, calling the music stop routine once it reaches zero before returning.
- `MU_startmusic`: Stores the song pointer in the music data, table and song pointers, silences all four audio channels, clears music state words, scans the instrument size table for the largest entry, calls MU_calcins, and installs the music interrupt vector.
- `MU_calcins`: Walks up to d7 instrument entries of 16 bytes, skipping empty ones, rebasing each start and end pointer to a1, replacing the end pointer with MU_empty for looping entries, and advancing a1 by each entry's size word.
- `MU_stopmusic`: Stores a global value, clears the four audio voice volume registers to zero, then writes 15 to DMACON to disable the audio DMA channels and returns.
- `MU_interrupt`: Saves all registers, tests bit 5 of the custom interrupt request register, calls the song player if set, restores registers and jumps to the old interrupt vector.
- `MU_playsong`: Calls the per-vertical update, increments the song counter at a fixed memory location, loads the song limit, compares it with the counter and returns if the limit is greater.
- `MU_setinstr`: Saves d0 into d5, masks d0 with the high nibble bits, and if any remain branches to the channel setup code, otherwise returns immediately.
- `MU_seteffect`: Splits d5 into low byte and effect nibble, then branches to the matching effect routine for slide, volume, position jump, song step or break flag, or returns when the nibble is zero.
- `MU_everyvert`: Adds slide values to the four audio period registers, reloads sample pointers and lengths for stopped channels, sets the DMA channel mask, and applies a fading volume to all four channels before returning.
- `fx_stop`: Saves a5, points a5 at the custom chip base, writes value 15 to the DMACON register to disable DMA channels, restores a5 and returns.
- `fx_buzzer`: Loads sound effect number 0 into d0, calls the set_sound routine to start it, then returns to the caller.
- `fx_kick`: Loads sound effect number 1 into d0, calls the set_sound routine to start it, then returns to the caller.
- `fx_footsteps`: Loads sound effect number 2 into d0 and branches to the set_sound routine, then returns to the caller.
- `fx_groan`: Loads sound effect number 3 into d0 and branches to the set_sound routine, then returns to the caller.
- `fx_bounce_glass`: Loads sound effect number 4 into d0 and branches to the set_sound routine, then returns to the caller.
- `fx_bounce_ceiling`: Loads sound effect number 5 into d0 and calls the set_sound routine, then returns to the caller.
- `fx_bounce_floor`: Loads sound effect number 6 into d0 and calls the set_sound routine, then returns to the caller.
- `fx_crowd_left`: Saves a5, points it at the custom chip base, sets audio channel 2 period, length, volume and modulation, enables DMA audio, restores a5 and returns.
- `fx_crowd_right`: Saves a5, points a5 at the custom chip base, then programs audio channel 0 with the crowd sample pointer, length, period and volume and enables DMA before restoring a5 and returning.
- `set_sound`: Masks d0 to a sound index, indexes the sound table, and if the selected channel is idle programs its pointer, length, volume and period into the Paula audio registers and marks the channel busy.
- `replay`: Checks each of the four channel-active flags, and for every active channel disables its audio interrupt and DMA via INTENA and DMACON, then clears all four channel flags and returns.

### ProSoccer 2190.adf:echo

- `load_and_install_sound_sample`: Reads a sample number from the sound driver, finds the largest sample size in the linked list, allocates chip memory, copies the sample data and installs it into the driver's sample table, returning zero on success or minus one on failure.
- `copy_d1_to_d0_and_branch`: Moves the value in d1 into d0 as the return value and branches forward to another routine's code, transferring control without returning here.

### ProSoccer 2190.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `store_address_to_global`: Stores a 32-bit constant into a global variable and returns immediately.
- `process_interrupt_requests`: Writes 1920 to the interrupt request register, then tests bits 7 through 10 of the value read from the interrupt request register to call one of four different routines based on which bit is set.
- `update_interrupt_mask`: Compares a global variable to zero; if it is not greater than zero, it writes 80 to the interrupt enable register and 0 to a global variable, then writes 80 to the interrupt request register and branches.
- `update_dma_and_interrupts`: Compares a global variable to zero; if it is not greater than zero, it writes 2 to the DMA control register, 100 to the interrupt enable register, 0 to a global variable, and 100 to the interrupt request register before branching.
- `update_interrupt_enable`: Compares a global variable to zero; if it is not greater than zero, it writes 200 to the interrupt enable register, 0 to a global variable, and 200 to the interrupt request register before branching.
- `update_dma_and_interrupts_alt`: Compares a global variable to zero; if it is not greater than zero, it writes 8 to the DMA control register, 400 to the interrupt enable register, 0 to a global variable, and 400 to the interrupt request register before branching.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_counter_loop`: Initializes a register with 1000 and enters a loop that decrements the value until it reaches negative one.
- `decrement_large_counter_loop`: Initializes a register with 10000 and enters a loop that decrements the value until it reaches negative one.
- `call_stub`: Call stub: calls a routine at a fixed address outside this code unit, then returns to its own caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `update_memory_blocks`: Calculates an offset based on a global variable and a value from a hardware register, fetches five words from a table, and enters a loop that performs bitwise AND and OR operations on four specific memory locations using those words.
- `calculate_destination_address`: Calculates a destination address based on a value from a hardware register and several constants, stores it in a global variable, and calls another routine.
- `copy_multiple_data_blocks`: Sets a value in a global variable and then calls another routine five times, each time with a different destination address and length stored in global variables.
- `copy_data_to_memory`: Calculates a destination address using a global variable and several values from a table, then enters a nested loop to copy sets of four words from a source table to the destination memory.
- `execute_conditional_routine`: Calls two routines and enters a loop that repeatedly calls the second routine as long as it returns zero, then calls a third routine and returns.
- `check_first_bit`: Stores a pointer in a global variable, calls another routine, and returns the result of a bitwise AND between a global variable and one.
- `check_two_bits`: Stores a pointer in a global variable, calls another routine, and returns the result of a bitwise AND between a global variable and three.
- `check_three_bits`: Stores a pointer in a global variable, calls another routine, and returns the result of a bitwise AND between a global variable and seven.
- `call_and_mask_global`: Stores the value of a0 into a global variable, calls another routine, masks the lower 4 bits of a global variable in d0, and returns.
- `call_until_threshold`: Stores the value of a0 into a global variable, calls another routine in a loop until the lower 7 bits of a global variable are greater than 100, and then returns.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `clear_memory_block`: Fills a block of memory starting at a pointer from a global variable with zeros using a loop that runs 8000 times.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_function_via_pointer`: Loads a pointer from an offset of a structure in a1, loads another pointer from an offset of a table at a0, and calls the second pointer.
