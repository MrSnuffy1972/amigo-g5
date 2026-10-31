# Game 608 (E-Motion): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00608). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904784 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 45 of 45 named (0 author symbols), 45 of 45 described (1 by rule, 44 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 895 of 895 (100.00%) |
| Independent decoder cross-check (Ghidra against capstone) | 1263 of 1263 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 345862 |
| code | 4666 |
| data | 42597 |
| padding | 1707 |
| unused | 509952 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| E-Motion.adf:e_motion | 3 | 363 | 16 |
| E-Motion.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 382 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 504 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |

## Cases handled

- runtime segment an address: 295 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 451 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- E-Motion/E-Motion.adf:e_motion: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Copy protection found

- none found

## Functions


### E-Motion.adf:e_motion

- `run_game_entry`: The routine calls decode_animation_frames and then initialize_game_state, and returns zero.
- `decode_animation_frames`: The routine copies a block of bytes from a static table into a RAM buffer and then repeatedly reads input bits, updates a counter, and writes bytes from another static table into the buffer backwards while calling set_player_direction and wait_for_input_bits.
- `set_player_direction`: The routine reads a word from the address in A0 minus four, stores a byte derived from that word together with the high byte of D7 into the word pointed to by A5, and returns the read word shifted right by one with the sign bit set.
- `wait_for_input_bits`: The routine repeatedly shifts the word in d0 right by one bit and, when the value becomes zero, calls set_player_direction and copies the short result from d1 into a local counter.
- `initialize_game_state`: The routine clears custom registers, programs display window and bitplane control registers, initializes sprite cop lists, installs interrupt handlers, runs game loops and waits for input, then copies palette data to chip memory and executes it.
- `install_interrupt_handlers`: The routine copies three long words from the address of copy_palette_data to the interrupt vector area and then calls the installed handler at that area.
- `copy_palette_words`: The routine copies four 32-bit words from the source pointer in A0 to the destination pointer in A1, decrementing a loop counter in D7.
- `check_and_close_window`: The routine calls a function through a pointer stored at an offset from a6, tests the returned integer, and if non-zero calls two further functions through pointers at other offsets from a6, passing the first result to the second and its result to the third.
- `run_game_loop`: The routine closes the window, runs the game tick loop, decodes and applies the palette, then performs a fade by waiting for scanlines and setting the palette fade fifteen times before returning.
- `run_game_loop_2`: The routine calls decode_and_apply_palette, then performs a sixteen-iteration loop that waits for a scanline and sets the palette fade each iteration, decrementing a counter until it wraps.
- `wait_for_scanline_loop`: The routine sets a short counter and repeatedly calls wait_for_scanline, decrementing the counter each iteration, until the counter reaches -1.
- `decode_palette_and_write_copperlist`: The routine walks a chunked resource area, decodes a palette chunk into a copper list buffer, then repeatedly decodes run-length blocks from a data chunk into a destination buffer, using the bytes as literal copies and negative repeat counts.
- `wait_scanlines_and_fade_palette`: The routine waits for scanlines and updates the palette fade repeatedly.
- `fade_palette_to_black`: The routine reads 32 palette entries from chip memory and writes them to the Amiga color registers.
- `wait_for_scanline`: The routine reads the vertical position register and spins until the value is greater than a lower bound and then spins until the value is less than an upper bound, effectively waiting for a specific scanline range.
- `init_sprite_cop_list`: The routine fills a short array with a repeating pattern of control words and pointer values derived from an initial register value, then writes a terminator pair and stores the base address and final pointer value in two global locations.

