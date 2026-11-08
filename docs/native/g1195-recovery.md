# Game 1195 (heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01195). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 38 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 53 of 53 named (0 author symbols), 53 of 53 described (4 by rule, 49 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 870 of 870 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 36754 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1655 of 1655 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 59224 |
| code | 5188 |
| data | 787439 |
| padding | 5237 |
| unused | 44032 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:heimdallII | 2 | 223 | 8 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:Fairlight_UK | 2 | 1199 | 36 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:installh2 | 2 | 223 | 8 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:bootblock | 1 | 10 | 1 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:font1p.pak#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:english.pak#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:german.pak#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:french.pak#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:italian.pak#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:font1p.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (992 -> 2846 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:stattabs.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (342 -> 560 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:english.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4053 -> 7978 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:intro.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (123217 -> 226062 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:german.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4359 -> 8340 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:flags.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (41096 -> 77248 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:french.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4264 -> 8572 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:endseq.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (181135 -> 226466 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:lokiend.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (24436 -> 63720 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:seqtext.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (656 -> 900 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:hltbar.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (61 -> 1070 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:worlds/loki.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (32479 -> 59752 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:worlds/hall.dat: high-entropy data file (7.72 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:worlds/gamegfx.dat: high-entropy data file (7.88 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:worlds/hot.dat: high-entropy data file (7.92 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:italian.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4301 -> 9018 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:font1p.pak#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:english.pak#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:german.pak#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:french.pak#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:italian.pak#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:heimdallII: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)[a2]/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:installh2: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 5 never-loaded region(s), 36754 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_graphics_and_interrupts` (Fairlight_UK): Checks a bit in a library-provided value, performs several library calls to manage memory and pointers, updates a structure with new function pointers, and modifies CIA registers. Kind: disk hardware/track validation. Pass path: the routine continues to the subsequent instructions (bsr.w an address, bsr.w an address) after the call to sub_0_0000.

## Functions


### Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:heimdallII

- `unpack_compressed_data_table`: Scans a table for a specific signature, then performs a decompression routine that reads bitmasks and values to fill a destination buffer.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a condition on a global mask is met, then reads a byte from a table entry and uses it as an index into a global mask to extract a bit, calling another routine if a borrow condition is detected.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `perform_arithmetic_on_registers`: Performs a series of arithmetic and bitwise operations on registers d1, d6, and d7, including a swap and a rotation, before returning.
- `fill_figure_table`: Reads a count, fills a table with values from a stack buffer, and then iterates through a loop to write bit masks and derived values into a table based on a key list.
- `process_pointer_table`: Iterates through a list of values pointed to by a4, performing a calculation involving a pointer derived from a constant address and adding the result to a table at a5 offset by the current value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:Fairlight_UK

