# Game 2357 (spy vs spy - the island caper (1989)(wicked software)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02357). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 26 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 226 raw-word lines |
| Functions named and described | pass, 696 of 696 named (223 author symbols), 696 of 696 described (31 by rule, 378 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1383 of 1383 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16639 of 17006 instructions identical in start and length, 5 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 219750 |
| code | 68612 |
| data | 111422 |
| padding | 5208 |
| unused | 496128 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| cd | 2 | 117 | 2 |
| spy3titles | 2 | 2162 | 103 |
| spy3 | 2 | 9348 | 252 |
| port-handler | 1 | 306 | 1 |
| disk-validator | 1 | 386 | 1 |
| ram-handler | 2 | 10 | 1 |
| serial.device | 4 | 976 | 83 |
| gb | 1 | 12 | 1 |
| parallel.device | 3 | 322 | 26 |
| printer.device | 19 | 3272 | 218 |
| arp.library | 3 | 85 | 7 |
| Spy vs Spy - The Island Caper (1989)(Wicked Software).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- spy vs spy - the island caper (1989)(wicked software)/Spy vs Spy - The Island Caper (1989)(Wicked Software).adf: blocks 1110-1110 (bytes 568320-568832) hold ASCII text

## Cracktros (kept, not protection)

- none

## Copy protection found

- `fill_channel_record_fields` (spy3titles): Fills a two-part record: one part receives a 32-bit pointer and two 16-bit values, the other receives zeroed counters, a flag set to one and a 32-bit value. Kind: disk hardware/track validation. Pass path: the routine returns, and the caller continues with the provided parameters in D0 and D1 (likely command/track IDs or buffer offsets).

## Functions


### cd

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen consecutive 32-bit values from a fixed offset in a state block into a caller-supplied destination when that pointer is non-null, then calls a routine through a code pointer.

### spy3titles

- `setup_and_launch_program`: Program entry point: it calls several setup routines, copies values between global slots, runs a fixed-count delay loop, then calls a function from a table reached through a stored library base.
- `wait_for_raster_line_range`: Polls the vertical beam position register until the current line falls inside a window that starts at the supplied value and extends a fixed distance beyond it.
- `split_interleaved_words_into_planes`: Copies 4000 groups of four 16-bit words from a source buffer, writing each word of a group into a separate plane area 8000 bytes apart in the destination.
- `shift_sixteen_words_left`: Copies sixteen 16-bit values from a source pointer into a global table, doubling each value on the way.
- `open_dos_library_base`: Opens the DOS library through the system exec base, stores the returned library base in a global, then makes two further library calls on that base.
- `initialise_display_registers`: Fills a table with preset constants and copies it into a second table, then writes display window, fetch, bitplane modulo and control registers and triggers the copper list start.
- `fill_bitplane_pointer_table`: Fills four entries of a pointer table, each storing a 32-bit address split into high and low 16-bit halves, with each address 8000 bytes after the previous one.
- `save_register_values_to_globals`: Calls a helper, then writes its return value and several register inputs, including two all-ones words, into consecutive global slots before returning.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `halt_after_single_call`: Calls one routine and then loops forever with no exit, so execution never returns from this point.
- `clear_block_of_zero_longs`: Writes zero into 2000 groups of four 32-bit words starting at the given pointer, clearing 32000 bytes.
- `set_global_pointers`: Stores the address of one global buffer into three global pointer slots and returns without further work.
- `allocate_memory_or_halt`: Calls the exec AllocMem function, returns when it succeeds, and spins in an endless loop when the allocation returns zero.
- `spin_forever_loop`: Does no work and loops endlessly; it is a dead end where execution never returns, not a real routine.
- `unpack_rle_bitplanes`: Copies sixteen 16-bit values into a destination table, then decodes three 8000-byte bitplanes from a stream of literal bytes and marker-introduced runs, writing each byte with interleaved spacing.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `wait_for_status_bit_toggle`: Calls a library function, then runs a fixed series of seventeen iterations, each calling a routine and waiting for a status bit to clear and then set.
- `execute_indirect_calls_and_loop`: Executes a function via a pointer stored in a global variable, followed by a recursive call to itself.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `execute_callback_and_loop_with_wait`: Executes a function pointer from a table, performs a recursive call, and enters a loop that repeats sixteen times.
- `execute_indirect_calls_and_loop_2`: Calls a function via a pointer stored in a global variable, followed by a recursive call to itself.
- `execute_indirect_calls_and_loop_3`: Executes a function via a pointer stored in a table, performs a recursive call, and then enters a loop that repeatedly calls itself while polling a bit in a global variable.
- `execute_indirect_calls_and_wait`: Executes a function pointer from a table, performs a recursive call, and enters a loop that polls a bit in a global variable while decrementing a counter.
- `synchronize_hardware_status`: Executes a routine via a pointer, then enters a loop that performs seventeen iterations of polling a specific bit in a global variable until it toggles from set to clear and back to set.
- `execute_indirect_loop`: Calls a function pointer retrieved from a global variable, then enters a loop that executes another routine sixteen times.
- `forbid_wait_then_permit`: Disables task switching through the exec library, runs the seventeen-iteration wait for a status bit to clear and then set, and re-enables task switching through the exec library.
- `initialise_audio_channels_and_buffer`: Fills an 8192-byte buffer with values from a shift-and-add generator, sets the length registers of four audio channels to 32 words, and writes the per-channel records with pointers to that buffer.
- `fill_channel_record_fields`: Fills a two-part record: one part receives a 32-bit pointer and two 16-bit values, the other receives zeroed counters, a flag set to one and a 32-bit value.
- `forward_call_and_return`: Forwards the call: it invokes one other routine and then returns at once, with no further work of its own, so it acts as an indirection entry reached through a pointer table.
- `invoke_recursive_call`: The routine performs a direct recursive call to itself before returning.
- `invoke_recursive_call_2`: The routine performs a single immediate call to itself before returning.
- `execute_indirect_call`: This routine performs an indirect call by loading a function pointer from a data structure and jumping to it.
- `call_fill_channel_record`: Calls a routine to populate a two-part record with a pointer, two values, and initialized counters.
- `call_fill_channel_record_2`: Triggers a routine that fills a record structure with a pointer, two values, and specific flags.
- `call_fill_channel_record_3`: Executes a routine to write a 32-bit pointer and two 16-bit values into a record while zeroing counters.
- `call_fill_channel_record_4`: Invokes a routine to initialize a channel record with a pointer, two values, and a flag set to one.
- `call_fill_channel_record_5`: Performs a call to a routine that populates a data structure with a pointer and two 16-bit values.
- `call_fill_channel_record_6`: Requests the filling of a two-part record containing a pointer, two values, and zeroed counters.
- `call_fill_channel_record_7`: Calls a routine to set up a record with a 32-bit pointer, two 16-bit values, and a flag.
- `call_fill_channel_record_8`: Triggers the population of a record structure using a pointer and two 16-bit values.
- `call_fill_channel_record_9`: Calls a routine to fill a record with a pointer, two values, and a flag set to one.
- `execute_indirect_loop_2`: Calls a function pointer retrieved from a global variable and then enters a loop that executes another routine sixteen times.
- `call_fill_channel_record_10`: Executes a routine that populates a record with a pointer, two 16-bit values, and zeroed counters.
- `fill_channel_record`: Calls a routine to populate a two-part record with a pointer, two values, and zeroed counters.
- `fill_channel_record_2`: Initializes a record structure using a specific routine that sets a flag and counters.
- `fill_channel_record_3`: Executes a routine to fill a data record with a pointer and specific 16-bit values.
- `fill_channel_record_4`: Performs a record initialization by passing a pointer and two values to another routine.
- `call_conditional_chain`: Calls one routine, then calls up to three further routines in sequence, each one only while the zero flag left by the previous call stays set.
- `select_slot_and_dispatch_handler`: When a record's count field is zero, it looks through four candidate slots for one holding a positive nonzero value and returns failure if none is found.
- `match_or_store_slot_word`: Masks the incoming word to its low ten bits and, if it matches one of four stored slot values, marks that slot.
- `service_audio_channels_interrupt`: Runs the audio interrupt service: when enabled it counts down a timer, calls two routines on expiry if a flag is set, calls an update routine for each audio channel record that is active, and then clears the interrupt request by writing the register.
- `step_audio_channel_sequence`: Advances one audio channel's sequence record: when its counter is zero it loads the next entry from a sequence list, handling end and repeat markers, then adds the pitch and volume steps and writes the current values to the channel's custom chip registers.
- `load_audio_channel_registers`: Configures up to three audio channels from per-channel flags: depending on the flags it loads a sample pointer and length from the sample table or from a default buffer, sets the period and volume values, and clears a pending value.
- `store_byte_at_index`: Stores one byte into a global table at the index held in one register, taking the value from a second register.
- `reset_channel_state_pointers`: Sets a flag, calls three routines, then stores a global buffer address in two pointer slots and clears four counters.
- `save_and_restore_around_calls`: Saves two values and a pointer from globals, calls routines, and restores the saved values afterward.
- `select_channel_action_by_flags`: Chooses which helper routines to call based on two parameter values and two saved flags.
- `delay_then_exit_program`: Runs setup and cleanup calls, copies several saved values, busy-waits through a fixed loop of about 1.2 million iterations, then calls the DOS Exit function through a stored library base and returns zero.
- `load_background_and_object_pictures`: Makes a series of 23 consecutive calls to a loader routine.
- `clear_flag_and_write_mask`: Calls one routine, zeroes a stored flag, and writes a fixed mask value to a global.
- `call_three_helpers_in_order`: Calls three routines in sequence and returns with no other work, so it acts as an indirection point reached through a handler pointer.
- `execute_callback_sequence_2`: Executes a series of five function pointers retrieved from a table in a specific sequence.
- `mix_three_state_words`: Updates three 32-bit state words in place: the first is added into the second, the second into the third, and the third is recomputed from a rotate-and-subtract of the second, combined with a fixed constant and exclusive-ORed with its previous value.
- `halt_execution`: Calls a routine and then enters an infinite loop to prevent further execution.
- `initialize_and_halt`: Copies data from a stack to a global location, clears a large table, performs a long delay loop, and enters an infinite loop.
- `enter_infinite_loop`: Enters an infinite loop that performs no operations.
- `call_two_helpers_in_order`: Calls two routines in sequence and returns with no other work, so it acts as an indirection point reached through a pointer.
- `repeat_calls_while_word_nonzero`: Repeatedly calls one routine and then another, passing along an address register, for as long as a 16-bit value left in a data register stays nonzero.
- `repeat_call_until_zero`: Calls one routine repeatedly until the 16-bit value left in a data register becomes zero.
- `call_per_string_character`: Walks a zero-terminated character string, calling a routine for each non-terminator byte, and returns the data register word unchanged.
- `call_until_string_end`: Calls one routine repeatedly until the byte at the pointer is the zero terminator, then returns the data word unchanged.
- `process_null_terminated_string`: Iterates through a null-terminated string by calling another routine for each character.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a7)+ to d0, then returns.
- `copy_data_table`: Iterates through a table to copy values from a global variable into multiple locations via a pointer.
- `return_argument_unchanged`: Returns an eight-byte value passed on the stack unchanged; it performs no other work.
- `initialize_and_halt_system`: Copies data from a stack to a global location, clears a table, and enters an infinite loop after a delay.
- `store_routine_address_three_times`: Writes the address of one routine into three consecutive global slots and returns.
- `empty_routine`: This routine is empty and contains no executable instructions.
- `draw_masked_shifted_sprite_rows`: Draws rows into four bitplanes: for each row it builds an inverted mask from four source words shifted by a bit offset, clears the masked destination bits, then ORs in the shifted source words.
- `apply_bitmask_to_buffer`: Performs a bitwise AND operation on a destination buffer using a calculated mask, followed by a bitwise OR operation to merge source data into the buffer.
- `blend_bitmask_graphics`: Performs a bitwise logical combination of two data streams using a variable bit-shift amount.
- `apply_bitmask_to_buffer_2`: Performs bitwise AND and OR operations on a destination buffer using a source pattern that can be bit-shifted.
- `blit_masked_bitplane_rows`: Blits source pixel data into a destination buffer using a bitwise mask generated from the source pixels.
- `draw_masked_shifted_sprite_rows_2`: Draws rows into four bitplanes: the first source row is ORed into the planes, then each further row builds an inverted mask from shifted source words, clears the masked destination bits and ORs in the shifted words.
- `interleave_bitplane_words`: Reads words from four bitplane areas 8000 bytes apart and writes them into consecutive groups of four, rebuilding interleaved data row by row.
- `build_sprite_header_and_rows`: Writes a header holding a size value and two dimension words, then copies bitplane words into an interleaved sprite layout, marking the end of each row with a zero word.
- `copy_planar_data`: Calculates a total size and populates a table with dimensions before performing a nested loop to copy and rearrange data from a source buffer into a destination buffer.
- `program_blitter_and_start`: Programs the blitter for a rectangular copy by writing its control and mask words, source and destination addresses, modulo values and size, then calls helper routines, including a blitter wait, to start the transfer.
- `initialize_memory_buffers`: Sets specific byte values to global variables and enters a loop to zero out two memory locations for a fixed number of iterations.
- `initialize_and_clear_buffers`: Sets several global variables, clears two memory buffers using a loop, and then performs a sequence of palette updates and timed frame operations.
- `count_down_with_periodic_call`: Increments one counter on each call, then counts a second counter down; when that reaches zero it resets it to four and calls a routine.
- `reset_counter_and_call`: Sets a counter to four and calls one routine before returning.
- `build_palette_from_table`: Copies sixteen entries from a table row chosen by the index into the global palette buffer, doubling each value and adding 273 to it.
- `step_position_until_limit`: While a direction flag is nonzero, steps a position counter up or down by one, clears the flag when the counter reaches its target or the limit of 21, and then calls a routine.
- `step_position_with_limit`: Increments a position counter and clears a direction flag on reaching the limit of 21, then calls one routine.
- `copy_table_to_working_buffer`: Copies 32000 bytes from one buffer to another in groups of four 32-bit words and then calls one routine.
- `run_timed_frame_sequence`: Runs a frame loop that ends once the counter passes 359: each step toggles a parity flag, advances the frame counter, copies a 32000-byte source buffer into the working buffer (a different source for a range of frames), makes timed calls for certain frame values, and waits for the frame counter to reach two.
- `execute_blitter_copy`: Configures the blitter hardware for a rectangular memory transfer and initiates the operation.
- `call_four_helpers_in_order`: Calls four routines in sequence and returns with no other work.
- `advance_sprite_position_and_frame`: Moves a sprite by its velocity and reverses the velocity when the position reaches a fixed bound.
- `call_and_return_word`: Calls one routine and returns the 16-bit value in the data register unchanged.
- `modify_memory_via_bit_shift`: Performs bitwise XOR operations and arithmetic subtractions on a memory location using a value shifted by a variable amount.
- `initialise_and_run_game_modes`: Allocates memory, initialises input and sound, sets up the screen and loads all data, waits through a fixed loop, installs interrupt handlers, then repeatedly selects a mode and calls the play-initialisation and game routines until an abort flag is set.