### E-Motion.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_game_state_handler`: The routine stores a pointer to initialize_game_state_2 in a global variable and then triggers a trap.
- `initialize_game_state_two`: The routine calls initialize_game_state_3, copy_code_bytes, update_game_state, func_0x00070022 and initiate_game_loop in that order and then returns.
- `delay_frame_count`: The routine loads a short with a negative constant and decrements it in a loop until it reaches -1, then returns zero.
- `copy_code_bytes`: The routine copies a block of bytes from a fixed source location to the address of initialize_input_interrupt, decrementing a counter until it reaches -1.
- `initialize_game_state_3`: The routine sets up several work areas in RAM, fills tables with constant values, runs a small loop that builds a byte table from a string and derives offsets from it, calls copy_word_pairs_to_buffer twice and load_animation_frames, copies a block of code into the low memory area, and writes a few bytes to custom hardware registers.
- `load_animation_frames`: The routine copies a small block of bytes from a fixed source area to a fixed destination area and then copies a fixed 144-byte block from a fixed source area to a fixed destination area.
- `copy_word_pairs_to_buffer`: The routine copies word pairs from D0 into the buffer pointed to by A0.
- `wait_vblank_update_state_copy`: The routine repeatedly waits for vertical blank, updates HUD digits, bob animation and animation frames while reset_game_state is non-zero, then copies a small block of bytes from a fixed source to a fixed destination and writes four bytes from a fixed source to fixed locations.
- `reset_game_state`: The routine conditionally writes a constant word pattern to several global locations based on the input registers and two memory flags, then stores a jump instruction word and a function pointer, and ends with a trap.
- `set_game_state`: The routine stores a byte value into a memory location and then returns.
- `wait_for_vblank`: The routine polls the vertical position register until the value in the range of the register equals the value that indicates the vertical blank interval, then returns.
- `update_hud_digit_display`: The routine updates a digit counter and a set of screen positions based on the counter, then selects a digit bitmap from a table and copies it to a buffer using the current digit value and a flag from CIAA_PRA and POTGOR. It finally copies a small block of data to a fixed location and returns.
- `update_animation_frames`: The routine increments a palette index with wrap-around and copies eight 16-bit palette entries from a table to a fixed destination.
- `update_emotion_palette`: The routine updates a palette-related counter and, when a condition is met, copies palette data and writes eight entries from a source table into a destination table with a fixed stride, then sets several global variables and waits for a hardware flag to clear before returning.
- `update_bob_animation`: The routine updates a bob animation counter and, when the counter wraps, calls copy_palette_data_2 and copies eight palette entries from a table to a fixed destination using a stride.
- `copy_palette_data_2`: The routine checks a flag in D1.
- `initiate_game_loop`: The routine calls an external function pointer, stores a constant byte in a global, copies a word from the returned pointer into a global, calls initiate_emotion_sprites, and then calls another external function pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_input_interrupt`: The routine calls initiate_emotion_sprites and then calls a function pointer stored in the data segment.
- `disable_interrupt_server`: The routine calls a function pointer stored in the data segment and writes a constant to a data location.
- `initiate_emotion_sprites`: The routine initializes a counter and optionally sets it from a RAM byte when a RAM word and byte match specific values.
- `trigger_game_over`: The routine calls update_sprite_positions and returns the value in d1.
- `update_sprite_positions`: The routine updates sprite position data by iterating over three sprite entries, writes the computed values to custom chip registers via DMACON, and conditionally changes the sprite offset.
- `process_input_events`: The routine iterates over a set of input slots, reads per-slot bytes from RAM and updates a per-slot control block in RAM. It copies and combines values into the block, writes to CIAA_PRA via the custom chip registers, and sets a dirty flag when a field changes.
- `init_sprite_animation`: The routine initializes a sprite animation structure from a table entry selected by a type byte, copies palette and frame data into the sprite buffer, sets DMA control bits, and updates the sprite position fields.
- `set_player_sprite_offset`: The routine writes a 16-bit value from a table indexed by a byte in D4 to the word at offset six from the pointer in A1.
- `update_audio_channels`: The routine loops over a set of channel structures, checks a field in each structure and, when the field is non-zero, calls update_animation_frames_2.
- `update_animation_frames_2`: Updates three independent animation counters stored in the structure pointed to by A2 using tables pointed to by A3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_two_functions`: The routine calls two functions and then returns zero.
