# Game 724 (Zeewolf): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00724). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909456 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 61 of 61 named (0 author symbols), 61 of 61 described (2 by rule, 59 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 517 of 517 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 3584 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1792 of 1943 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 38513 |
| code | 11872 |
| data | 858045 |
| padding | 2 |
| unused | 1024 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Zeewolf.adf:bootblock | 1 | 136 | 4 |
| logged disk load from Zeewolf.adf boot-block load | 1 | 215 | 12 |
| logged disk load from Zeewolf.adf boot-block load | 1 | 107 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 79 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1206 | 30 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 101 | 7 |
| Zeewolf.adf bytes 450560-452096 (code never loaded by a reference run) | 1 | 0 | 0 |
| Zeewolf.adf bytes 458240-459264 (code never loaded by a reference run) | 1 | 0 | 0 |
| Zeewolf.adf bytes 753152-754176 (code never loaded by a reference run) | 1 | 99 | 1 |

## Cases handled

- Zeewolf/Zeewolf.adf: custom-format (non-DOS) disk, 1756 non-empty blocks: matched against track-loaded segments
- Zeewolf/Zeewolf.adf: the boot block reads 10240 bytes from disk offset 890880 to an address (found statically); recovered as a unit
- Zeewolf/Zeewolf.adf: the boot block reads 22016 bytes from disk offset 856064 to an address (found statically); recovered as a unit
- runtime segment an address: 49 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 393 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 75 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Zeewolf/Zeewolf.adf: bytes 450560-452096 look like code no run loaded: recovered position-independently (load address unknown)
- Zeewolf/Zeewolf.adf: bytes 458240-459264 look like code no run loaded: recovered position-independently (load address unknown)
- Zeewolf/Zeewolf.adf: bytes 753152-754176 look like code no run loaded: recovered position-independently (load address unknown)
- Zeewolf/Zeewolf.adf: 5 logged loads cover 35840 bytes of the image
- Zeewolf/Zeewolf.adf: 22 track-loaded blocks found in recovered runtime segments and classified through them
- Zeewolf/Zeewolf.adf: blocks 2-877 (bytes 1024-449536) outside the filesystem hold data, not code (opcode density 0.049, entropy 6.22)
- Zeewolf/Zeewolf.adf: blocks 885-892 (bytes 453120-457216) outside the filesystem hold data, not code (opcode density 0.027, entropy 4.40)
- Zeewolf/Zeewolf.adf: blocks 904-1264 (bytes 462848-647680) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.71)
- Zeewolf/Zeewolf.adf: blocks 1266-1319 (bytes 648192-675840) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.38)
- Zeewolf/Zeewolf.adf: blocks 1321-1468 (bytes 676352-752128) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.04)
- Zeewolf/Zeewolf.adf: blocks 1480-1671 (bytes 757760-856064) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.58)
- Zeewolf/Zeewolf.adf: blocks 1715-1739 (bytes 878080-890880) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.35)
- 2 never-loaded region(s), 2560 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Zeewolf.adf:bootblock

- `initialize_boot_block`: The routine prepares a small control block in memory by writing constants to several of its fields and installing a pointer to a jump table that is chosen based on a short value derived from the library base.
- `configure_two_tables`: The routine writes fixed values into two separate tables via pointers passed in registers, then calls another routine through a table indexed by a global variable and returns its result after invoking a further routine.
- `store_jump_instruction`: The routine writes a 32-bit value to an offset within a structure pointed to by a register, effectively storing a jump instruction in a table entry.
- `initialize_display_font`: The routine opens the intuition library and a font, then waits for a hardware condition, copies a font table from a global table into a work buffer using a loop that processes control bytes and writes bytes to two locations per iteration, waits for another hardware condition, and then closes the libraries and disables interrupts.

### logged disk load from Zeewolf.adf boot-block load

