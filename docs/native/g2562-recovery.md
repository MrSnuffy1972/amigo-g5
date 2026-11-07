# Game 2562 (odyssey (1995)(audiogenic)[cr lsd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02562). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1808032 top-level bytes in 99 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 43 of 43 named (0 author symbols), 43 of 43 described (6 by rule, 37 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1228 of 1228 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 48818 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1332 of 1332 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1352910 |
| code | 4520 |
| data | 320992 |
| padding | 24650 |
| unused | 104960 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Odyssey (1995)(Audiogenic)[cr LSD](Disk 1 of 2).adf:cracktro.exe | 2 | 196 | 7 |
| Odyssey (1995)(Audiogenic)[cr LSD](Disk 1 of 2).adf:Odyssey | 2 | 28 | 2 |
| Odyssey (1995)(Audiogenic)[cr LSD](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1098 | 33 |
| Map1.CMP (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- odyssey (1995)(audiogenic)[cr lsd](disk 1 of 2)/Odyssey (1995)(Audiogenic)[cr LSD](Disk 1 of 2).adf: file CJM1004 (header 54316 bytes, blocks hold 23912): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 919 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- odyssey (1995)(audiogenic)[cr lsd](disk 2 of 2)/Odyssey (1995)(Audiogenic)[cr LSD](Disk 2 of 2).adf:FinalMaps/Demo/Map1.CMP: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- odyssey (1995)(audiogenic)[cr lsd](disk 2 of 2)/Odyssey (1995)(Audiogenic)[cr LSD](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- odyssey (1995)(audiogenic)[cr lsd](disk 1 of 2)/Odyssey (1995)(Audiogenic)[cr LSD](Disk 1 of 2).adf:cracktro.exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- odyssey (1995)(audiogenic)[cr lsd](disk 1 of 2)/Odyssey (1995)(Audiogenic)[cr LSD](Disk 1 of 2).adf:Odyssey: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 48818 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 57344 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- Odyssey (1995)(Audiogenic)[cr LSD](Disk 1 of 2).adf:cracktro.exe

## Copy protection found

- none found

## Functions


### Odyssey (1995)(Audiogenic)[cr LSD](Disk 1 of 2).adf:cracktro.exe

- `init_library_at_entry`: Runs at program startup and calls an AmigaOS library located at a negative library base index, then returns to the caller.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `invoke_table_copy_routine`: Invokes a helper routine that waits for a flag to clear before filling a global variable and copying words from a source table, calls it a second time, and then returns.
- `wait_and_copy_table`: Waits until a global flag clears, then writes a fixed sequence of values into a global variable and copies short words from a source table into a destination table until it reaches a sentinel value of negative one.
- `allocate_and_setup_memory`: Allocates a block of memory through the system library and then coordinates two helper routines: one waits for a flag to clear before filling a global variable and copying words from a source table, and another loads a value, retrieves a pointer to a two-byte field, and stores a halved value into it.
- `store_halved_value`: Loads a value from the system library, retrieves a pointer to a two-byte field, and stores a computed value into that field by dividing a source value by two.
- `handle_by_two_callers`: Invoked by two unnamed routines and performs a short sequence of eight instructions before returning to its callers.

### Odyssey (1995)(Audiogenic)[cr LSD](Disk 1 of 2).adf:Odyssey

- `empty_return_stub`: This routine is an empty stub that returns to the caller immediately without performing any operation.
- `empty_return_routine`: This routine contains no instructions; it is an empty routine that simply returns to its caller.

### Odyssey (1995)(Audiogenic)[cr LSD](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_system_2`: Initializes the graphics library, clears a block of memory, and sets up DMA channel pointers for tile data.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 112 into d0.
- `initialize_graphics_system`: Initializes a graphics library, clears a block of memory, and sets up DMA channel pointers for tile data.
- `save_and_restore_globals`: Saves two incoming register values, sets a global variable to a small number, decrements it, stores the result, restores the saved values into the global variable and two adjacent memory locations, and finally writes a constant into the global variable before returning.
- `save_and_decrement_globals`: Saves two incoming register values, sets a global variable to a small number, subtracts one from it, stores the result, restores the saved values into the global variable and two adjacent memory locations, and finally writes a constant into the global variable before returning.
- `save_and_store_globals`: Saves two incoming register values, sets a global variable to a small number, stores the result, restores the saved values into the global variable and two adjacent memory locations, and finally writes a constant into the global variable before returning.
- `transform_coordinate_table`: Walks through a table of three-word entries, each holding a pair of coordinates and a scale factor, and applies a 2D rotation and scaling transform to each pair.
- `initialize_coordinate_transform`: Waits for a flag bit to clear, then writes a fixed sequence of values into a global variable.
- `clamp_interpolated_values`: Clamps two interpolated values into the ranges implied by the surrounding code, adjusting each so the lower bound stays at least one and the upper bound does not exceed its maximum.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `configure_blitter_operation`: Selects a blitter operation code from a set of constants based on the relative positions of two points, then programs the blitter source and destination registers and waits for the previous blitter operation to finish.
- `empty_routine`: This routine is empty and performs no operations.
- `load_tile_data_frame`: Increments a frame counter and, once it exceeds a threshold, copies a block of tile data from a source table into a destination buffer while polling a joystick register and advancing a pointer.
- `tiny_ori_3`: Small routine of 1 instruction that merges 255 into d0.
- `align_value_sequences`: Adjusts a sequence of values nibble by nibble toward matching target values, looping until a flag is cleared or a counter reaches its limit, then decrements a counter stored through a pointer.
- `store_address_and_loop_forever`: Writes its own address into a slot within a hardware register block and then enters an infinite loop that never returns.
- `process_dma_table_entries`: Walks through a table of paired entries, copying data blocks from a source area into a destination area and invoking a callback for each entry, then updates DMA request flags and decrements two counters.
- `increment_and_enable_dma`: Adds a constant to a value stored through a pointer, loads a value into a global variable, and writes a bit set into a register that enables the DMA channels.
- `program_custom_registers`: Copies a series of values into a block of custom registers, subtracts a constant from a stored value, writes a character into a memory location, and sets a bit in a register that enables DMA channels.
- `return_immediately`: An empty routine that returns immediately without modifying any data or registers.
- `clear_two_byte_field`: Writes a zero into a two-byte field located at a fixed offset from a pointer and returns.
- `increment_and_clamp_value`: Adds a low byte to a value stored through a pointer, caps the result at a maximum, stores the capped value back and into another location, and returns.
- `store_byte_and_swap_fields`: Stores a byte into a field at a fixed offset when it is non-zero, and when another value is non-zero it swaps two adjacent fields through a pointer.
- `clamp_struct_value`: Clamps a value stored at an offset within a structure, using a byte argument to either add or subtract from it while bounding the result between zero and sixty-four.
- `store_inverted_byte`: Inverts the low byte of a register and stores the complement at a fixed offset within a structure.
- `update_struct_fields`: Stores a byte into a memory location at an offset from a pointer and clears a word at another offset from a different pointer.
- `calculate_encoded_value`: Performs a mathematical transformation on a word using bitwise operations and arithmetic before storing the result at an offset from a pointer and clearing a word at another offset.
- `init_struct_from_byte`: Initializes a structure field based on a byte argument, taking a fast path for small values and computing a reciprocal-based quantity for larger ones.
- `store_parallel_button_state`: Checks a byte read from the parallel port register and, depending on whether it is zero or nonzero, sets or clears a single bit in a global variable while also clearing a two-byte field pointed to by an address register.
- `clear_bit_global`: Clears a single bit in a global variable, corresponding to releasing a button state held in the parallel port register.
- `load_character_data`: Detects a valid game data file by its magic marker, then scans a table of character definitions to compute and store per-character volume and pitch values.
- `set_channel_volumes`: Writes the volume values for the three audio channels into DMACON, with bit 15 set to enable the DMA channels, leaving the sound hardware configured.