### spy3

- `START_CODE`: Jump stub: passes control straight on to START.
- `DUMMY_SPRITE`: Enters a loop that waits until a value in a global variable satisfies a specific comparison against a register and a shifted value.
- `ATARI_COPY`: Copies data from a source buffer to a destination buffer using a loop that reorders words into specific offsets.
- `COLOUR_COPY`: Iterates through a source list and stores the values into a global table after shifting them left by one bit.
- `LOAD_FILE`: Opens the DOS library through the exec base and stores that library base.
- `SETUP_SCREEN`: Sets the two screen pointers, fills a copper table with preset values, copies a plane pointer table into a copper list, writes display window, fetch, modulo and control registers, and starts the copper list.
- `WHICH_PLANES`: Fills a table with values derived from a 32-bit integer, incrementing the value by a fixed amount in each iteration.
- `BLIT`: Waits for the blitter to finish, then writes its return value and several register inputs, including two all-ones words, into consecutive global slots.
- `WAIT_BLIT`: Polls the blitter-busy status bit in a loop and returns once the blitter is idle.
- `SWAP_SCREEN`: Waits for the blitter to finish and then spins forever with no return, so it is a dead end.
- `CLEAR_SCREEN`: Fills a block of memory with zeros using a loop that iterates two thousand times.
- `MYINT2`: Interrupt handler that records a keyboard event by calling the key matrix update routine, waits a short fixed delay, and then acknowledges the interrupt by clearing a status bit and writing the interrupt request register.
- `ADD_MATRIX`: Converts a keyboard code into a row and column position, then sets or clears the matching bit in the keyboard matrix table depending on the code's low bit.
- `DOWN_STROKE`: Clears one bit in a byte of the keyboard matrix at an offset given in a data register, selecting the bit from the low three bits of a second register.
- `INIT_KEY`: Sets four 32-bit words of the keyboard matrix to all ones.
- `INKEY`: Tests one bit of the keyboard matrix, selected by a key code, and returns all ones when that bit is zero and zero otherwise.
- `SCAN_JOY`: Reads joystick direction and fire bits from the input register values, storing left, up and fire flags for both ports, then stores four key-test results.
- `HANDLE_MOUSE`: Updates mouse movement tracking by calling a helper twice, then sets two button flags from register bits, one for each button.
- `DO_MOUSE`: Decays a movement counter by up to three steps per call, compares each axis byte with its previous value to set direction timers, and stores the new position.
- `LAST_MOUSE0`: Finds memory for two screens and clears each one, then finds memory for a back buffer.
- `LAST_MOUSE1`: Calls several routines to update three different global pointers.
- `KLEFT1`: This routine is a single instruction handler used for input processing.
- `ALLOCATE_MEMORY`: Finds available memory blocks and assigns them to three global variables representing different screen and background buffers.
- `CLEARD0`: Writes zero into 8000 consecutive 32-bit words starting at the pointer, clearing 32000 bytes.
- `FIND32K`: Calls a memory allocation routine and enters an infinite loop if the returned pointer is null.
- `DECO_PIC`: Loads a picture file, copies sixteen words of header data into a destination table, then decodes three 8000-byte bitplanes from a stream of literal bytes and marker-introduced runs, writing each byte with interleaved spacing.
- `DOS_BASE`: Calls a function via a pointer, performs a loop sixteen times to trigger sound routines, and then calls another function via a pointer.
- `DOS_HANDLE`: Performs a sequence of function calls and a loop that waits for a specific bit in a global variable to change.
- `ONESCREEN`: Executes a series of pointer-based calls and a loop that interacts with a global flag.
- `TWOSCREEN`: Triggers a sound loop and updates a global variable through a pointer-based call.
- `SCREEN2`: Runs a loop sixteen times to call a sound routine while monitoring a global variable bit.
- `SCREEN1`: Calls a function through a pointer and executes a loop that waits for a bitwise state change in a global variable.
- `COPPER1`: Calls a routine via a pointer, starts sounds, and enters a loop that plays sounds sixteen times while waiting for a specific bit in a global variable to toggle.
- `COPPER2`: Performs a sequence of calling a pointer, starting sounds, and looping sixteen times to trigger new sounds while polling a global variable bit.
- `COPPER_LIST2`: Calls an indirect function, then enters a loop sixteen times to trigger new sounds while waiting for a specific bit in a global variable to toggle.
- `E_KISS`: Executes a routine through a pointer, starts sounds, and runs a loop sixteen times that triggers new sounds and waits for a bit change in a global variable.
- `E_RUMBLE`: Calls a function pointer, starts sounds, and iterates sixteen times to trigger new sounds while monitoring a bit in a global variable.
- `TEST`: Calls a routine through a pointer, starts sounds, and loops sixteen times to trigger new sounds while waiting for a bit in a global variable to change.
- `START_SOUNDS`: Fills an 8192-byte buffer with values from a shift-and-add generator, sets the length registers of four audio channels to 32 words, writes the per-channel records, installs an interrupt handler and initialises the music routine.
- `GO_SOUND`: Writes several values to a table in memory and updates a hardware register.
- `HIGH_PING`: Calls a routine to play a sound.
- `MEDIUM_PING`: Calls the sound request routine and returns with no other work.
- `LOW_PING`: Calls a routine to play a specific low-frequency sound effect.
- `DROP_PING`: Calls another routine to initiate an audio playback sequence.
- `TAKE_PING`: Calls a routine to play a sound effect.
- `MEDIUM_BEEP`: Calls another routine to trigger a sound effect.
- `BOP`: This routine calls a subroutine to initiate a sound playback.
- `BANG`: Calls a routine to initiate audio playback.
- `OTHER_SMACK`: Executes a routine to play a sound.
- `GRAVESOUND`: This routine calls another routine to trigger a sound effect.
- `BOOM`: Requests a sound to be played.
- `BURY`: Calls a routine to play a sound and then returns.
- `SPLASH`: Plays a sound effect via a subroutine.
- `RUMBLE`: Calls a sound routine and another subroutine.
- `FEET`: Performs a sound playback call.
- `PICKY`: Triggers a sound through a specific routine.
- `SAWY`: Calls a routine to initiate sound playback.
- `KISS`: Calls a routine to play a sound and executes another routine.
- `SUB`: This routine calls a routine to play a sound and then returns.
- `HANDLE_SOUNDS`: Repeatedly calls a routine to handle requests based on the zero flag status.
- `HANDLE_REQUEST`: Checks a series of global variables to find a valid pointer and then calls a routine from a table using an index derived from a bitwise operation.
- `NEW_SOUND`: Searches four request slots for the given sound code and marks an existing match, otherwise stores the code in the first empty slot.
- `MYINT6`: Interrupt handler that polls the mouse, advances the music delay counter and plays the next music step, updates each active volume envelope, processes queued sound requests, and then clears the interrupt request.
- `CHANGE_ENVELOPE`: Modifies a data structure to update volume and envelope parameters, potentially iterating through a list of values and updating hardware registers.
- `X8912`: Configures up to three audio channels from per-channel flags: depending on the flags it loads a wave pointer and length from wave tables or from a noise buffer, sets the frequency and volume values, and clears a pending flag.
- `PSGW`: Stores one byte into the music buffer at an index held in one register, taking the value from a second register.
- `INITMUSC`: Sets the music-on flag, calls the music buffer write routine three times, points the two note-table pointers at their tables, and clears the delay and flag counters.
- `PLAYMUS`: When music is on, runs the channel player twice, saving and restoring the delay and pointer values between the two passes.
- `PLAY_CHANNEL`: Updates audio hardware registers and modifies global flags based on the state of a pointer and various counters.
- `MUSICON`: Performs a large sequence of initialization steps including memory allocation, screen setup, and a main loop that calls various subroutines.
- `DELAYB`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `FLAGB`: Sets a global variable, performs memory allocation, screen setup, and enters a loop that repeatedly calls initialization and game routines.
- `SAVE_INT2`: Performs a large loop to decrement a value, then stores two specific integer values into global variables.
- `TEMP_MUSIC`: Executes a loop that decrements a value until it reaches zero and updates two global variables.
- `clear_test_data`: An extremely short routine that clears or moves data as requested by its callers.
- `MUSINF_1`: A routine used by sound handlers to process information, though the provided code shows a large initialization loop.
- `clear_test_data_2`: A short routine that performs a clear or move operation on data.
- `MUSINF_3`: A routine called by sound-related handlers to manage music information.
- `MUSIC_DELAY`: A single-instruction routine used to manage timing or delays within a sound interrupt.
- `MASTER_ENABLE`: A single-instruction routine used to enable a system or interrupt state.
- `START`: Initializes memory, sound, and input, sets up screens, and enters a main loop that alternates between game modes.
- `READALL`: Runs a long fixed sequence of picture read, grab, buffer save and clear calls, referencing eight picture files, to load the graphics into memory.
- `MYINT3`: Interrupt handler that increments a frame counter, clears the interrupt request register and writes the interrupt acknowledge value.
- `GRAB`: Calls the buffer save routine four times in a row and returns.
- `GRABBIG`: Calls another routine three times in a loop.
- `GRAB1`: Calls another routine exactly once.
- `GRABOBJ`: Calls the buffer save routine twenty times in a row, then returns.
- `GRABLAND`: Small routine of 2 instructions that copies 20 to d3, then subtracts 1 from d3.
- `GRABLAND2`: Calls a buffer routine repeatedly, once for each count held in a register, counting down to zero, then returns.
- `GRABMAP`: Executes another routine three times in succession.
- `PLAYINIT`: Clears trail, menu, nudge and auto flags, copies a level's map and terrain blocks into buffers, clears player and object state fields, and places the initial objects through helper calls.
- `PLAYINIT0`: Resets numerous global variables for both players, clears the trail, and copies level-specific map and terrain data from a table into active memory.
- `GETA4`: Small routine of 3 instructions that clears d1, then copies (a4)+ to d1, then returns.
- `WHICH3`: Performs bitwise OR operations on three different memory locations using specific bitmasks.
- `WHICH4`: Performs a bitwise OR operation on a byte at a location pointed to by a register.
- `WHICH5`: Sets two flag bits in state bytes, then walks a table of 76 entries and saves a buffer for each nonzero entry.
- `WHICH6`: Sets one flag bit in a state byte, then walks a table of 76 entries and saves a buffer for each nonzero entry.
- `WHICH7`: Sets a bit in a memory location and then iterates through a table, calling another routine for every non-zero entry found.
- `READ_G`: Walks a table of 76 entries and saves a buffer for each nonzero entry.
- `INT_REQ`: Interrupt-time draw step: clears two blocks, draws eight lines when a mode value equals one, draws sprites (more when two players are present), then swaps the screen.
- `DRAW_NEW_SEL`: Clears blocks, draws lines, renders sprites, and swaps screens.
- `PARMS`: Runs a menu loop that reads button triggers and joystick input on every fourth count, moves a selection value up or down and plays a sound on each change, and cycles the level or player count when a trigger is pressed.
- `DO_GAME`: Runs the in-game loop: when no object is active it places one at a random free position, updates both players' movement and state each frame, moves the window and map positions, toggles a blink value, and returns when the abort flag is set.
- `READPIX`: Loads and decodes a picture into a buffer, splits its interleaved words into four bitplanes, and copies the colour words into the palette table with each value doubled.
- `STIT`: Loops a fixed number of times, calling the random-value routine and storing one fixed word value at the target address on each pass.
- `STUFFSPY`: Repeatedly picks random positions, converts them to map coordinates and retries until the two spies are a set distance apart.
- `RNDER`: Updates three random-generator state words with additions, a rotate-and-subtract step and an exclusive-or with a constant.
- `RNDER2`: Calls the random-state advance routine and returns whatever value it leaves in the registers as a long word.
- `GETRAND`: Advances the random state, derives a table offset from its bits and loops until the table entry at that offset is zero.
- `SETXY`: Subtracts a base value from the input offset, halves it, and divides it by ninety-six.
- `CLEAROUTCRAP`: Sets the four clock digit values and both spies' fuel values to all ones, so they are marked as cleared.
- `GOERROR`: Crash handler: saves the stack area, copies the palette, clears the screen buffer and prints register and stack contents as text.
- `BLOOP`: Jump stub: passes control straight on to BLOOP.
- `OPEIGHT`: Runs the four-item print loop twice in a row, so one call prints an eight-item block.
- `OPFOUR`: Repeatedly prints a hexadecimal value and then a zero-terminated text string until the loop counter reaches zero.
- `OPHEX`: Calls the single-character output routine repeatedly until the data register value reaches zero.
- `OPTEXTZ`: Walks a text pointer and sends each character to the single-character output routine until it reads a zero byte.
- `emit_text_until_nul`: Emits one character at a time from the pointer register until it reads a zero byte.
- `print_text_until_zero_byte`: Loops on the single-character output routine while the byte at the pointer is not zero.
- `stub_return_stack_word`: Stub with no effect: it returns the stack word unchanged and does nothing else.
- `OPITEM`: Copies the eight rows of one character from the font table into four bitplanes of screen memory.
- `return_stack_value`: Returns the value currently residing at the top of the stack.
- `ILLEG`: Jump stub: passes control straight on to GOERROR.
- `SETUPZ`: Stores the address of the illegal-instruction routine into one global location three times.
- `PALETTEZ`: This routine performs a sequence of operations on data pointed to by a handler stored in a data table.
- `AREGISTERS`: Checks whether each spy's map cell matches the current window, then computes its screen offsets and plots its sprite into the screen buffer.
- `draw_spy_sprites_by_window`: Checks each spy's map cell against the current window and, on a match, computes screen offsets, plots the spy sprite and draws its hands.
- `DREGISTERS`: Repeats the map-cell check and sprite plot for each spy in the window, drawing the spy and its hands when the cell matches.
- `FROMSTACK`: Performs the same per-spy map-cell check and sprite plot as its neighbours, returning through the stack path used by the crash handler.
- `VUSP`: Performs the same per-spy map-cell check and sprite plot as its neighbours, returning through the user stack pointer path.
- `DRAWSPY`: Checks each spy's map cell against the window and, on a match, plots its sprite and hands into the screen buffer.
- `DRAWHANDS`: Draws the hands sprite for one spy when that spy is not carrying a special state and its depth and swamp values are small enough.
- `DRAWBUTTONS`: Calls the single-button drawing routine seven times in a row to draw the full button row.
- `ONEBUTTON`: Draws one button using either the empty or the full glyph, depending on a flag passed in.
- `SWAPSCREEN`: Jump stub: passes control straight on to SWAP_SCREEN.
- `DRAWTIME`: Compares each of the four clock digits with its stored value and redraws only the digits that changed.
- `HLINE`: Calls the horizontal line routine and returns the register values it leaves behind.
- `METER`: Clamps each spy's energy value to the range zero to sixty-five and redraws its meter bar, skipping the full and empty extremes.
- `FLASH`: Draws two sprites, then draws the flash sprites twice more only when bit two of the global counter is set.
- `FLASH_ONE`: Clears a sprite field, then draws the sprite frames selected by a three-bit state value, adding extra frames under certain flag conditions.
- `COMPARETWICE`: Returns without change if the data value matches both the word at the pointer and the word after it.
- `SPRITER_MOD`: Chooses one spy's sprite to draw, splitting wide sprites into two plots when the spy is at its last hand frame.
- `RSPRITER`: Draws a shifted sprite into four bitplanes by clearing the destination with an inverted mask and then merging the sprite words.
- `RBACKER`: Clears each destination row in four bitplanes, then ORs in the shifted sprite words for each column of the row.
- `copy_shifted_sprite_rows`: Continues the shifted sprite copy row by row across the buffer.
- `RSAVE_BUFF`: Copies a rectangle of four bitplane words from screen memory into a save buffer, row by row.
- `RDRAW_BUFF`: Copies a saved rectangle from the buffer back into four bitplanes of screen memory, row by row.
- `RSTUFF_BUFF`: Stores a width-and-height header, then writes each row of the buffer as the OR of the four bitplane words from screen memory.
- `RSTUFF_BUFF_SHORT`: Writes a width-and-height header, then copies the four bitplane words of each screen row into the buffer.
- `RSPRITER_AM`: Programs the hardware blitter registers with the source, mask, destination pointers, modulo values and minterm logic, then starts the blit and waits for it to finish.
- `SPRITER`: Calls the masked sprite routine and returns its register values.
- `SPRITER_AM`: Calls the blitter-based sprite routine and returns its register values.
- `BACKER`: Calls the background sprite routine and returns its register values.
- `X`: Advances the snow timing, chooses the sleet or snow graphic from the minute counters, and draws snow flakes for each spy not inside an igloo.
- `WIDTH`: Runs the same snow weather update as its neighbours, with the same timing, graphic choice and per-spy flake drawing.
- `BUFFER`: Decrements a counter and conditionally updates a pointer to a slab based on global status values and calls another routine.
- `SCREEN`: Manipulates a bit in memory and updates a pointer to a slab based on several global conditions and calls another routine.
- `MINTERMS`: Runs the same snow weather update as its neighbours, with the same timing, graphic choice and per-spy flake drawing.
- `RSNOWING`: Advances the snow timing and draws snow flakes for each spy not inside an igloo.
- `DOSNOW`: Draws the snow flake graphic into the screen planes for each spy's column, using a per-row pattern table and a counter.
- `SNOWCOUNT`: Calls the font-glyph routine for the current character and returns with no other work.
- `NEXTCHAR`: Copies one eight-row character from the font table into four bitplanes, choosing the character from the low four bits of the data value.
- `CALCAD`: This routine consists of fifteen instructions but performs no functional operations or data manipulations before returning.
- `PLOTPOINT`: Sets or clears one pixel by applying a mask to the four bitplane words at the current screen address.
- `LEFT_RIGHT`: Reads the left-movement flag and a bit of the data value, then returns without changing any state.
- `UP_DOWN`: Reads the left-movement flag, then returns without changing any state.
- `DRAW_LINE`: Computes the horizontal and vertical step directions and a slope value, then plots points along the line by stepping one axis at a time.
- `GO_LEFT`: Walks a vertical span of rows, clearing or setting pattern bits in the four bitplanes and jumping to edge handlers at the span's ends.
- `HORLINE`: Draws a horizontal line of pattern bits across the bitplanes, clearing or setting bits by pattern value and handling the run's edges.
- `fill_row_bits_plane_one`: Clears or sets pattern bits in the first three bitplanes as it walks a row, stopping when the row count runs out.
- `fill_row_bits_plane_two`: Clears or sets pattern bits starting from the second bitplane as it walks a row, stopping when the row count runs out.
- `fill_row_bits_plane_three`: Clears or sets pattern bits starting from the third bitplane as it walks a row, stopping when the row count runs out.
- `fill_row_bits_then_line`: Clears pattern bits in the fourth bitplane for each row, then continues into the horizontal line routine.
- `GOT_BOUNDARY`: Writes whole sixteen-pixel words into all four bitplanes from the low pattern bits, stepping sixteen pixels at a time while enough remain.
- `write_pattern_words_plane_one`: Writes sixteen-pixel pattern words into the first bitplane, setting or clearing each word from the pattern bits.
- `write_pattern_words_plane_two`: Writes sixteen-pixel pattern words into the second bitplane, setting or clearing each word from the pattern bits.
- `write_pattern_words_plane_three`: Writes sixteen-pixel pattern words into the third bitplane, setting or clearing each word from the pattern bits.
- `write_pattern_words_plane_four`: Writes sixteen-pixel pattern words into the fourth bitplane, setting or clearing each word from the pattern bits.
- `LATER`: Writes the leftover pattern bits of a line that do not fill a whole sixteen-pixel word, setting or clearing one bit at a time across the bitplanes.
- `rotate_pattern_bits_plane_one`: Rotates the pattern value one bit at a time and clears or sets bits in the first bitplane for each step.
- `rotate_pattern_bits_plane_two`: Rotates the pattern value one bit at a time and clears or sets bits in the second bitplane for each step.
- `rotate_pattern_bits_plane_three`: Rotates the pattern value one bit at a time and clears or sets bits in the third bitplane for each step.
- `rotate_pattern_bits_then_line`: Rotates the pattern value and clears or sets bits in the fourth bitplane, then falls through to the horizontal line routine.
- `POINT`: Combines the two low data registers and masks the result down to a single bit pattern, returning it.
- `GET_RANDOM`: Updates the three random seed words with additions and a rotate by a shift count taken from the low bits, then subtracts the result from one seed.
- `READ_TRIGS`: Reads the joystick scan and the keyboard for each player and sets that player's trigger word to all ones when fire is held, or to zero otherwise.
- `MOVE`: Dispatches each spy's movement by its control mode, choosing joystick, keyboard, computer logic or random movement, and then updates both positions.
- `set_spy_direction_four`: Sets the spy direction value to four and writes two into the location the caller passes in.
- `set_spy_direction_three`: Sets the spy direction value to three and writes one into the location the caller passes in.
- `set_direction_three_and_check`: Sets the spy direction to three and writes one into the caller's location, then branches into the meeting, trap and state routines.
- `CHECKMEET`: Recomputes the window rows from the spies' map rows, then checks whether both spies share a window.
- `CHECKMEET1`: Moves the window column in steps of two while spy one is closer than a set distance, counting the steps.
- `step_window_toward_spy_two`: Moves the window column in steps of two while spy two is closer than a set distance, counting the steps.
- `mark_window_buffers_dirty`: Steps the window toward spy two as needed and sets both window buffer row flags to all ones when the step count is above one.
- `reset_window_to_spy_one`: Clears the window flags and sets the window column from spy one's map column, with a minimum of two.
- `CHECKMEETEND`: Sets both window buffer row flags to all ones when the count is above one, then returns.
- `SETTEMPS`: Copies the window and map position for the chosen spy into the temporary position variables.
- `DIRFIX`: Chooses the frame buffer address for a spy from its direction value, then advances that spy's frame counter, wrapping after four frames.
- `TEST_OPTIONS`: Decrements the special delay counter while it is above zero, and calls the option test routine once it reaches zero.
- `TEST_M`: Waits for a key press through the keyboard routine, then toggles a flag that sets or clears three sound volume values and resets the option delay to twenty.
- `JOYMOVE`: Scans the joystick and keyboard, then loops with short delays while waiting for a key or direction change, calling the option test routine as it waits.
- `SETWINDOW`: Copies the chosen window's column and row into the spy position variables.
- `GETWINDOW`: Copies the spy position variables back into the chosen window column and row.
- `LITTLE_DELAY`: Counts a fixed loop down to zero as a short delay.
- `FEET_COUNT`: Runs the per-frame drawing sequence: flashing, landscape and object draws or block clears for each spy's window, buttons, clock and the energy countdown.
- `DRAWMOVE`: Runs the per-frame drawing sequence for both spy views, buttons, clock and meters, then updates the snowball and snow.
- `RLINES`: Advances a snow line counter at a set rate and draws the snow line blocks into each screen row for spies outside an igloo.
- `MOVE_SNOWBALL`: Steps each active entry in the snowball list by its stored offset, and checks both spies' map positions against the entries.
- `DRAWLAND`: Draws the landscape background for one spy's window, setting the background pointer and buffer rows and redrawing slabs with the blitter as needed.
- `ONE_SLAB`: Clamps the slab row index to zero or above, then sets up the blitter with masks, modulo values and pointers to copy one landscape slab.
- `DRAWSLAB`: Calls the single landscape slab blit and returns.
- `FIRE_IN_HERE`: The routine performs no operations and immediately returns to its caller.
- `DRAWOBJ`: Draws the objects for each spy's window, setting the screen and window coordinates before calling the per-spy object routine.
- `DRAWSPYOBJ`: Sets the object position and sprite width, then draws the spy's object sprites into the screen buffer.
- `WHICHBACK`: Chooses which background picture to use from a table, based on the window position and a computed index.
- `SNOW_LIST`: Reads the trigger input and branches on each spy's trigger and state values to the matching action, such as moving, adding or clearing traps, or setting the spy's state.
- `BUSY`: Reads the trigger input and branches to the per-state handlers for the spies, such as trap, delay, position and setup.
- `DELAY_TABLE`: Looks up a delay entry and, depending on its value, adds or deletes traps, updates positions and states, or reads joystick movement.
- `BUSY4_A`: Copies the chosen spy's map position into the spy position, clears that spy's hand value, then updates its state and calls the position routine.
- `DELETE_TRAP`: Searches a fixed trap list for the entry matching the pointer.
- `ADD_TRAP`: Walks a list of trap positions, checking the spy's auto flag and a range test on each entry, and records the trap at the first slot that passes.
- `DOSETXY`: Compares a value against several constants and calls another routine if a match is found.
- `COUNTER1`: When no window is active, reads joystick movement, then picks a display width and state code from the spy's depth and swamp values.
- `BUSY5_0`: Handles one busy state by reading joystick movement, drawing the map display and sprite, then updating the spy state.
- `update_sprite_list_entries`: Walks a list of sprite entries until the end marker, computing an offset from each entry and drawing horizontal lines for it.
- `clear_both_spies_flash`: Clears both spies' flash values and one spy's counter, choosing the spy from the input value.
- `BUSY6_4`: Clears the counter and flash value for the chosen spy, using spy one when the input is zero and spy two otherwise.
- `SETSAFE`: Sets the chosen spy's safe value to four, using spy one when the input is zero and spy two otherwise.
- `KILLDOUBLE`: Stores the counter minus twenty into the chosen spy's trap time, and copies the value to the other spy's time when the input is zero.
- `GETPOS`: Converts the stored column value into a grid offset by dividing by ten and scaling, then sets the screen position from the row.
- `GETPOS1`: Computes a screen column from the input column, scaled by sixteen and offset, and sets the screen row from the row value.
- `SETSTATE`: Stores the state value into the chosen spy's state word, and clears its flash value when the state is zero.
- `GETBTIME`: Checks if a value in a register is zero and returns.
- `SHOWMAP`: Draws the level map sprite into the screen buffer at a fixed position, selecting the map image by the current level.
- `ABSD2`: This routine is empty and performs no operations before returning.
- `CLEANTRAIL`: Sets all six entries of both spies' trail lists to the all-ones value, marking them as empty.
- `SETUPA2`: Returns without change, so it is an empty setup hook with no effect.
- `SETUPA2D`: tests if a certain value is non-zero and returns or exits if it is zero.
- `SETUPA2E`: Returns without change, so it is an empty setup hook with no effect.
- `GETMODE`: Runs the mode selection screen: clears the trail lists, draws the menu items and waits for input, then starts a game with the chosen setup.
- `DRAWITEMS`: Loops over the menu items, positioning and drawing three horizontal line segments for each one through the line routine.
- `SELECTITEM`: Empty routine: returns to the caller immediately without doing anything.
- `DRAWLINE`: Calls the line drawing routine and returns its register values.
- `PY1`: Computes a screen address from the position, then fills a vertical run of bytes in the bitplane for the pixel count given.
- `PY2`: Computes a screen address from the position, then fills a vertical run of bytes in the bitplane for the pixel count given.
- `RCLEARBLOCK`: Computes a screen address from the position and clears a rectangle of bytes in the bitplanes for the given width and height.
- `BRAINMOVE`: Chooses a move for the computer-controlled spy using random values and map searches, then sets its state.
- `GETD0D1`: Sets the spy direction to three, four, one or two according to the lowest set bit of the flag word, or to zero when no bit is set.
- `BRAINBUSY1`: Clears a specific global variable to zero and writes a fixed value to a location pointed to by a register.
- `reset_spy_two_frame`: Clears spy two's frame value and writes a fixed value to the pointer word.
- `RANDMOVE`: Occasionally picks a random step direction for the chosen spy when it is not inside an igloo, using two random draws.
- `FIND_A_MOUND`: Searches the map cells around the spy's position for a suitable mound, using the map and position tables.