- `copy_and_patch_code_block`: The routine copies a block of long words from a source area in RAM to a destination pointer derived from a global variable and a byte in RAM, aligning the pointers to even long-word boundaries.
- `copy_and_patch_memory`: Copies a block of memory or shifts a buffer and then writes a sequence of words from a global variable to a location near the stack.
- `copy_and_shift_buffers_loop`: The routine repeatedly copies bytes between buffers using a counter, then shifts a buffer forward by one byte per iteration, and finally copies a small table from a global variable into a stack area before indirect calling a routine selected from that table.
- `extract_bit_from_table`: The routine scans a table of 16-bit entries using a mask in a register until it finds a matching pair, then reads a byte from the table to form an index.
- `return_masked_word`: The routine builds a 32-bit value from a global word and a constant high word, then conditionally calls another routine if a global byte is less than a threshold, and returns the resulting value.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a global variable and, if a byte from a register is greater than a byte from a global variable, it calls another routine and returns its result; otherwise it returns the masked word.
- `perform_boot_block_initialization`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `accumulate_32bit_from_bytes`: Reads four consecutive bytes from a pointer in a register and concatenates them into a single 32-bit register value.
- `decode_and_fill_table`: The routine reads a count from another routine, fills a temporary buffer with that many bytes from another routine, then iterates over a 12-byte stack buffer and a table pointed to by a register.
- `copy_table_and_jump`: The routine copies ten 16-bit words from a global variable backwards into a stack area, then loads the word stored at the last copied location as a code pointer and jumps to it.
- `restore_registers_from_stack`: Moves the value in d1 to a location on the stack, restores d0-d7 and a0-a7 from the stack, and returns.
- `clear_bytes_dbra_restore_regs_return`: Clears d0 plus one bytes starting at a6, stores a5 above the return address, restores all registers from the stack, releases the frame and returns.

### logged disk load from Zeewolf.adf boot-block load

