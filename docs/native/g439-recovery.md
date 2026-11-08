# Game 439 (Banshee): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00439). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3606960 top-level bytes in 97 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 60 of 60 named (0 author symbols), 60 of 60 described (10 by rule, 44 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1005 of 1005 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3356 of 3356 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 337028 |
| code | 10134 |
| data | 2900757 |
| padding | 17537 |
| unused | 341504 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:picture.exe | 3 | 111 | 3 |
| Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:assign | 1 | 938 | 7 |
| Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:cpu | 1 | 879 | 16 |
| Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:bans.exe | 7 | 235 | 8 |
| Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:Installer | 2 | 267 | 8 |
| Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:DELIRIUM | 6 | 205 | 7 |
| Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 703 | 10 |

## Cases handled

- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:endseq1.002: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3574 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:endseq1.003: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3619 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:endseq1.004: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4183 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:endseq1.005: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5130 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:P60.intro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (16105 -> 24730 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:endseq2.001: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3580 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:endseq2.002: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3602 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:endseq2.003: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3670 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:endseq2.004: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4284 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:endseq2.005: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5489 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:P60.end: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (23003 -> 33512 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:bossc: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (109557 -> 222488 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:bossf: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (43918 -> 76416 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:introseq.001: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4342 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:introseq.002: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8254 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:introseq.003: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5314 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:introseq.004: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6908 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:introseq.005: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3954 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:introseq.006: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7927 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:introseq.007: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11429 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:endseq1.001: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3579 -> 40960 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 2 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 2 of 4).adf:P60.calm bottom4: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (81292 -> 106038 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 2 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 2 of 4).adf:soundfx: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (129459 -> 153946 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 2 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 2 of 4).adf:plyrmisc: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (46244 -> 147912 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 2 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 2 of 4).adf:introscreen: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (100931 -> 164864 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 2 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 2 of 4).adf:level1c: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (272677 -> 487644 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 2 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 2 of 4).adf:level1f: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (158929 -> 325868 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 2 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 2 of 4).adf:core: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3079 -> 81920 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 3 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 3 of 4).adf:level3c: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (248768 -> 469264 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 3 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 3 of 4).adf:P60.calm bottom4: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (81292 -> 106038 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 3 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 3 of 4).adf:banspic2: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (21894 -> 164864 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 3 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 3 of 4).adf:level2c: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (281827 -> 500136 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 3 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 3 of 4).adf:level2f: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (163413 -> 336436 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 4 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 4 of 4).adf:soundfx4: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (108871 -> 128584 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 4 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 4 of 4).adf:level3f: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (156407 -> 342238 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 4 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 4 of 4).adf:level4c: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (216291 -> 399448 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 4 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 4 of 4).adf:level4f: high-entropy data file (7.86 bits/byte), classified as packed data
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 4 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 4 of 4).adf:P60.calm bottom4: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (81292 -> 106038 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 4 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 4 of 4).adf:plyrmisc4: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (35017 -> 92444 bytes)
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 4 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 4 of 4).adf:banspic1: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (37379 -> 164864 bytes)
- runtime segment an address: 585 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf: blocks 11-21 (bytes 5632-11264) hold ASCII text
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:bans.exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:Installer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- banshee (1994)(core design)(aga)(m4)[cr dlm](disk 1 of 4)/Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:DELIRIUM: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:picture.exe

- `initialize_palette_and_wait`: Copies a block of longwords from a source table into a global variable, then fills two tables with pointers to that variable and initializes a third table with decoded palette entries.
- `jump_computed_address`: Loads a pointer from a global table, adds a fixed offset to it, and jumps to the resulting address without returning.
- `store_pair_in_loops`: Loops seven times, storing the low word of a value from a global variable at offset 6 and the high word at offset 2 of a structure in a0, then advances the source pointer by 80 and the destination pointer by 8 each iteration.

### Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:assign

- `parse_assign_command`: Parses a command-line assignment string of the form 'Name:' and validates the device name and options, then prepares internal state for adding, subtracting, deferring or setting paths, returning an error code when the input is invalid.
- `check_volume_mount`: Checks a volume reference stored in a global table and tests whether the volume is mounted.
- `copy_bytes_to_buffer`: Checks the length field of a buffer descriptor and, when the destination is too small, allocates a new buffer through a global routine pointer.
- `process_assignment_queue`: Walks a linked list pointed to by the first argument, invoking a handler stored at a fixed offset from the node and then a second handler in a loop controlled by a counter field.
- `assign_object_to_slot`: Attempts to assign an object identifier to a slot list, checking for duplicates and capacity using a global variable and another routine.
- `copy_name_to_buffer`: Copies characters from a source string into a destination buffer until a colon or null terminator is reached, stopping at thirty characters.
- `compare_strings_case_insensitive`: Compares two byte strings pointed to by the arguments character by character, treating letters case-insensitively by uppercasing them.

### Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:cpu