### port-handler

- `process_hunk_load_records`: Compares values with the AmigaDOS hunk type codes and runs the loader's record handling, updating entry fields and calling through pointers.

### disk-validator

- `walk_hunk_relocation_table`: Walks the hunk relocation records in a loop, adding to counters and writing offsets into the table entries.

### ram-handler

- `call_through_global_vector`: Loads a code pointer from a global table at a fixed offset and jumps through it.

### serial.device

- `terminate_execution`: The routine is empty and contains no valid instructions.
- `call_object_handlers`: Calls a handler from a table at a fixed offset, and when a value is supplied it calls a second handler and stores its result in the structure fields.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `clear_flag_and_call_handler`: Sets a flag byte in the structure, then if a mask test on another byte passes, calls the handler from a table entry.
- `set_interrupt_enable_bits`: Writes a fixed bit pattern to the interrupt enable register, calls a routine, then writes a second pattern.
- `unlink_list_entry_flag`: Clears the flag byte in the entry.
- `unlink_marked_list_entry`: Unlinks the entry from its list when its marker bit is set, updating the head pointer when the entry is the head.
- `unlink_flagged_list_entry`: Unlinks the entry from its list when its flag bit is set, decrements the list count and clears the flag.
- `process_library_list_entry`: When the entry is the list head with type ten and a flag set, it calls a library routine through a global base and then moves to the next entry.
- `unlink_flagged_entry_counter`: Unlinks the entry from its list when its flag bit is set and decrements a different list count.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `mark_or_release_entry`: When a field is nonzero it sets the entry's flag bit and returns zero.
- `call_library_routine_pair`: Calls a library routine twice through a global base, then returns the value stored in the entry.
- `update_entry_active_flag`: Clears the entry's value field, and when the value is nonzero and the conditions hold it clears the entry's active flag.
- `walk_linked_list_entries`: Starts at the list head pointer and walks the linked entries until it reaches the end, returning the last pointer it reaches.
- `walk_pending_list_items`: Loops while a count field in the structure is nonzero, moving through a list of pending items and calling helper routines for each one.
- `process_pending_list_items`: Iterates through a linked list of structures to update status flags and bitmasks.
- `process_linked_list_items`: Iterates through a linked list of structures, updating status flags and bitmasks within each element.
- `clear_bit_mask_from_field`: Clears one bit in the data value, with the bit number taken from a byte in the structure, then continues a list loop.
- `stub_empty_list_hook`: Does nothing and returns, so it is an empty hook with no effect.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `update_structure_flags`: Updates a flag byte in the structure, setting or clearing bits according to the entry's state, then calls the next queue step.
- `update_structure_flags_two`: Updates a second flag byte in the structure with the same bit logic, then calls the next queue step.
- `set_entry_state_twelve`: Sets the entry's state byte to twelve, then checks the structure's flags before continuing.
- `service_serial_interrupt_request`: Reads the interrupt request and serial data state, clears or sets the request bits, and processes the entry flags.
- `update_interrupt_enable_state`: Compares the input byte with a field in the structure.
- `buffer_input_byte`: Checks status flags and attempts to store an input byte into a buffer, advancing the buffer pointer and count if space is available.
- `append_byte_to_buffer`: Returns without change when the high bit of the input is set.
- `mark_entry_pending_and_queue`: Sets the pending flag bit in the entry and, when a pointer field is empty, fills it.
- `mark_entry_pending_call`: Sets the pending flag bit in the entry, fills an empty pointer field, then calls a library routine through a global base.
- `append_byte_if_room`: Stores the input byte at the buffer pointer and advances both the count and the pointer, but only while the count is below the limit.
- `queue_entry_and_call_library`: Fills an empty pointer field if needed, calls a routine, then calls a library routine through a global base.
- `update_cia_port_state`: Copies a field into a structure byte, clears the flag bit, and changes the CIA port register by the entry's flags.
- `set_cia_port_state`: Copies a different field into a structure byte, sets the flag bit, and changes the CIA port register by the entry's flags.
- `clear_entry_byte_field`: Clears one byte field in the structure to zero.
- `initialize_entry_state_once`: Sets the entry's flag bit when it is not already set, clears two fields, copies a field and subtracts ten from the pointer.
- `start_serial_transfer_entry`: Clears the entry's transfer flag, then when the flag was set and the serial status is idle, it sets another flag, fills an empty pointer and calls a helper.
- `mask_entry_flags_call_library`: Masks one entry word with a stored value, clears bits chosen by the entry's flag byte, then calls a library routine through a global base.
- `mark_entry_ready_call_library`: Sets the ready flag bit in the entry, fills an empty pointer field if needed, then calls a library routine through a global base.
- `reset_entry_and_check_handler`: Clears the entry's first field and its flag bit, then checks the structure's flags before calling the handler from the table if the test passes.
- `reset_entry_state`: Modifies status flags and updates several fields in a structure, including a pointer and a long value, based on specific bit tests.
- `stub_empty_interrupt_hook`: Does nothing and returns; it is an empty interrupt hook with no effect.
- `set_entry_state_thirteen`: Sets the entry's state byte to thirteen when two global flag bits are set.
- `process_serial_data_buffer`: Manages a linked list of serial data entries by popping elements, updating status flags, and decrementing countdown timers.
- `set_entry_countdown_value`: Sets a countdown word in the structure to two hundred, then depending on a sign test and a flag it updates a pointer field.
- `compare_and_update_timer_word`: Compares the input value with a stored field and, when it is larger, sets a global and clears a flag bit, then recomputes a stored word.
- `update_serial_status_bits`: Combines a status word with a bit chosen from a structure byte, when the test allows, and writes the result to a global word.
- `pop_serial_list_entry`: When the count field is nonzero, removes the first entry from a list and returns it, or returns zero when the list is empty.
- `set_global_flag_one`: Sets a global flag word to one and returns, changing nothing else.
- `acknowledge_serial_interrupt`: Sets a global flag word to one, reads and clears a flag byte from the entry, and merges it into a global word.
- `service_serial_transfer_done`: Writes a register value for the serial transfer, runs the handler, copy and clear helpers for the entry, and resets its flags.
- `check_entry_flags_then_handler`: Calls a helper, then tests the entry's flag byte against a mask and calls the handler from the table when the test passes.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `set_flag_and_check_handler`: Sets a flag bit in the structure when a field is nonzero, calls a helper, then tests the entry's flag byte and calls the handler when the test passes.
- `clear_flag_and_check_handler`: Clears a flag bit in the structure, calls a helper, sets a global flag word and then tests the entry's flag byte before calling the handler.
- `acknowledge_interrupt_and_handler`: Writes the global flag words, calls a helper, then tests the entry's flag byte and calls the handler when the test passes.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `drain_list_entries_loop`: Repeatedly removes the first entry from the list and calls a helper on it, until the list is empty.
- `return_immediately`: The routine is empty and performs no operations before returning.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `update_handler_data`: Calculates a bitmask from various status flags and stores it along with a byte and a long value into a structure.
- `process_serial_entry_2`: Updates status flags in a structure and manages a linked list of entries within a device structure.
- `process_serial_entry`: Clears an entry pointer and conditionally updates a structure field, a global variable, and a status bit based on a byte comparison.
- `clear_entry_pointer`: Calls another routine and then sets a long value in a structure to zero.
- `modify_status_flags`: Clears a bit in a status byte and conditionally updates a pointer and a flag if a specific value is found in a linked structure.
- `initialize_structure_fields`: Sets several fields in a structure to specific values and calls another routine to retrieve a return value.
- `update_serial_status`: Modifies bits in a structure and conditionally calls a function through a pointer based on status flags.
- `check_serial_status`: Checks various status flags and, if specific conditions are met, toggles interrupts and calls a function via a pointer.
- `toggle_interrupt_and_call_handler`: Sets a global variable to enable interrupts via the INTENA register, calls a handler, and then sets the variable to disable interrupts.
- `validate_and_copy_serial_buffer`: Validates a sequence of characters and a numeric range within a buffer before copying the data to a destination structure.
- `calculate_bitmask_value`: Computes a bitmask based on a byte value and various flags, then stores the result and two other values into a table.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `set_byte_value`: Stores the value five into a specific offset of a structure.
- `calculate_division_result`: Performs a division based on a value in a structure, stores the result, and modifies status bits and a global variable based on specific character and bit checks.
- `update_division_result`: Performs a division, stores the result in a structure, and modifies status flags based on a character comparison.
- `initialize_serial_request`: Populates a structure with data and pointers, performs multiple function calls through pointers, and assigns a routine to a table entry.
- `configure_serial_request_2`: Initializes a request structure with specific values and pointers, then calls a function via a pointer from a table.
- `populate_structure_data`: Stores several register values into a structure, including a pointer to a global variable and a byte value.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `configure_serial_request`: Initializes a request structure with a global variable pointer and byte values, then calls a library function via a pointer.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `call_library_function`: Loads a library base from a global variable and calls a function at a specific offset within that library.

