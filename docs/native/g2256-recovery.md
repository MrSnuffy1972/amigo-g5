# Game 2256 (railroad tycoon (1990)(microprose)[cr toad][tr sp toad]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02256). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 93 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 23 of 23 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 150 of 150 named (0 author symbols), 150 of 150 described (61 by rule, 73 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 219 of 219 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3162 of 3362 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 72185 |
| code | 10748 |
| data | 787818 |
| padding | 21153 |
| unused | 9216 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SetMap | 13 | 674 | 30 |
| Off | 2 | 52 | 2 |
| Type | 2 | 156 | 3 |
| path | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| loadwb | 2 | 233 | 8 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| e | 1 | 12 | 1 |
| Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:TOAD | 10 | 231 | 6 |
| 16 | 1 | 2 | 1 |
| 19 | 1 | 2 | 1 |
| 1 | 1 | 2 | 1 |
| 4 | 1 | 2 | 1 |
| 7 | 1 | 51 | 2 |
| 9 | 1 | 51 | 2 |
| 17 | 1 | 51 | 2 |
| 20 | 1 | 51 | 2 |
| 9 | 1 | 51 | 2 |
| 5 | 1 | 2 | 1 |
| 8 | 1 | 51 | 2 |
| diskfont.library | 15 | 1040 | 76 |
| Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:bootblock | 1 | 18 | 1 |

## Cases handled

- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:diffs.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:clocos.pix: high-entropy data file (7.86 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:page7.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:locos2.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:page5.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:locos0.pix: high-entropy data file (7.87 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:diffsp.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:council.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:credits2.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:page3.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:elocos2.pix: high-entropy data file (7.89 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:tracks.pix: high-entropy data file (7.90 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:title.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:elocos0.pix: high-entropy data file (7.86 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:page2.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:locosm.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:page0.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:advert.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:elocosm.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:page8.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:britain.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:csprites.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:europe.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:page6.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:locos1.pix: high-entropy data file (7.88 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:locos.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:page4.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:elocos3.pix: high-entropy data file (7.87 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:westus.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:estation.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:elocos1.pix: high-entropy data file (7.83 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:station.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:clocosm.pix: high-entropy data file (7.89 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:labs.pix: high-entropy data file (7.87 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:page1.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:faces.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:esprites.pix: high-entropy data file (7.90 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:sprites.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:elocos.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:page9.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:eastus.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr toad][tr sp toad]/Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:TOAD: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### SetMap

- `initialize_map_data_and_parse_string`: Retrieves a pointer from a global structure, calls restore_patch_state, and then calls load_and_process_keymap to parse a string into a buffer.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `release_recursive_pointer_blocks`: Iterates through a counted table of pointers to recursively release each non-zero entry, then recursively releases additional blocks stored in global variables before finally releasing the block passed as a parameter.
- `save_arguments_and_run`: Stores two arguments from the stack into global variables, then calls a routine with several parameters and a final routine with a fixed value.
- `load_and_process_keymap`: Loads a keyboard keymap from a file or resource, converts the characters in the resulting buffer to lowercase, and stores the keymap data into a table of pointers.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_argument_to_library_stub`: Passes its one incoming argument, a library base pointer read from a global variable, and the address of its own stack argument to the library call stub at offset minus 132.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Off

- `open_libraries_and_run_task`: Program entry of fifty instructions that opens libraries by name, adds an interrupt server, waits until two screen dimension fields reach fixed values, and then removes its task through Exec routines.
- `store_byte_and_or_register`: Stores a byte from a pointer in a3 into a memory location at offset 0 of a5, then performs a bitwise OR on d0 with zero.

### Type

- `parse_and_process_dos_string`: Opens a file and reads its name, then parses a space-delimited string to extract a character and performs a series of memory allocations and data transfers.
- `perform_dos_write_operation`: Calls a DOS library function using a pointer from a register, then sets up registers for a subsequent call to another routine and clears d0 before returning.
- `execute_write_operation`: Performs a write operation using a DOS library function via a sequence of six instructions.

### path

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_long_word_and_branch`: Copies a long word from register d1 to register d0 and then performs a relative branch.

### Assign

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `transfer_long_word_and_jump`: Moves a long word from d1 into d0 and executes a short branch to a different location.

### loadwb

- `parse_amiga_hunk_data`: Parses a data stream by identifying specific hunk type codes and performs memory relocation or data copying based on those types.
- `calculate_memory_offset_and_call`: Decrements a value in d1, performs pointer arithmetic on a5, and if certain conditions are met, calculates a difference between d3 and a pointer from the stack to pass to a library function, then stores a shifted version of the result into a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_bit_stream_2`: Initializes several registers, loads a longword from a pointer in a0, and enters a loop that calls other routines to perform bitwise shifts and data movement between a3 and a2 based on a counter in d2.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: Performs bitwise shifts and rotations on d5 using d4 as a count in a loop, while also reading bytes from a buffer in a3 and writing them to a destination pointed to by a3.
- `execute_trap_and_combine`: This routine moves a word from offset 3213 of the structure in a2 into d4, executes a trap, and performs a logical OR of d2 into d0.

### Disk-Validator

- `update_disk_validation_data`: Calculates a character representation of a value from a global structure, stores it in a table, and performs a series of bitwise operations and memory writes to update various counters and bitmasks within several structures.

### Ram-Handler

- `execute_indirect_jump`: Sets a value in d0, calls a routine using a pointer from a table at an offset from a library base, and then performs an indirect jump using a value from a6.

### e

- `write_constant_values`: Performs a series of logical operations to set d0 to zero and then writes several constant bytes into a memory location at offset 76 of a structure pointed to by a6 plus d0.

### Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:TOAD

- `execute_multiple_routines`: Calls a routine, then calls another routine via a pointer in a1, executes a loop five times calling a routine and a library function, and finally calls a routine twice more while clearing the memory pointed to by a1.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `expand_and_accumulate_data`: Calls a routine via a pointer in a4 multiple times, then enters a loop that calls another routine and adds the result to a destination buffer at an offset determined by a third routine's result.
- `load_address_and_constant`: Loads an address into a1 using a PC-relative offset and sets d0 to two.
- `expand_and_copy_data`: Performs a multi-stage data expansion and copy operation where bytes are read from a source buffer and written to a destination buffer, with the expansion factor and bit-shifting logic determined by values read from a table.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### 16

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### 19

- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.

### 1

- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.

### 4

- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.

### 7

- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `perform_no_operations`: Performs a series of logical OR operations with zero on the d0 register without changing its value.

### 9

- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `perform_null_logical_operations`: Performs a series of logical OR operations with zero on the d0 register.

### 17

- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `clears_d0_register`: Repeatedly performs bitwise OR operations with zero on the d0 register, including a single operation with sixteen on d0, before returning.

### 20

- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.
- `contains_null_data_table`: This routine consists of a sequence of operations that perform a logical OR of zero onto a data register, which results in no change to the register state.

### 9

- `return_constant_9`: Returns the constant an address in d0 to the caller without any other effect.
- `contains_null_data_sequence`: This routine consists of a sequence of operations that perform a logical OR with zero on a data register, which results in no change to the register's state.

### 5

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### 8

- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_conditional_callback`: Calls a function pointer located at an offset from a global variable; if the return value is non-zero, it stores the contents of a register into a structure at an offset of 34 from the returned pointer and calls another function pointer located at a different offset from the global variable.

### diskfont.library

- `jump_to_font_handler`: As part of library code, this routine branches to an offset of 80 from the library base in a6.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `update_font_handle`: As library code, it stores a value from d0 into two global variables, calls another routine using a pointer from a global variable, and if the result is non-zero, calls a second routine using a different pointer from a global variable; if both calls succeed, it updates a linked list by setting a pointer to a new location and clearing a longword at an offset of 4 from that location.
- `calculate_memory_offset`: As library code, it calls a function at a negative offset from a library base, calculates a new pointer by adding an offset to a value from a structure, and then calls another library function.
- `remove_linked_nodes`: As library code, it iterates through a linked list from a pointer in a6, unlinking nodes and calling another routine if a 16-bit value at offset 84 of a node is not greater than zero, then decrements a 16-bit counter in a global variable; if certain conditions are met, it calls two different routines and then unlinks a final node, otherwise it sets bit 7 of a byte at offset 14 of a global variable.
- `update_linked_list`: As library code, it checks two conditions in a structure; if they fail, it sets bit 7 of a byte at offset 14 of the structure in a6, otherwise it performs several library calls and updates a linked list by reassigning pointers.
- `set_status_bit`: As part of library code, this routine sets bit 7 of a byte at offset 14 of the structure in a6 and clears d0.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_branch`: As part of library code, this routine decrements a word at offset 38 of the structure in a6 and branches to offset 38 if the result is not zero; otherwise, it checks bit 7 of a byte at offset 14 of the structure in a6 and branches to offset 38 if the bit is set, or jumps to an offset of 18 from the library base in a6 if the bit is clear.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `populate_font_entry_table`: Iterates through a linked list of font entries to copy string data into a destination buffer and populates a table with metadata including an offset, a length, and two bytes of data from the entry structure.
- `initialize_string_counter`: Loads a pointer from an offset of a4 into a1, sets d4 to 1, and branches to another location.
- `count_null_terminated_string`: Iterates through a null-terminated string starting at the address in a1, incrementing a counter in d4 for each byte processed until a zero byte is encountered.
- `parse_font_data_stream_2`: Copies bytes from a source buffer to a destination buffer in a loop, then iterates through a linked list of font entries to populate a table with metadata including offsets, sizes, and flags.
- `load_and_parse_font_data`: Opens a file using a pointer at offset 8 of a structure, validates a header, and if a specific condition is met, iterates through a data block to copy character information into a destination table.
- `count_null_terminated_string_2`: Iterates through a null-terminated string in a buffer, incrementing a counter for each non-zero byte.
- `copy_font_data_from_disk`: Copies a sequence of bytes from a source buffer to a destination buffer using a loop, then calls a routine to read data from a disk and checks if the read result is successful and matches a specific constant value.
- `parse_font_data_stream`: Reads a data block from a file, validates a header, and iterates through a sequence of font entries by updating a structure with metadata and copying string data into a buffer.
- `return_zero_stub`: As part of library code, this routine clears d0, restores registers d2 through d6 and a2 from the stack, unlocks a6, and returns.
- `calculate_font_offset_3`: As library code, it checks a word at offset 256 of a structure in a1; if that word is zero, it calls a routine that returns a constant value, otherwise it calculates a value based on a word at offset 4 of a structure in a0 and a word from a1, then calls another routine.
- `calculate_font_offset_2`: Subtracts a value from a pointer by checking bitwise differences between bytes in two structures and conditionally subtracting values from a table based on those differences.
- `calculate_font_offset`: Calculates a longword by subtracting a value from a pointer at a table offset determined by shifting a word in d4 left by two, then performs bitwise operations on d0 and d1 using values from a structure in a1 and a table.
- `update_font_index_and_offset`: Calculates an offset from a table using a value in register d4 shifted left by two, subtracts the resulting longword from a pointer at a fixed address into register d2, increments register d4, and loops if d4 is less than seven.
- `return_constant_value`: This library routine returns a constant value without using any arguments or registers.
- `update_font_glyph_metadata`: Iterates through a list of font glyphs to find the one with the largest value, then updates its internal structure by modifying several fields, including a 16-bit value at offset 256, a pointer at offset 30, and a status byte at offset 85.
- `load_font_resource_loop`: Calls another routine and stores the return value into a global variable at an offset of negative twelve from the base pointer in a6, then branches to a previous instruction or continues to another routine.
- `load_and_update_font_data`: Opens a file, reads font data into a memory allocation, and iterates through the font entries to find the largest entry based on a comparison loop.
- `clear_registers_and_branch`: Clears words in d4 and d3, sets d2 to 255, and branches to another location.
- `update_font_resource_table`: Iterates through a list of font entries to find the largest available resource, then performs memory allocation and structure updates for font data.
- `manage_font_allocation`: Calls a library routine, calculates a longword by multiplying a word in d2 by 260, and clears a 16-bit word at offset 256 of a structure pointed to by a pointer derived from that longword and d5.
- `manage_font_resource_allocation`: Calculates an offset based on a word value multiplied by 260, clears a 16-bit value at a specific offset in a structure, and performs a series of conditional calls to free memory, unlock resources, and close files or fonts depending on the input parameters and the state of a structure in a6.
- `build_disk_font_table`: Iterates through a list of font data to populate a new table, copying specific bytes from the source font structures into a newly allocated memory block.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `load_pointers_and_branch`: As part of library code, this routine loads a long word from offset 4 of the stack into a0, loads a long word from offset 8 of the stack into d0, and branches to an offset of 28 from the current instruction.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `initialize_pointer_and_branch`: As part of library code, this routine loads a long word from offset 4 of the stack into a0, clears d0, and branches to an offset of 54 from the current instruction.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `compare_memory_strings`: As library code, it loads two pointers from the stack into registers a1 and a0 and branches to another routine.
- `skip_zero_byte`: Tests a byte at the pointer in a0 and increments the pointer; if the byte is not zero, it branches to another routine, otherwise it clears d0 and branches to another routine.
- `compare_bytes_and_branch`: Compares a byte from a pointer in a0 with a byte from a pointer in a1, and if they are equal, compares a byte from a0 with a byte from a pointer in a1 and branches based on the result.
- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `load_byte_and_branch`: As library code, it loads a pointer from the stack into a0, loads a byte from an offset of 11 on the stack into d0, and branches to another routine.
- `compare_and_move_pointer`: Compares d1 with d0; if they are not equal, it branches to another routine, otherwise it moves the pointer in a0 to d0 and branches to another routine.
- `increment_and_check_byte`: Increments a pointer in a0, loads a byte from the new address into d1, and if that byte is non-zero, clears d0 and returns.
- `process_pointer_with_offset`: As library code, it saves registers d2 and d3 to the stack, loads a long word from offset 12 of the stack into d3, loads a byte from offset 19 of the stack into d2, pushes d3 onto the stack, calls another routine, moves d0 into a0, decrements a0 by 1, adds d3 to a0, cleans up the stack by adding 4 to a7, and branches to another routine.
- `compare_byte_and_move_pointer`: Compares the byte at the pointer in a0 with d2; if they are not equal, it branches to another routine, otherwise it moves the pointer in a0 to d0 and branches to another routine.
- `check_pointer_boundary`: Decrements a pointer in a0, compares it to a value in d3, and if it is not greater than or equal to d3, clears d0 and restores registers d2 and d3 from the stack before returning.
- `process_pointer_list`: As library code, it pushes d2 to the stack, loads a pointer from offset 8 of the stack into d2, and calls another routine using a value from the stack; it repeats this process with a different constant if the first call does not result in a non-zero return value in d0.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_another_routine`: This library routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Railroad Tycoon (1990)(MicroProse)[cr TOAD][tr Sp TOAD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