- `initialize_supervisor_state`: Initializes a small supervisor state structure and a set of function-pointer tables, then inspects a status byte at offset 297 of a structure to select one of several initialization paths, calling the appropriate helper routines and returning a small constant.
- `run_conditional_sequence`: Calls a routine through a global pointer and, depending on whether the result is zero, invokes a fixed sequence of four other routines through pointers stored at different offsets.
- `allocate_and_fill_memory_blocks`: Allocates three blocks of 512, 1024 and 256 bytes from the memory manager, then fills each with a sequence of words whose high half increments by one million and whose low half is set to one, and writes a two-bit marker into a structure.
- `allocate_and_fill_memory_structures`: Allocates three blocks of 512, 1024 and 256 bytes from the memory manager, then fills the first block with 256 words whose high half increments by 65536 and whose low half is one, sets the second block's final 256 words with a high half counting down from two billion, and zeroes the low byte of the third block.
- `configure_supervisor_state`: Calls two helper routines on the stack, then reads a rounded pointer from a global variable and branches on a masked value.
- `invoke_pointer_routine_2`: Loads a pointer from a global variable and calls the routine located at the address stored at offset -30 of that pointer.
- `patch_routine_pointer`: Tests bit 3 and bit 2 of a byte at offset 297 in a structure, and when both are clear it stores the address of a fixed routine into a function pointer at offset 50 of another structure.
- `handle_stack_value`: Pops a 16-bit value from the stack and compares it against eleven.
- `clear_stack_value_and_return`: Clears the 32-bit value at the address pointed to by the stack pointer and advances that pointer by 4 bytes before returning.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_supervisor_2`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `invoke_supervisor_call`: Loads the exec library base, saves the frame pointer, sets up a stack frame, and calls a routine through a pointer stored at offset 30 of that base.
- `call_supervisor_3`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `invoke_pointer_routine`: Loads a pointer from a global variable, calls the routine located at the address stored at offset -30 of that pointer, then restores the previous value and returns.
- `call_supervisor_4`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `compare_strings_case_insensitive_2`: Reads one byte from each of two null-terminated strings pointed to by the registers, converts lowercase letters to uppercase, and compares them.

### Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:bans.exe

- `initialize_memory_structures`: Allocates memory via the exec library, then scans a linked list of structures, copying and bit-reversing the data block of each one whose identifier matches the expected value, and finally frees the memory list entries.
- `extract_flag_bit_from_table`: Scans a table of word pairs, advancing two words each time, until the global mask in d6 ANDed with the current word does not equal the following word, then reads a byte at offset 60 and subtracts it from the global byte in d7.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: Masks the word in d0 with the global word in d6, then if the global byte in d7 is less than the byte in d1 it calls another routine and uses its returned word, otherwise it returns the masked word.
- `return_empty_routine`: Loads two bytes from a table pointed to by a3 into a 32-bit register, performs a few arithmetic adjustments on a value in d1, and then returns without modifying memory or any other register.
- `fill_figure_table`: Fills a table with values from a stack buffer, then iterates through a loop to write bit masks and derived values into the table based on a key list.
- `free_memory_list_entries`: Checks whether the four-byte value at the register points to the identifier 100, and if so walks a linked list of pointers, zeroing fields and accumulating their values into a table.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:Installer

- `process_hunk_format_buffer`: Walks through a hunk-format buffer, matching executable-type markers, and for each one copies the following data block to a new location while patching the pointer fields in place.
- `decrement_counter_and_maybe_init`: Decrements a 16-bit counter in d1 until it reaches zero, then returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `shift_and_copy_buffer_bytes`: Reads a byte from memory, and when it is non-zero it shifts and indexes into a buffer, then copies bytes backward through the buffer in a loop that runs until the pointer reaches the start.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `rotate_global_right_loop`: Loops a number of times given by d0, rotating the global value in d5 right by the low six bits of d6 into an accumulator in d1 and ORing the carried bit into the result, reloading the source from the stack when the shift reaches zero.
- `invoke_system_trap`: Loads a word from memory into a register, sets bits 3 and 7 of another register, invokes a system trap, and combines the result into that register before falling through to the following code.

### Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:DELIRIUM

- `free_memory_clear_slot`: Saves the registers on the stack, calls two unnamed routines, then calls another unnamed routine and frees a memory block through exec.FreeMem.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `initialize_bit_stream_tables`: Loads two setup values from a global table, clears two 16-entry tables, then fills two tables with values from another routine and runs a shift-based bit stream to pack bytes into a third table.
- `shift_bit_stream`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `initialize_table_values`: Iteratively calculates and stores new values into a table using a pointer.

### Banshee (1994)(Core Design)(AGA)(M4)[cr DLM](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_copper_and_palette`: Loads graphics.library and its interface, then clears the copper list and custom-chip registers and copies a block of saved values into the copper.
- `swap_and_store_words`: Swaps the order of four consecutive 32-bit values in a global variable, then waits in a loop until a 16-bit field in a structure pointed to by a6 matches a fixed value.
- `compute_and_pack_bitmap_blocks`: Reads two 16-bit values from a global variable, scales them by two more values, and writes a series of computed 16-bit results into consecutive words of a table via a pointer, repeating the block twelve times.
- `process_table_byte`: Increments a global counter, reads a byte from a table pointed to by a computed address, and returns immediately when that byte is zero.
- `copy_words_from_global`: Copies 31 words from a global variable at offset 504 to a table starting at a fixed address, advancing the source by one word and the destination by two words each iteration, then returns.
- `adjust_table_nibbles`: Reads 31 words from a table and 31 words from a global variable, extracts their nibbles, and adjusts each extracted nibble toward the corresponding global nibble by one in the direction of its sign, then recombines and stores the result back into the table.
- `wait_flag_and_store`: Waits until a flag bit in a structure in a6 clears, then stores a pointer, a large constant, a cleared word, and a value into fields of that structure before returning.
- `manage_custom_flag_states`: Saves the registers, points a6 at the custom chip, and checks three consecutive global flag bytes.
- `store_register_and_rte`: Stores the 32-bit value passed in a register into a global variable at offset 92 and returns via a supervisor return instruction.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
