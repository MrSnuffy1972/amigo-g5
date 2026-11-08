# Game 102 (PinballIllusions v2.2 AGA 0710): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00102). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3358874 top-level bytes in 32 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 52 of 52 named (9 author symbols), 52 of 52 described (8 by rule, 44 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2244210 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1160 of 1160 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 181336 |
| code | 3742 |
| data | 3173781 |
| padding | 15 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| PinballIllusionsAGA.slave | 1 | 2 | 1 |
| Pinball | 2 | 552 | 11 |
| lowlevel.library | 1 | 371 | 20 |
| nonvolatile.library | 1 | 235 | 20 |
| Table001.bin (raw code file, load address unknown) | 1 | 0 | 0 |
| Table002.bin (raw code file, load address unknown) | 1 | 0 | 0 |
| Table003.bin (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- PinballIllusions_v2.2_AGA_0710/PinballIllusionsAGA/data/Table001.bin: high-entropy data file (7.70 bits/byte), classified as packed data
- PinballIllusions_v2.2_AGA_0710/PinballIllusionsAGA/data/Table002.bin: high-entropy data file (7.68 bits/byte), classified as packed data
- PinballIllusions_v2.2_AGA_0710/PinballIllusionsAGA/data/Table003.bin: high-entropy data file (7.69 bits/byte), classified as packed data
- PinballIllusions_v2.2_AGA_0710/PinballIllusionsAGA/data/intro.bin: high-entropy data file (7.81 bits/byte), classified as packed data
- PinballIllusions_v2.2_AGA_0710/PinballIllusionsAGA/data/main.bin: high-entropy data file (7.81 bits/byte), classified as packed data
- PinballIllusions_v2.2_AGA_0710/PinballIllusionsAGA/data/menudata.bin: high-entropy data file (7.63 bits/byte), classified as packed data
- PinballIllusions_v2.2_AGA_0710/PinballIllusionsAGA/data/music001.bin: high-entropy data file (7.76 bits/byte), classified as packed data
- PinballIllusions_v2.2_AGA_0710/PinballIllusionsAGA/data/table001.mnu: high-entropy data file (7.66 bits/byte), classified as packed data
- PinballIllusions_v2.2_AGA_0710/PinballIllusionsAGA/data/table002.mnu: high-entropy data file (7.67 bits/byte), classified as packed data
- PinballIllusions_v2.2_AGA_0710/PinballIllusionsAGA/data/table003.mnu: high-entropy data file (7.67 bits/byte), classified as packed data
- PinballIllusions_v2.2_AGA_0710/PinballIllusionsAGA/data/Table001.bin: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- PinballIllusions_v2.2_AGA_0710/PinballIllusionsAGA/data/Table002.bin: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- PinballIllusions_v2.2_AGA_0710/PinballIllusionsAGA/data/Table003.bin: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3 never-loaded region(s), 2244210 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### PinballIllusionsAGA.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Pinball

- `Begin`: Initializes the font, opens graphics and intuition libraries, and opens the dos library, storing the handles in a global variable.
- `store_value_offset_28`: Stores the value five into offset 28 of the structure in a5, then branches away.
- `SetCopper`: Disables copper copy mode by clearing the copper control register and waits for the copper to finish before returning.
- `InitFont`: Walks a table of short values, writing each running total into the following slot and advancing by four bytes, repeating the process ten times.
- `ShowErrors`: Scans a zero-terminated string of return codes and, when it finds an empty entry, displays the internal error requester, otherwise it returns.
- `LoadSeg`: Loads a data file from disk and, if it is a valid ILUS chunk, decompresses it and relocates the pointers inside the loaded segment so they point at their final addresses.
- `UnLoadSeg`: Walks a segment table, clearing the high bit of the first negative entry and freeing that segment, otherwise it returns when the table is exhausted.
- `GetDisk`: Stores d0 into a global variable, converts it to a character, locks a disk by name, copies five longwords from a graphics table into a swap buffer in a loop, clears the remaining entries, then waits for the disk to be ready while polling the CIA port register before unlocking and returning a status value.
- `load_table_value`: Loads a value from a table indexed by a program offset into a data register and then falls through into the following routine.
- `DeCrunch`: Copies the header fields from a packed data block pointed to by a0 into the block itself, then copies eight bytes into a stack buffer.
- `CDIO2`: Performs twelve sequential no-op byte OR operations on register d0, each of which leaves the value unchanged, and then returns.

### lowlevel.library

- `return_minus_one`: Library code that loads the value minus one into register d0 and returns immediately.
- `initialize_lowlevel_devices_2`: Initializes a device structure by storing the base pointer and a second pointer, clearing several fields, and registering three handlers, then opens graphics and intuition libraries and sets up a timer with a keyboard handler.
- `initialize_lowlevel_devices`: Stores a pointer plus 500 into a structure, sets a flag word, then repeatedly calls a low-level library routine and, while it succeeds, opens and initializes devices, storing their handles at offsets 64, 44, 68, and 72 before returning the structure pointer.
- `release_structure_entry_return_zero`: Library code that reads a longword from the structure at offset 64 in a5, calls two library functions, then returns zero.
- `increment_and_return_a6`: Library code that increments a 16-bit value at offset 32 in a6 by one, copies a6 into d0, and returns.
- `tiny_addq_moveq_rts`: Small routine of 3 instructions that adds 1 to (a1), then loads 0 into d0, then returns.
- `build_and_post_event_record`: Library code that calls a library function, stores its result and two bytes into a structure, then branches on a check to set a word at offset 38 before calling more library functions and returning.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `return_tristate_check_result`: Library code that calls a library function, then returns zero, one, or minus one depending on whether the first and second success checks succeed.
- `dispatch_via_a5_pointer`: Library code that checks bit 2 of a byte at offset 109 in a1, and if it is set, loads pointers from offsets 76, 18, and 14 of that structure before jumping to the address in a5.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `process_keyboard_flag`: Reads the keyboard serial register, inverts it, rotates the result right by one bit, then calls the handler list routine and updates a flag word at offset 40 of the structure in a4 when the processed value matches a specific mask.
- `store_masked_flag_word`: Checks bit 7 of the byte in d2 and, if that bit is clear, stores d2 into the word at offset 42 of the structure in a4, otherwise it returns.
- `mask_handler_flag`: Copies d2 into d1, clears bit 7 of that value, compares it against the word at offset 42 of the structure in a4, and if they differ sets that word to all ones, copies d2 into d0, subtracts 249 from d2, and loads a pointer from offset 252 into a1.
- `call_each_handler_in_list`: Library code that walks a linked list of handler nodes starting from a1, calling the function at offset 18 of each node until the list ends, then restores registers and returns.
- `process_input_and_notify`: Decrements a 32-bit counter at offset 34 of the structure in a5 when the byte at offset 50 is non-zero and the counter reaches zero, then copies the value at offset 38 into offset 34.
- `store_state_and_notify`: Compares the top nibbles of two input values and calls a routine when both are non-zero and differ, then stores the second value at offset 46 of the structure in a5 and calls routines depending on which top nibble the stored value has.
- `process_input_byte_pair`: Reads two consecutive bytes from the buffer in a2 into d0 and d1, then uses d0 as a bit mask to test d2 and d7, setting bit 7 of d1 when the d0 mask is set in d7, otherwise it returns.
- `post_shifted_input_event`: Copies the longword at offset 42 to offset 34 of the structure in a5, shifts the value at offset 30 left by 8 bits, combines it with d1, calls a routine at offset 12 to post the event, then jumps back to an earlier routine.
- `post_input_event_message`: Library code that builds an input event record from the code in d0, setting flag bits when d0 matches a stored value or equals 120, sends it through library calls, then updates a stored value in the structure at a5.

### nonvolatile.library

- `return_negative_one`: Returns the value -1 in the data register d0 and returns.
- `init_nonvolatile_structure`: Initializes a structure pointed to by d0 by storing the a0 and a6 registers at offsets 36 and 40, writes the value 36 to the word at offset 22, then calls the library at offset -558 in a6 twice with pointers from offsets 110 and 64 before calling another routine and returning the structure pointer in d0.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_structure_head`: Clears the longwords at offsets 0 and 4 of the structure pointed to by d0 and returns without modifying any registers.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `check_handler_pointers`: Pushes a register on the stack, clears register d7, loads a pointer from offset 60 of the structure in a6, and branches away when that pointer is zero after testing two memory locations.
- `run_library_allocation_sequence`: Saves several registers, then calls a sequence of library handlers that allocate a block, store a stack address into it, and run further handlers depending on whether each call returns zero, before restoring the stack and returning a pointer.
- `call_library_handlers_2`: Saves the value in d5 into d1 and calls the library handler at offset -126 in a6, then loads a stack pointer from d7, pops a value into d0, and restores two base pointers from offsets 40 and 44 of the structure in a5 before calling the routine at offset -210 in a6.
- `call_library_handlers`: Calls the library routine at offset -126 in a6 three times with the values from d5, d6, and a3 in d1, then restores the saved registers and returns the value of d7 in d0.
- `check_pointers_and_call`: Pushes d7 and several registers onto the stack, sets d7 to 3 and copies a6 into a5.
- `handle_library_dispatch`: Clears a4, calls the library routine at offset -126 in a6 and saves the result in d6, then loads a value from offset 24 of the stack into d1 and sets d2 to -2 before calling the library routine at offset -84.
- `dispatch_library_responses`: Copies d0 into d1 and calls a library routine at offset -126 in a6, then loads a value from offset 28 of the stack into d1 and the top stack word into d2 before calling the library routine at offset -30.
- `dispatch_conditional_call`: Calls a library routine, then compares the returned value against several constants and loads a specific value into register d7 when the result matches one of them.
- `dispatch_conditional_library_calls`: Sets d7 to one, selects a value from d1 based on whether d4 or d5 is zero, then calls two library handlers and invokes another routine with a5 and a6 swapped, restoring them before returning.
- `conditional_library_calls`: Skips the first library call when d5 is zero, then calls the library routine at offset -126 in a6 with d5, calls the one at offset -90 with d6, restores the saved registers, and returns the value of d7 in d0.
- `dispatch_library_sequence_2`: Pushes several registers onto the stack, clears d7, loads a pointer from offset 60 of the structure in a6 into d1, and returns immediately when that pointer is zero.
- `dispatch_library_sequence`: Reads a base pointer from offset 44 of the structure in a5, calls the library routine at offset -126, then branches to a second block when the result is zero that calls the routines at offsets -84, -126, -72, -90, -126, -72, and -126 in sequence, returning the final result in d0.
- `call_library_pair_zero`: Loads a routine address into a1, clears d0, calls the library routine at offset -414, saves its pointer, calls the one at offset -552 with the cleared d0, and returns zero.