- `execute_startup_sequence`: Runs a loop 32767 times, then calls six different routines in sequence before returning.
- `setup_graphics_and_interrupts`: Checks a bit in a library-provided value, performs several library calls to manage memory and pointers, updates a structure with new function pointers, and modifies CIA registers.
- `initialize_library_pointers`: Calls another routine, modifies a byte at an offset of a global variable, sets two 16-bit values in a structure, and performs several library calls to update pointers and values.
- `clear_hardware_registers`: Runs a loop eight times, calling another routine and clearing two 32-bit words at the address pointed to by the hardware register at offset 144.
- `initialize_table_values`: Writes several 16-bit values to a table, clears a word at a global address, calls another routine, and then loads a pointer into d0.
- `store_split_word`: Stores the lower word of d0 at offset 6 of the structure in a1 and the upper word of d0 at offset 2 of the structure in a1.
- `clear_memory_block`: Clears a block of memory starting at an address loaded from a PC-relative location for 5831 iterations.
- `process_encoded_bytes_2`: Calls another routine five times with different addresses passed in a0.
- `process_encoded_bytes`: Searches for a specific byte sequence in a buffer, then modifies a byte at a specific position using a calculation involving a subtraction and a bitwise shift.
- `execute_conditional_updates`: Calls several routines, then enters a loop that increments a value at a PC-relative address and clears it if it reaches 80, provided a byte at a location in a6 is not 127; it continues this loop while a word at another PC-relative address is zero.
- `process_data_table`: Increments a global word by four, then enters a loop that calls another routine eighty times, potentially updating a pointer from a table based on a value in d0.
- `copy_data_from_table`: Reads a byte from a0 and, depending on its value, either returns a constant, performs a pointer addition, or copies eight bytes from a table into a destination pointed to by a1.
- `set_memory_value`: Pushes a0 to the stack, loads a PC-relative address into a0, writes the word four to that address, and branches to another location.
- `check_and_set_flag`: Checks a word at a PC-relative address and, if it is zero, writes the value 2 to that same address; otherwise, it enters an infinite loop.
- `update_split_values`: Calls another routine, loads two PC-relative addresses into a0 and a1, calls another routine, and returns if the zero flag is set.
- `update_split_values_double`: Calls another routine, loads two PC-relative addresses into a0 and a1, calls another routine, loads a different pair of PC-relative addresses into a0 and a1, calls another routine, and returns if the zero flag is set.
- `synchronize_split_values`: Calls check_and_set_flag to validate a word at a PC-relative address, then calls increment_split_value twice using different pairs of PC-relative addresses to align two 16-bit words.
- `synchronize_split_values_2`: Calls check_and_set_flag to validate a word at a PC-relative address, then calls increment_split_value twice to update two separate 16-bit words using different source and destination pointers.
- `increment_split_value`: Compares two 16-bit words and, if they differ, increments the nibbles of the first word to match the nibbles of the second word, then stores the result back to a0.
- `handle_interrupt_and_hardware`: Checks an interrupt status register and, if a specific bit is set, writes values to CIAA registers and performs an interrupt return.
- `handle_interrupt_request`: Checks the interrupt status register, and if specific bits are set, calls another routine and decrements a value at a PC-relative address before returning via an interrupt return.
- `update_status_and_flag`: Checks a byte in a6 and, if it matches a specific value, modifies a PC-relative word and clears the byte in a6, provided certain bits in a4 and a5 are set.
- `branch_to_offset`: Performs a short branch to an offset of 4f8.
- `branch_to_offset_2`: Performs a long branch to an offset of 586.
- `update_entity_data`: Iterates through a table to calculate and store new values into a structure, then clears several hardware audio registers and sets a bit in a CIA register.
- `update_entity_coordinate_and_state`: Calculates a new coordinate value by reading a byte from a table, shifting and adding it to a value at a negative offset of the structure in a5, and then calls several subroutines to update the entity state using different parameter constants.
- `update_entity_state_and_properties`: Updates an entity structure by loading data from a table based on a calculated index, then performs conditional branches to other routines based on state nibbles.
- `dispatch_entity_state_logic`: Calls another routine, then checks a 12-bit value and a 4-bit nibble within a structure in a4 to branch to various subroutines based on the nibble's value.
- `update_entity_state_index`: Reads a 16-bit value from offset 16 of a structure in a4 and stores it into a location calculated by adding 6 to the sum of a6 and a signed word in d5, then returns.
- `search_table_and_update_structure`: Searches a table for a value that satisfies a comparison against a 12-bit masked word from a structure, then updates a 16-bit value at offset 24 of that structure.
- `calculate_scaled_value`: Updates a 16-bit value at offset 16 of a structure in a4 based on a byte at offset 23, then performs a loop to find a value in a table and stores the result at offset 6 of a structure in a6.
- `calculate_scaled_offset`: Performs complex arithmetic using values from a structure in a4 to calculate a scaled offset, which is then stored in a structure at a6, and updates a field in a4.
- `process_entity_state_and_input`: Reads a state byte from a structure in a4 and branches to various subroutines based on its low and high nibbles.
- `update_structure_fields`: Writes a byte derived from masking a byte at offset 3 of a structure in a4 into offset 18 of the same structure.
- `update_pointer_and_check_bounds`: Extracts the high nibble of a status byte and, when it is nonzero and a flag at offset 35 is set, adds a value from a table to that byte and clears the flag, then adjusts a pointer stored at offset 36 so it does not pass a computed address, and inverts the byte it points to.
- `decrement_counter_loop`: Runs a loop that decrements a counter five times, each iteration performing a comparison of a byte at offset 6 of a structure against itself before returning.

### Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:installh2

- `unpack_compressed_data_table_2`: Scans a table for a specific signature, then performs a decompression routine by reading bitmasks and values to fill a destination buffer.
- `extract_flag_bit_from_table_2`: Scans a table of word pairs until a condition on a global mask is met, then reads a byte from a table entry and uses it as an index into a global mask to extract a bit, calling another routine if a borrow condition is detected.
- `initialize_registers`: Sets d0 to 255 and d1 to 16.
- `mask_and_conditional_call_2`: Masks a word in register d0 with a word in register d6 and, if a byte in register d7 is less than a byte in register d1, calls another routine and returns its result; otherwise it returns the masked word.
- `calculate_bit_manipulated_difference`: Performs arithmetic on a byte in a register, extracts two bytes from a table using an offset in a register, performs bitwise rotations and swaps on those bytes, and calculates a final difference between the modified register value and the processed table data before returning.
- `fill_figure_table_2`: Reads a count from another routine, fills a stack-based buffer with bytes, and then iterates through a loop to write bit masks and derived values into a table based on a key list.
- `process_data_list`: Iterates through a list of values starting at a pointer in a4, clearing memory and using values to calculate and add offsets to a destination array pointed to by a5.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
