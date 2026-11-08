# Game 861 (alianator (1991)(p.s.)(pd)[h trsi]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00861). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908032 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 43 of 43 named (0 author symbols), 43 of 43 described (8 by rule, 34 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 960 of 960 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 360085 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1839 of 2084 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 496 |
| code | 7220 |
| data | 411168 |
| padding | 1212 |
| unused | 487936 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Alianator (1991)(P.S.)(PD)[h TRSi].adf:TRSI! | 2 | 267 | 8 |
| Disk-Validator | 1 | 386 | 1 |
| Alianator (1991)(P.S.)(PD)[h TRSi].adf:Alianator | 1 | 376 | 7 |
| Alianator (1991)(P.S.)(PD)[h TRSi].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 226 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 819 | 8 |
| Alianator (1991)(P.S.)(PD)[h TRSi].adf:MainPart (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 161 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 546 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- alianator (1991)(p.s.)(pd)[h trsi]/Alianator (1991)(P.S.)(PD)[h TRSi].adf:MainPart: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 360085 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Alianator (1991)(P.S.)(PD)[h TRSi].adf:TRSI!

- `relocate_amiga_dos_hunks`: Parses a data structure containing AmigaDOS hunk type codes to perform memory relocation and address patching.
- `process_data_and_call_library`: Decrements a value in d1 and, if non-zero, performs arithmetic on a pointer in a1, calls a library routine using a pointer derived from d3 and a1, and then potentially calls another library routine after updating a global variable with a value from d0.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_and_shift_buffer_data`: Calculates a destination address by shifting and adding values, then enters a loop to copy bytes from a source to a destination, calling other routines to handle bitwise operations and data movement.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `rotate_and_extract_bits`: Performs a bitwise rotation on d5 using d4 as a count, and uses a loop to process data from a source pointed to by a0, potentially calling other routines and writing bytes to a destination pointed to by a3.
- `load_and_call_system_trap`: Loads a 16-bit word from an offset of 1741 in a2 into d4, sets bit 6 and bit 7 of d1, calls a system trap, and then performs a logical OR of d0 into d1.

### Disk-Validator

- `update_disk_validation_data`: Calculates a character representation of a value from a global structure, writes it to a buffer, and performs a series of bitwise operations and calls to update a bitmask and several counters within a data structure.

### Alianator (1991)(P.S.)(PD)[h TRSi].adf:Alianator

- `load_game_resources`: Allocates memory, opens a file, reads data, and performs several library calls to manage the loaded data, otherwise it sets a hardware register to a specific value and branches.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_d0_register`: Performs a long sequence of logical operations to set the d0 register to zero and then returns.
- `copy_buffer_segments_2`: Sets a hardware register to fifteen, calculates a length by subtracting a value from eighty thousand, and then enters a loop that copies bytes from one memory location to another while decrementing a counter.
- `copy_buffer_segments`: Copies a byte from a source to a destination and decrements a counter, then if the counter reaches zero, it attempts to copy a second segment of data from a different source to a new destination using a loop.
- `copy_bytes_with_conditional_jump`: Negates a byte in d1, decrements d0, and enters a loop that copies bytes from a0 to a1, then checks a value at a specific memory location to decide whether to jump to a fixed address.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Alianator (1991)(P.S.)(PD)[h TRSi].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_state`: Performs several library calls to manage memory and graphics, updates two tables by swapping and storing word values, and calls several other routines including one that checks a bit in a hardware register.
- `dispatch_input_event`: Checks a pointer against several constant values, calls a routine at that pointer, sets bit 1 of a global variable, and stores a value from 1 to 3 into a global variable.
- `execute_input_callback`: Compares a global variable against 1, 2, or 3, and calls a specific routine at an offset of the pointer in a6 based on the match.
- `reset_audio_control_words`: Clears four global words, sets one global word to fifteen, and clears bit 1 of it.
- `call_two_subroutines`: Calls two consecutive subroutines and then returns.
- `jump_to_address`: Performs an indirect jump to an address stored in a memory location.
- `advance_table_index_and_call`: Decrements a 16-bit counter in a global variable and returns while it is nonzero, otherwise advances a table index past entries that are null and calls the first non-null routine found, storing its return value.
- `store_pointer_and_load_d1`: Stores a fixed address into a global variable, and if the byte pointed to by another global variable is zero, it replaces that pointer with the address of a global variable, then loads a fixed value into data register d1 and returns.
- `store_constants_return`: Stores the value an address into a global variable and returns the value an address in the register pair, performing no other work.
- `load_immediate_values`: Loads the constant a fixed address into d1 and 200 into d0, then returns.
- `init_struct_and_copy_table`: Writes fixed constants into several fields of the structure in a6, then waits until a bit in a global variable is cleared before copying twelve words from one table to another and advancing the source pointer.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `translate_buffer_advance_pointers`: Loads two addresses from global variables into registers, calls a routine that translates and copies a buffer, then increments one address by 48 and the other by 384, stores both updated addresses back into the global variables, and returns.
- `modify_global_and_register`: Performs a bitwise OR on a value at an offset of a4 and clears d0.
- `copy_data_from_table`: Uses a byte from a source buffer to index into a table, then copies seven words from that table location to a destination buffer in a1, repeating this for 44 iterations.
- `rotate_register_right`: Rotates the bits of d7 to the right by one position.
- `tiny_asr`: Small routine of 1 instruction that shifts d0 right by 7.
- `calculate_and_store_summed_bytes`: Reads three bytes from a table using offsets from three global variables, sums them with a constant, and stores the result into a destination table in a loop that updates the offsets and the destination pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `build_audio_channel_table`: Clears a 32-longword block and builds a table of four entries from a source list, computing their pointers and fixed values.
- `install_handler_and_reset_state`: If a global variable is not one, it saves the current value of a global pointer into a location two bytes after another routine and replaces that global pointer with its own address, then sets a global variable to one, clears two 32-bit fields in a structure, and loads a value from a table into a third field in that structure before returning.
- `update_audio_and_sprite_states`: Sets four different values in a structure, calls a routine four times to update audio channels, and then checks counters to potentially update sprite animation states and call several other routines.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_sprite_animation_state`: Updates a structure's animation frame and pointer by reading from a table based on a sprite index.
- `write_audio_channel_registers`: Checks the state of each of four sound channels, updates its stored pointer and length, and writes them to the audio location and length registers.
- `negate_sample_byte`: Negates one byte of sample data at an index computed from a counter and a base offset.
- `mix_two_sample_buffers`: Checks two separate count fields in a structure; if a count is non-zero and its corresponding internal counter is zero, it initializes the counter and performs a loop that averages two 32-byte sample buffers into a third destination buffer by adding a bias of 128 to each sample and using a wrapping index.