### gb

- `restore_fpu_state`: Attempts to restore a floating point unit state frame from a memory location.

### parallel.device

- `update_status_flags`: Performs bitwise operations on various memory locations and updates a table based on conditional checks of input values.
- `sync_state_data`: Copies data from one memory structure to another and increments a counter if certain conditions are met.
- `handle_conditional_callback`: Evaluates structure fields to conditionally execute a function via a pointer and modify global status bits.
- `execute_conditional_callback`: Checks a value in a structure and conditionally modifies a global variable and executes a function via a pointer.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `handle_conditional_callback_2`: Evaluates structure fields to conditionally execute a function through a pointer and modifies bits in a global variable and a structure.
- `update_device_context`: Modifies structure fields and manages a linked list of structures while updating status bits and a global variable.
- `initialize_device_context_2`: Updates structure fields, manages a linked list of structures, and modifies status bits and a global variable based on input values.
- `initialize_device_context`: Sets several fields in a structure, modifies a global variable, and performs bitwise operations on memory values.
- `process_parallel_device_request`: Manages a linked list of device requests and updates internal counters and status flags.
- `update_device_counters`: Updates counters and status bits within a structure, conditionally executes a function through a pointer, and performs a loop to compare values in a buffer.
- `reset_device_state`: Copies a value from a structure, clears specific status bits, and conditionally executes a function via a pointer.
- `reset_device_status`: Increments counters in a structure and conditionally clears bits in a structure before calling a function through a pointer.
- `synchronize_structure_data`: Copies data between structures, increments a counter, and updates status flags based on specific bitwise conditions.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `call_if_not_zero`: Checks if a value in a structure is non-zero and calls a function through a pointer if it is.
- `invoke_callback_pointer`: Executes a function through a pointer stored in a structure if the associated index is not zero.
- `traverse_linked_list`: Iterates through a linked list of structures, executing a function for each valid entry, and resets a counter in a structure.
- `clear_word_value`: Stores a zero word into a specific offset of a structure.
- `modify_status_byte`: Sets a specific bit in a byte and stores a value into another byte before calling a function via a pointer.
- `update_global_and_field`: Manipulates bits within a global variable and updates a byte in a structure using a library function call.
- `conditional_copy_data`: Checks a bit in a structure and either copies multiple values from one location to another or sets a byte to one.
- `copy_structure_fields`: Copies a byte and potentially several long words from one structure to another based on a bit flag.
- `process_parallel_request`: Manages a linked list of device requests and updates internal counters and status flags.
- `return_zero_2`: This routine is empty and simply returns zero to the caller.