- `store_word_and_return_shifted`: The routine reads a word from four bytes before a pointer in A0, stores a byte derived from D7 and the low byte of that word through a pointer in A5, and returns the word shifted right by one with the high bit set.
- `shift_loop_decrement`: The routine repeatedly shifts a value in d0 right by one and decrements a counter loaded from d1, reloading the counter from d1 on each iteration.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_input_state`: The routine clears a global variable and then, if a flag in that variable is set, it calls another routine and writes several fixed values to memory locations, including a flag and a value that appears to be a timer or counter.
- `update_input_flag_table`: The routine updates a byte in a table indexed by a value from a register, setting a flag bit and optionally incrementing a counter, and maintains a global variable that tracks the current input state and can be cleared when a matching condition is met.
- `fetch_global_byte`: Loads a byte from a global variable into d0 and returns.
- `fetch_masked_byte`: Fetches a byte from a table using a sign-extended index in d0 and returns it with only the highest bit set.
- `decrement_table_entry`: Fetches a byte from a table using a sign-extended index in d0, and if the lowest three bits are non-zero, decrements the byte at that table location before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_global_table_entries`: The routine calls another routine repeatedly and walks a small table pointed to by a global variable, replacing the pointer with values returned by the callee when table entries are non-negative.
- `jump_to_update_audio_dma_state`: Jump stub: passes control straight on to update_audio_dma_state.
- `initialize_sound_channel`: The routine initializes a sound channel entry from a descriptor table, copies descriptor fields into the channel structure, computes signed deltas and direction flags for two parameters, programs the Amiga audio hardware registers for the channel, and updates a global variable and a small RAM state used for channel allocation.
- `increment_pointer_and_set_byte`: Increments a 16-bit value at a pointer in a6 by four, performs three bitwise OR operations on d6, and stores the value six into a global variable.
- `update_audio_dma_state`: The routine walks a table of channel descriptors and, for each active entry, computes a period value from descriptor fields and writes it to the audio hardware registers.
- `set_dma_one_and_globals`: Clears a 16-bit global variable, sets the DMA controller register to one, and writes two hundred fifty-five to another global variable.
- `reset_dma_config`: Clears a 16-bit global variable, sets the DMA controller register to two, and writes two hundred fifty-five to another global variable.
- `reset_dma_and_globals`: Clears a 16-bit global variable, sets the DMA controller register to four, and writes two hundred fifty-five to another global variable.
- `set_dma_and_globals`: Clears a 16-bit global variable, sets the DMA controller register to eight, and writes two hundred fifty-five to another global variable.
- `set_audio_volume`: Calculates an offset by shifting the value in d1 left by 4 and adding it to the address of the audio volume register, then stores the word from d0 at that location.
- `set_audio_period`: Calculates an offset by shifting the value in d1 left by 4 and adding it to the address of the audio period register, then stores the word from d0 at that location.
- `initialize_asset_table`: Calls a routine to copy a table, then copies 64 words from the source to a global destination.
- `update_structure_and_clear_tables`: Updates four pointers in a structure by adding the value at offset 0 of the structure to offsets 4, 8, and 12, then performs a loop to update values in a list, followed by four loops that clear 20 words each in four different tables, and finally stores several values into a local stack frame.
- `copy_table_and_set_global`: The routine copies a small table from a global variable to a table pointed to by another routine when a flag is cleared, then sets a global variable based on several sentinel checks and finally writes a constant to the global variable.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `check_global_table_entries_2`: The routine walks a small table pointed to by a global variable, invoking another routine for each non -1 entry and for a final flag check, then performs a short loop that spins while a global variable changes, calling another routine before and after the spin.
- `copy_registers_to_ram_block`: The routine checks a global variable and, if non-zero, calls another routine and then stores the returned values from registers D2-D7, A0, A1, A2, A3, A4 and A6 into a contiguous block of RAM, also writing the address of a global variable into the block.
- `process_audio_channel`: The routine updates a counter and, when it wraps, decrements a delay counter or calls another routine and then iterates over a table of entries.
- `update_entity_state`: The routine updates fields in a per-entity structure pointed to by a table base and an index, applying type-dependent arithmetic to speed, direction and related counters and clamping the results to allowed ranges.
- `clamp_and_store_value`: Reads a byte from offset 2 of a structure in a5, adds it to a word at offset 12 of the same structure, compares that word to 888, and if it is greater than 888, sets it to 888 before storing the result at offset 6 of a structure in a4.
- `process_animation_frames`: The routine decrements a counter in a structure and returns early if it is non-zero.
- `update_game_state_from_table`: The routine updates several global variables from a table pointed to by a global pointer, using values read from a table referenced by another global variable.
- `process_tile_map_data`: The routine processes a table of tile map entries using several pointer registers and a loop counter.
- `parse_script_command`: The routine reads a byte-indexed table entry and stores a pointer to it, then loops over a set of entries updating pointers and flags, handling a command type 3 case that copies bytes and updates a table field, and finally returns a concatenated pointer and value derived from the processed entry.
- `decode_tile_attribute`: The routine reads bytes from a table pointed to by a register, manipulates nibbles in those bytes and in a register, and writes the modified bytes back to the table.
- `merge_nibbles_in_structure`: Reads a byte from offset 2 of a structure, then merges its lower nibble with the lower nibble of a byte at offset 24 of the same structure, or merges its upper nibble with the upper nibble of the byte at offset 24, and writes the result back to offset 24.
- `dispatch_tile_command_and_update`: Selects and executes a command from a jump table based on a masked byte from a structure, then executes a series of conditional arithmetic operations on structure offsets including subtraction and addition for bounds checking, bitwise manipulation of a hardware register, and writing values to registers a3 and a5 before calling copy_table_and_set_global.
- `update_tile_attributes`: Updates or initializes tile-related metadata and attribute-related memory or buffers.
- `update_entity_state_4`: The routine updates fields in a per-entity structure pointed to by a pointer and a parallel control block, applying arithmetic on bytes and words, clamping values, and indexing a global table with a computed offset to read a signed short that is added to another field.
- `decrement_and_copy_register_state`: Decrements a global counter and, if non-zero, updates a set of limit-check variables using a bit-masked value from that counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_interrupt_flag_and_call_handler`: The routine checks a bit in a global variable and, if clear, writes a value to a global variable, optionally calls another routine based on a flag, then writes a different value to another global variable before returning.
- `update_palette_and_global_pointer`: Calls a routine to assign a global routine pointer, copies a table of thirty words to the hardware color registers, and calls a routine to write a palette entry before returning.
- `wait_for_d0_low_nibble`: The routine spins until a low-nibble field in a global variable is zero and then spins until that field is non-zero, then writes a constant to a global variable and returns the concatenation of another routine and a constant.
- `assign_global_routine_pointer`: The routine stores a reference to itself into a global variable and then returns.
- `increment_two_counters`: The routine increments two global variables, wrapping each to a negative value when it reaches a specific positive limit, then calls another routine and returns the original register values.
- `write_palette_entry`: The routine computes an index from two input values and a global variable, stores the resulting 32-bit value as two 16-bit words into four consecutive table entries, then writes a nibble-mirrored byte derived from the low bits of one input into the next table slot.
- `copy_color_palette`: Copies 30 words from a table located at a specific offset from the current program counter to the hardware color registers starting at offset a fixed address.

### Zeewolf.adf bytes 753152-754176 (code never loaded by a reference run)

- `disable_dma_and_interrupts`: The routine writes to DMACON to disable DMA channels and then writes to INTENA to disable interrupts, and then returns.
