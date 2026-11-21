# Game 3731 (fruitmania (1989)(bbs de saen)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03731). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906832 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 42 of 42 named (0 author symbols), 42 of 42 described (9 by rule, 33 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 427 of 427 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 512 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1248 of 1248 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 10716 |
| data | 541300 |
| unused | 354816 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Fruitmania (1989)(BBS de Saen).adf:bootblock | 1 | 60 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 72 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1027 | 32 |
| Fruitmania (1989)(BBS de Saen).adf bytes 532480-532992 (code never loaded by a reference run) | 1 | 89 | 1 |

## Cases handled

- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: custom-format (non-DOS) disk, 1065 non-empty blocks: matched against track-loaded segments
- runtime segment an address (play-rs-m1500): 71 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 310 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: bytes 532480-532992 look like code no run loaded: recovered position-independently (load address unknown)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: 1 logged loads cover 512 bytes of the image
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: 9 track-loaded blocks found in recovered runtime segments and classified through them
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 2-23 (bytes 1024-12288) hold ASCII text
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 44-78 (bytes 22528-40448) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.84)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 80-104 (bytes 40960-53760) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.40)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 110-111 (bytes 56320-57344) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.63)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 113-114 (bytes 57856-58880) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.63)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 118-121 (bytes 60416-62464) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.72)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 123-124 (bytes 62976-64000) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.84)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 130-132 (bytes 66560-68096) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.12)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 137-138 (bytes 70144-71168) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.43)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 140-143 (bytes 71680-73728) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.12)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 148-152 (bytes 75776-78336) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.14)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 158-161 (bytes 80896-82944) outside the filesystem hold data, not code (opcode density 0.010, entropy 2.88)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 170-171 (bytes 87040-88064) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.53)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 190-190 (bytes 97280-97792) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 198-200 (bytes 101376-102912) outside the filesystem hold data, not code (opcode density 0.029, entropy 4.66)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 220-222 (bytes 112640-114176) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.15)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 224-454 (bytes 114688-232960) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.33)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 462-477 (bytes 236544-244736) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.30)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 484-676 (bytes 247808-346624) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.62)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 681-686 (bytes 348672-351744) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.43)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 694-694 (bytes 355328-355840) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.61)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 704-722 (bytes 360448-370176) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.67)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 724-790 (bytes 370688-404992) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.71)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 792-809 (bytes 405504-414720) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.64)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 812-829 (bytes 415744-424960) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.44)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 832-849 (bytes 425984-435200) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.22)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 852-869 (bytes 436224-445440) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.39)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 872-874 (bytes 446464-448000) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.94)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 876-880 (bytes 448512-451072) outside the filesystem hold data, not code (opcode density 0.006, entropy 1.29)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 892-892 (bytes 456704-457216) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.81)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 902-974 (bytes 461824-499200) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.07)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 976-980 (bytes 499712-502272) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.93)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 993-994 (bytes 508416-509440) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.56)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 996-997 (bytes 509952-510976) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.20)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 999-1000 (bytes 511488-512512) outside the filesystem hold data, not code (opcode density 0.008, entropy 0.85)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 1002-1002 (bytes 513024-513536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.98)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 1012-1022 (bytes 518144-523776) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.17)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 1024-1026 (bytes 524288-525824) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.95)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 1028-1033 (bytes 526336-529408) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.16)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 1044-1051 (bytes 534528-538624) outside the filesystem hold data, not code (opcode density 0.012, entropy 3.70)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 1056-1253 (bytes 540672-642048) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.57)
- fruitmania (1989)(bbs de saen)/Fruitmania (1989)(BBS de Saen).adf: blocks 1265-1275 (bytes 647680-653312) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.68)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `boot_device_initialization` (bootblock): Initializes device parameters including addresses, sizes, and status flags, then calls system functions to perform I/O operations and handle device setup. Kind: track/checksum verification. Pass path: The check passes if exec_DoIO succeeds and the high bit of uRam01000138 is not set (i.e., no error). If the high bit IS set, u0__sub_00BC() is called, indicating a protection check failure or special handling..
- `call_do_i_o` (bootblock): Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller. Kind: trackdisk.device I/O check. Pass path: The call_do_i_o routine performs a DoIO operation using trackdisk.device, likely checking for valid disk geometry or presence of specific tracks/data. When successful, it would continue execution normally (e.g., the 'jsr -an address(a6)' in sub_0.

## Functions


### Fruitmania (1989)(BBS de Saen).adf:bootblock

- `boot_device_initialization`: Initializes device parameters including addresses, sizes, and status flags, then calls system functions to perform I/O operations and handle device setup.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a2, so the target is only known at run time.
- `initialize_device`: Initializes a device by clearing a global variable, setting a status byte, and calling system functions to find a task, add a port, and open a device.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `frag_movea_bsr_lea_jsr_rts`: Short code fragment that copies a fixed memory address to a6, then calls another routine, then loads the address of a location in the code into a1, then calls another routine, then loads the address of a location in the code into a1, then calls another routine, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_jsr_btst_bne_rts`: Short code fragment that copies 34048 to a fixed memory address, then calls another routine (2 times in a row), then copies a fixed memory address to a fixed memory address, then calls another routine, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then returns.
- `blit_graphics`: Performs multiple bit-block transfers using hardware registers to draw graphics on the screen by calculating memory addresses and setting up transfer parameters.
- `clear_memory_blocks`: Clears two memory blocks starting at specific addresses, then opens a library and copies data between memory locations using a loop.
- `frag_cmpi_beq_btst_bne_movea_move_rts`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was zero, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then copies a fixed memory address to a0, then copies 58444 to the memory at an offset from a0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_input_and_update_dma`: Reads input from hardware registers including keyboard serial, joystick fire buttons, and mouse buttons, then updates DMA control flags based on the input values.
- `update_display_flags`: Sets bit 0 of a global variable, clears bits 1 and 2, and updates a counter at offset 10 of a table.
- `process_animation_frame`: Checks a flag bit, sets another flag, reads a byte from memory, clears that byte, and if the memory value is zero, writes back the original byte.
- `handle_game_state`: Processes game state changes by calling various routines, updating global variables, and performing arithmetic operations on memory values before clearing a counter.
- `frag_btst_bne_rts_bclr_jsr`: Short code fragment that tests bit 2 of a fixed memory address, then branches if the last result was not zero, then returns, then clears bit 2 of a fixed memory address, then calls another routine, then returns.
- `update_graphics_buffer`: Calls a graphics function multiple times with specific parameters, copies data between memory locations, and then calls another graphics function with different parameters.
- `check_collision`: Compares coordinates with stored values to determine if a collision has occurred, returning a calculated value based on position comparisons.
- `process_input_events`: Checks for specific input conditions from hardware registers, updates counters and flags based on comparisons, and calls graphics functions to render sprites or play sounds.
- `draw_sprite`: Calculates sprite positions and calls a graphics function to draw sprites using parameters from global variables and memory locations.
- `manage_sprite_timers`: Manages sprite timers by decrementing countdown values in a table, calling rendering routines when timers reach zero, and performing loop operations with delay counts.
- `update_sprite_data`: Modifies sprite data by swapping bits, performing arithmetic operations, and calling graphics functions to update sprite display properties.
- `frag_movem_movea_move_clr_addi_jsr_rts`: Short code fragment that saves registers on the stack, then copies an address to a0, then copies 2000 to d0, then clears d1, then adds 350 to d1, then calls another routine, then restores saved registers from the stack, then returns.
- `call_enable`: Wrapper that calls the exec library function Enable and returns its result to the caller.
- `initialize_video_tables`: Initializes multiple video-related tables by copying data from fixed memory locations to designated output addresses using looped word assignments.
- `update_game_counter`: Increments a global counter and performs conditional operations based on its value, including setting bits in another global variable and calling animation processing routines.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `handle_animation_frame`: Processes animation frame data by reading from a table, calculating memory offsets, and calling a graphics function to render the current frame.
- `update_counter_and_render`: Increments a counter, updates related data structures, and calls a graphics function to render updated animation frames based on calculated values.
- `update_counter_byte`: Reads a byte from a table, adds a constant to it, and stores the result back to the same table location.
- `store_video_position`: Reads video position data from hardware registers, performs bit manipulation, and stores the processed values in a memory table.
- `render_number_string`: Converts a number to its decimal string representation and calls a graphics function multiple times with calculated parameters to render the string.
- `play_sound_sample`: Configures audio hardware to play a sound sample by setting up DMA channels, audio registers, and waiting for a specified duration before cleanup.
- `convert_number_to_string`: Converts a long integer value into its decimal string representation by repeatedly dividing by powers of ten and storing the resulting digits in memory.
- `read_joystick_input`: Reads joystick movement data from hardware registers, calculates differences between current and previous positions, and updates global variables with adjusted values.
- `update_input_flags`: Reads input status from a global variable, modifies it based on bit tests, and stores the results into a structure starting at address a fixed address.
- `update_game_logic_2`: Updates game logic by decrementing a counter and processing animation data based on its value.
- `manage_animation_counter`: Decrements a counter value, resets it when zero, increments another counter, and processes animation data based on the current state.
- `update_game_logic`: Processes game state updates by reading hardware registers, performing arithmetic operations on memory values, and calling graphics rendering routines.
- `initialize_display_buffer`: Clears a display buffer by writing zeros to its elements, updates counters, and sets a specific value in memory at an offset calculated from the counter.
- `setup_graphics_context`: Initializes graphics parameters including coordinates, dimensions, and color values, then calls a system function to set up the graphics context.
- `update_score_display`: Updates a score display by processing a numeric value from a global variable, calculating a new position, and calling a rendering routine multiple times.
- `handle_input_polling`: Polls keyboard input by reading from hardware registers, processes input states through a series of conditional checks, and updates game state based on input conditions.

### Fruitmania (1989)(BBS de Saen).adf bytes 532480-532992 (code never loaded by a reference run)

- `initialize_audio_dma`: Sets up audio DMA channels by configuring register values, initializes sound parameters, and performs multiple operations to prepare audio output.