### printer.device

- `initialize_printer_device`: Calls a routine through a pointer and stores the result in a global variable.
- `call_subroutine`: Calls another routine and returns.
- `manage_linked_list`: Sets a bit in a byte, performs a linked list insertion if a flag is set, and calls a function through a pointer.
- `execute_printer_initialization`: Calls a routine through a pointer, stores the result in a global variable, and updates a structure member with a value from a register.
- `initialize_printer_state`: Attempts to acquire multiple system resources, fills a memory buffer with a specific pattern, and updates several structure members.
- `execute_multiple_callbacks`: Calls a function via a pointer multiple times with varying arguments.
- `traverse_linked_list_nodes`: Iterates through a linked list by repeatedly calling another routine through a pointer until a null pointer is encountered.
- `insert_linked_list_node`: Performs a linked list insertion and sets a bit in a byte within a structure.
- `set_status_bit`: Sets the lowest bit of a byte in a structure.
- `clear_status_bit`: Clears the lowest bit of a byte in a structure.
- `iterate_linked_list`: Iterates through a linked list by repeatedly calling a routine through a pointer until a null pointer is encountered.
- `null_operation`: This routine is empty and performs no actions.
- `execute_recursive_call`: The routine performs a direct recursive call to itself and then returns.
- `initialize_event_structures`: Initializes multiple structures with pointers and offsets, then enters a loop to process events and update status bits.
- `process_event_loop`: Iterates through a linked list to execute routines through pointers while modifying status bits in a structure based on returned values.
- `handle_status_updates`: Checks a bitmask to iterate through a list and updates specific bits in a structure based on values returned from another routine.
- `call_bit_mask_clear_loop`: Calls a routine that clears a specific bit within a data value based on a bit index stored in a structure, then continues iterating through a list.
- `update_structure_state`: Sets bits in a status byte and stores two 32-bit values and a 16-bit value into a structure based on a return value.
- `clear_bit_in_register`: Clears a specific bit within a value held in a register using a bit index retrieved from a structure field.
- `update_structure_fields`: Sets a bit in a byte within a structure and stores two 32-bit values and a 16-bit value into other offsets of that structure.
- `process_list_loop`: Iterates through a list, checking a status byte and performing bitwise operations on a structure field based on comparisons with specific offsets.
- `process_callback_loop`: Iterates through a list of callbacks, executing them through pointers and modifying specific bits in a status byte based on comparison results.
- `update_callback_status_bits`: Clears specific bits in a status byte within a structure based on comparison results.
- `process_status_loop_2`: Iterates through a loop that checks a status byte and executes routines through pointers while clearing specific bits in a structure.
- `process_status_loop`: Checks a status byte in a structure and enters a loop that executes routines through pointers and modifies bits in a structure.
- `initialize_entry_data`: Writes a 16-bit value, a 32-bit value, and zeroed bytes into a structure before calling a routine via a pointer.
- `clear_status_byte`: Sets a byte at a specific offset to zero and calls a routine through a pointer found in a structure.
- `execute_callback_logic`: Checks a character in a structure and, if it does not match a specific value, executes two routines through pointers.
- `execute_callback_sequence`: Calls a routine through a pointer in a structure and then calls another routine using a value from a global base.
- `manage_list_state`: Checks a status byte and enters a loop that executes multiple routines through a global base while modifying bits in a structure based on comparison results.
- `manage_callback_status`: Executes a series of routines via pointers and updates a status byte within a structure based on specific comparison values.
- `manage_pending_calls`: Checks a status byte and enters a loop that executes routines through pointers and modifies bits in a structure based on a value from another.
- `manage_status_bits`: Checks a status byte and enters a loop that executes routines through pointers and modifies bits in a structure based on a value from another routine.
- `monitor_list_state`: Checks a bitmask in a structure and enters a loop that executes routines through pointers and modifies bits in a structure.
- `clear_status_recursively`: Updates a pointer based on a global variable and recursively clears specific bits in a status byte.
- `clear_status_bits_2`: Clears specific bits in a status byte and recursively calls itself if certain bits remain set.
- `clear_status_bits`: Clears specific bits in a structure's status byte and recursively calls itself if certain bits were previously set.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `update_linked_list_and_lookup_table`: Performs a linked list insertion and sets a status bit before copying a null-terminated string into a lookup table.
- `reset_structure_field`: Sets a 32-bit field in a structure to zero and conditionally updates it from another field based on a routine return value.
- `execute_recursive_calls`: Performs two consecutive calls to another routine before returning.
- `execute_recursive_calls_2`: The routine performs two consecutive recursive calls to itself before returning.
- `dispatch_pending_entry`: Checks a condition via a routine call, and if met, populates a structure with two parameters and executes several routines through a global base.
- `parse_and_reorder_string_buffer_2`: Calculates the length of a null-terminated string and copies its characters into a lookup table.
- `parse_and_reorder_string_buffer`: Calculates the length of a null-terminated string and reorders its characters into a lookup table based on specific character matches.
- `search_buffer_for_character_2`: Iterates through a data buffer to locate a specific character match.
- `search_buffer_for_character`: Iterates through a data buffer to locate a specific character match.
- `process_string_buffer`: Iterates through a data structure to search for specific character matches within a buffer.
- `search_buffer_for_character_3`: Iterates through a data buffer to locate a specific character match.
- `copy_and_clear_memory`: Copies data from one memory location to another using an incrementing index and then sets a value at a specific offset to zero or another value based on a condition.
- `clear_memory_offset`: Stores a zero at a specific offset within a memory location pointed to by a register.
- `process_stream_input`: Processes input data by performing character substitution and conditional checks.
- `transform_character_stream_3`: Iterates through a stream of data, performing character substitution based on a lookup table and conditional logic.
- `map_character_stream_2`: Performs character substitution by comparing input bytes against a table and applying mapping rules before shifting the table elements.
- `map_character_stream`: Performs character substitution by comparing input bytes against a table and applying specific mapping rules based on index values.
- `transform_character_stream_2`: Performs character substitution by looking up values in a table and shifts elements within a global array.
- `transform_character_stream`: Performs character substitution by looking up values in a table based on a counter and a data stream, then shifts elements within a global array.
- `transform_character_data`: Iterates through a data stream, performing lookups in a table to transform characters.
- `process_character_data`: Performs a complex lookup and transformation of data by comparing values against a table and updating a destination buffer with calculated characters.
- `transform_character_stream_data`: Performs complex character substitution and stream transformation by comparing input bytes against a lookup table and applying conditional mapping rules.
- `process_ansi_stream`: Iterates through a character stream to identify and interpret ANSI escape sequences and control codes.
- `parse_ansi_escape_sequences_7`: Iterates through a character stream to identify and interpret ANSI escape sequences and control codes.
- `parse_ansi_control_sequences`: Iterates through a character stream to identify and interpret ANSI escape sequences and control codes.
- `process_ansi_control_sequences`: Iterates through a character stream to identify and interpret ANSI escape sequences and control codes.
- `parse_ansi_escape_sequences_5`: Parses a stream of characters to identify and process ANSI-style escape sequences and control codes.
- `expand_ansi_control_sequences`: Parses a character stream to identify and expand ANSI-style escape sequences and control codes into a destination buffer.
- `parse_ansi_escape_sequences_6`: Processes a stream of characters to interpret ANSI-style escape sequences and control codes.
- `parse_escape_sequences`: Processes a stream of characters to handle control codes and escape sequences.
- `parse_escape_sequences_2`: Processes a stream of characters to handle escape sequences and control codes.
- `expand_ansi_escape_sequences`: Parses a stream of characters to identify and expand ANSI escape sequences and control characters into their corresponding literal values.
- `process_ansi_escape_sequences`: Parses a string for ANSI escape sequences and control characters, converting numeric escape codes into their corresponding character values.
- `parse_escape_sequences_3`: Iterates through a string to identify and process backslash-prefixed escape sequences and numeric values.
- `parse_ansi_escape_sequences_4`: Iterates through a string to identify and interpret ANSI escape sequences, converting character-based numeric values into integers.
- `parse_ansi_escape_sequences_3`: Iterates through a string to identify and interpret ANSI escape sequences, converting character-based numeric values into integers.
- `parse_ansi_escape_sequences`: Processes a string to interpret ANSI-style escape sequences, converting numeric characters into integer values and handling special control characters.
- `parse_escape_sequences_4`: Parses a string to convert numeric characters into integer values and processes escape sequences.
- `parse_ansi_escape_sequences_2`: Iterates through a string to identify and interpret ANSI escape sequences, converting character-based numeric values into integers.
- `parse_ansi_control_sequences_2`: Iterates through a character stream to identify and interpret ANSI escape sequences and control codes.
- `update_global_values`: Subtracts the difference between two global variables from a third global variable and clears a fourth global variable.
- `copy_stack_data`: Copies a sequence of bytes from a local stack buffer to a destination address using a global index.
- `copy_bytes_conditionally`: Copies a sequence of bytes from a source to a destination if a counter is within a specific range, otherwise updates global variables and potentially calls another routine.
- `decrement_global_pointer`: Decrements a global variable by one.
- `copy_bytes_with_increment`: Copies a sequence of bytes from a source to a destination while incrementing a global pointer for each byte.
- `copy_memory_block`: Copies a sequence of bytes from a source address to a destination address, incrementing a global pointer for each byte.
- `parse_numeric_stream`: Converts a stream of ASCII digit characters into integer values and stores them in a destination buffer.
- `parse_input_buffer`: Converts numeric characters into values and stores them in a destination buffer while handling specific control characters.
- `parse_input_characters_2`: Processes input characters to convert numeric digits into values and handles specific control characters by updating a destination buffer.
- `parse_input_characters`: Processes input characters by converting numeric digits to values and handling specific control characters like spaces or delimiters.
- `parse_numeric_input_stream`: Converts ASCII digit characters into their integer equivalents and stores them in a destination buffer.
- `increment_and_fetch_table_value`: Increments a value through a pointer and returns a value from a global table based on that incremented value.
- `return_register_value`: Returns the value currently held in a data register.
- `decrement_and_verify_pattern`: Decrements a value and checks if a specific bit pattern exists in a global table.
- `decrement_and_check_table`: Decrements a value pointed to by a register and continues looping as long as the value is positive and a specific bit pattern is found in a table.
- `initialize_global_state`: Resets several global variables and performs a comparison check before setting a global flag and specific byte values.
- `copy_data_to_globals`: Copies six 16-bit values from a structure into a set of global variables.
- `compare_global_values`: Compares six 16-bit values from a structure against previously stored global values and returns zero if they all match, or all ones otherwise.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `dispatch_conditional_logic`: Evaluates a short value and a series of bytes to conditionally call another routine with different parameters.
- `dispatch_conditional_routines`: Calls various versions of another routine with different parameters based on the state of registers and return values.
- `format_string_data`: Retrieves a string from a table based on a specific index and copies it to a destination.
- `process_formatted_string`: Retrieves a string from a table based on a specific index and copies it to a destination.
- `copy_processed_string`: Copies characters from a source to a destination while handling specific control characters and adjusting the source pointer.
- `copy_string_data`: Copies a null-terminated string from one memory location to another and conditionally appends a byte.
- `copy_string_data_2`: Copies a null-terminated string from a source to a destination.
- `set_flag_and_call`: Checks if a byte at a specific offset in a passed pointer is zero, sets it to one if so, and then calls another routine.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `validate_clipping_rect_bounds`: Validates a rectangular area by checking if its dimensions and coordinates fit within the boundaries of a specified source buffer or screen region.
- `calculate_clipping_bounds`: Calculates the dimensions and position of a rectangular area by subtracting coordinate pairs and compares them against existing boundaries.
- `calculate_clipping_area_dimensions`: Calculates the width and height of a rectangular area by subtracting coordinate pairs and compares these dimensions against existing boundary constraints.
- `update_clipping_rectangle_bounds_4`: Calculates the width and height of a rectangle and compares them against existing boundary constraints.
- `update_clipping_rectangle_bounds_3`: Calculates and updates the minimum and maximum coordinate boundaries for a rectangular area by comparing current values against a set of provided coordinates.
- `update_clipping_rectangle_bounds_2`: Calculates the minimum and maximum coordinate boundaries for a rectangular area by comparing current values against a set of provided coordinates.
- `update_clipping_rectangle_bounds`: Calculates and updates the minimum and maximum coordinate boundaries for a rectangular area.
- `update_clipping_rect_bounds`: Calculates and updates the minimum and maximum coordinate boundaries for a rectangular area.
- `update_clipping_rect_bounds_2`: Calculates and updates the minimum and maximum coordinate boundaries for a rectangular area.
- `update_clipping_bounds_rect_2`: Calculates and updates the minimum and maximum coordinate boundaries for a rectangular area.
- `update_clipping_bounds_rect`: Calculates and updates the minimum and maximum coordinate boundaries for a rectangular area.
- `update_clipping_bounds_3`: Calculates and updates the minimum and maximum coordinate boundaries for a rectangular area.
- `update_clipping_bounds_2`: Calculates and updates the minimum and maximum coordinate boundaries for a rectangular area.
- `update_clipping_bounds`: Calculates and updates the minimum and maximum coordinate boundaries for a rectangular area.
- `update_clipping_rectangles`: Calculates and updates bounding box coordinates by comparing input values against existing limits and applying bitwise shifts.
- `calculate_color_palette_table`: Calculates a color palette by extracting nibbles from source data and performing bitwise inversion and masking.
- `generate_color_palette_table_4`: Calculates a color palette by extracting nibbles from source data and performing bitwise inversion and masking.
- `generate_color_palette_table_3`: Calculates a color palette by extracting nibbles from source data and performing bitwise inversion and masking.
- `generate_color_palette_table_2`: Populates a color table by extracting nibbles from source data and performing bitwise inversion and masking.
- `build_color_palette_table`: Populates a color table by extracting nibbles from source data and performing bitwise inversion and masking.
- `generate_color_palette_table`: Populates a color table by extracting nibbles from source data and performing bitwise inversion and masking.
- `unpack_and_invert_color_data`: Extracts nibbles from a source value to populate a table of four-bit color components and performs bitwise inversion and masking on a series of 32-bit color values.
- `validate_timer_callback_timing`: Checks if a global variable is zero and evaluates whether a value from a table satisfies a modulo condition.
- `verify_timer_callback_conditions`: Checks if a global variable is zero and evaluates whether a value from a table satisfies a modulo condition.
- `validate_timer_callback_condition`: Checks if a global variable is zero and evaluates whether a value from a table satisfies a modulo condition.
- `evaluate_timer_callback_condition`: Checks if a global variable is zero and evaluates a modulo condition against a value retrieved from a table.
- `validate_timer_and_execute_callback`: Checks if a global variable is zero and evaluates a modulo condition against a value from a table.
- `check_timer_condition`: Checks if a global counter is a multiple of a value retrieved from a table and calls another routine if a condition is met.
- `check_timer_interval`: Checks if a global counter is a multiple of a value stored in a table.
- `call_empty_routine`: Calls another routine with two zero arguments and returns.
- `calculate_weighted_value`: Calculates a value based on three input bytes using a weighted sum formula or finds the minimum of the three depending on a global variable and a bit flag.
- `find_minimum_byte`: Checks a bit in a global variable and, if set, finds the smallest byte value among three provided bytes.
- `update_pointer_state`: Calculates an index using global variables to retrieve a value from a table.
- `resolve_pixel_data`: Reads a pixel value and uses it to index a table to update several global pointers.
- `update_pointer_table_5`: Retrieves a value from a table using an index and updates four global pointers by adding a value derived from a global variable.
- `update_pointer_table_4`: Updates a global variable and increments four global pointers by a specific byte value.
- `increment_pointer_table_elements`: Updates a global variable and four global pointers by adding a byte value to them.
- `update_pointer_table_3`: Updates a global variable and four global pointers by adding a byte value to them.
- `update_pointer_table_conditional`: Updates a global variable and four global pointers by adding a byte value to them after calling another routine.
- `update_pointer_table_direct`: Sets a global variable and updates four global pointers by adding a byte value to them after calling another routine.
- `update_pointer_table_offsets`: Updates a table of four global pointers by adding a byte value to each.
- `update_pointer_table_2`: Calculates a value using another routine and updates four global pointers by adding a byte value to them.
- `update_pointer_table`: Loads a value from a table using an index and updates four global pointers by adding a base address to them.
- `calculate_and_update_pointers`: Calculates a value based on the product or sum of two globals and updates four global variables by calling another routine multiple times.
- `validate_table_entries`: Iteratively checks a series of table entries using a comparison routine and, if a match is found, stores a result into a global variable via another routine.
- `compare_and_store_results`: Compares values using three different table entries and stores the result of a routine call into a pointer.
- `compare_and_store_result`: Compares values using two different table entries and stores the result of a function call into a pointer.
- `fetch_table_value`: Calls another routine to compare values and, if a non-zero result is returned, calls a third routine to store a value into a pointer.
- `clear_register_value`: clears a register by moving zero into it and returns.
- `increment_and_check_timer`: Increments a global counter and checks if it is a multiple of a value from a table.
- `store_function_result`: Calls another routine and stores the returned value into a location pointed to by a register.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `traverse_nested_tables`: Performs a multi-level nested loop search through hierarchical tables using scaled indexing and pointer increments.
- `search_nested_tables`: Performs a nested loop search through a table using a scaled index, updating a global variable and returning a value from another routine.
- `search_non_zero_values`: Performs a nested loop to search for non-zero values in a table and updates a pointer's target with the result.
- `execute_nested_loops`: Performs complex nested loops involving multiple global counters and conditional calls to other routines until a specific condition is met.
- `execute_nested_search`: Performs a complex nested loop structure that searches for non-zero values in a table.
- `execute_nested_search_2`: Performs a nested loop search through a table to find non-zero values while updating various counters and pointers.
- `traverse_nested_search_tables`: Performs a multi-level nested loop search through hierarchical tables using scaled indexing.
- `traverse_linked_table_entries`: Iterates through a table using a global variable as an offset and performs a secondary search loop that decrements a counter.
- `traverse_table_entries_3`: Iterates through a table using a global variable as an offset and performs a secondary search loop that decrements a counter.
- `search_with_decrementing_counter`: Executes a loop that decrements a counter while searching for a non-zero value to update a pointer.
- `search_with_decrement`: Executes a nested loop that includes a decrementing counter.
- `traverse_table_entries`: Iterates through a table, updating global counters and pointers while executing routines based on comparison results.
- `traverse_table_entries_2`: Iterates through a table while decrementing a counter and performing nested searches based on comparison results.
- `search_linked_table_entries`: Iterates through a table using a global variable as an offset and performs a secondary search loop that decrements a counter.
- `search_decrementing_table`: Executes a nested search through hierarchical tables while iterating through a loop that decrements a value.
- `process_iterative_data`: Iterates through a structure using a nested loop pattern, calling other routines to update counters and pointers until a limit is reached.
- `decrement_and_search`: Decrements a value in a table and, if it reaches zero, increments a counter and performs a search through a table.
- `search_with_inner_decrement_2`: Implements a nested search structure that decrements a value in a table until it reaches zero.
- `search_with_inner_decrement`: Implements a nested search where an inner loop decrements a value until it reaches zero.
- `search_with_inner_decrement_3`: Searches a table and decrements a value in a table, triggering a secondary loop when that value reaches zero.
- `decrement_and_search_table`: Decrements a value at a pointer and, if it reaches zero, increments a counter and performs a nested search through a table.
- `traverse_nested_search_tables_2`: Performs a multi-level nested loop that iterates through a table structure using decremented counters and pointer increments.
- `process_indexed_data`: Executes a nested loop structure where an inner loop's limit is determined by a routine call and an index is incremented by a global variable.
- `search_with_nested_loops_2`: Executes a complex search involving multiple nested loops and decrements a value in a table when certain conditions are met.
- `search_with_reset_counters`: Performs a search using a decrementing loop while explicitly resetting several global counters at the start of the routine.
- `decrement_table_counters`: Traverses a structure to decrement a counter and increment another, executing routines if a specific limit is reached.
- `search_with_nested_loops`: Traverses multiple nested loops to search for a non-zero value while updating various counters and table entries.
- `process_table_loops`: Decrements a table value and executes a loop structure that traverses a table and performs nested searches.
- `accumulate_scaled_values`: Iteratively adds a doubled value to a pointer's target until the target is no longer less than one, then subtracts a doubled second parameter from that target.
- `compare_table_entry`: Compares a provided byte against either a value from a table indexed by bitwise operations or a value derived from a global variable.
- `compare_table_value`: Performs a comparison between a byte retrieved from a table using bitwise-derived indices and a provided byte.
- `negate_boolean_results`: Calls two different routines and returns the negation of the first return value based on the status of a second return value.
- `perform_32bit_division`: Performs 32-bit division and modulo operations on a 32-bit value using a divisor.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `recursive_call`: The routine calls itself and then returns.
- `call_and_return_d1`: The routine calls another routine and returns the value found in the D1 register.
- `negate_on_flag`: The routine calls another routine, checks a second return value, and returns the negated first return value if the flag is non-zero.
- `negate_on_flag_extended`: Calls another routine and returns its first return value, or the negation of that value if the second return value is non-zero.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `sync_string_buffer_2`: Compares a string from a table with a global buffer.
- `sync_string_buffer`: Compares a string from a table with a global buffer and, if they match, copies the source string into the global buffer.
- `return_error_code`: Returns a value representing negative one.
- `validate_and_update_string_buffer`: Compares two null-terminated strings and, if they match, updates a global buffer with data from a source pointer.
- `compare_and_copy_strings`: Compares two strings and, if they match, performs memory operations and copies a string into a global variable.
- `process_string_comparison`: Iterates through two strings to compare characters, manages memory allocation via a pointer in a structure, and copies a string into a global buffer.
- `execute_dos_sequence`: Performs a sequence of operations involving locking, loading segments, and copying a string into a global buffer.
- `call_dos_functions`: Calls two different library routines and returns a value of negative one.
- `execute_callback_and_recurse`: Checks a global value, calls a function pointer from a table, and then calls itself with a value from a pointer.
- `compare_normalized_chars`: Compares two characters after optionally subtracting thirty-two from them if they fall within a specific ASCII range.
- `return_zero_short`: An empty routine that returns zero.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `return_zero`: An empty routine that simply returns zero.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d0, then returns.

### arp.library

- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 34 into d0, then returns.
- `manage_arp_nodes`: As library code, this routine traverses a linked list, performs indirect function calls, and manages node allocation and updates.
- `manage_arp_linked_nodes`: As library code, it traverses and modifies a linked list of nodes, allocating new nodes and updating records through indirect function calls.
- `update_library_record`: As library code, it modifies a field in a record using a value from a register, calls several indirect functions, and conditionally updates further record fields based on a returned value.
- `retry_arp_step_until_flag`: As library code, it repeatedly calls a function via a pointer until a zero flag is set, then increments a 16-bit value in a library record.
- `clear_arp_link_and_call`: Within the arp shared library, clears a pointer field in the given record and calls an entry through a library pointer stored in the base record.
- `allocate_arp_linked_node`: In the arp shared library, allocates a memory block with AllocMem when given a nonzero size, and stores a link value offset by four in its first word.

### Spy vs Spy - The Island Caper (1989)(Wicked Software).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
