# Game 579 (Zool 2): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00579). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1803936 top-level bytes in 78 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 34 of 34 named (0 author symbols), 34 of 34 described (6 by rule, 27 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 648 of 648 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 32183 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 995 of 995 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1413990 |
| code | 2776 |
| data | 297768 |
| padding | 17722 |
| unused | 71680 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Classic | 5 | 253 | 7 |
| Zool 2 (1993)(Gremlin)(AGA)[cr CLS](Disk 1 of 2).adf:z2 | 4 | 223 | 8 |
| Zool 2 (1993)(Gremlin)(AGA)[cr CLS](Disk 2 of 2).adf:installer | 3 | 223 | 8 |
| Zool 2 (1993)(Gremlin)(AGA)[cr CLS](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 286 | 10 |
| Zool 2 (1993)(Gremlin)(AGA)[cr CLS](Disk 2 of 2).adf:mng.sod (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- zool 2 (1993)(gremlin)(aga)[cr cls](disk 2 of 2)/Zool 2 (1993)(Gremlin)(AGA)[cr CLS](Disk 2 of 2).adf:mng.toomus: high-entropy data file (7.69 bits/byte), classified as packed data
- runtime segment an address: 235 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- zool 2 (1993)(gremlin)(aga)[cr cls](disk 2 of 2)/Zool 2 (1993)(Gremlin)(AGA)[cr CLS](Disk 2 of 2).adf:mng.sod: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- zool 2 (1993)(gremlin)(aga)[cr cls](disk 2 of 2)/Zool 2 (1993)(Gremlin)(AGA)[cr CLS](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- zool 2 (1993)(gremlin)(aga)[cr cls](disk 1 of 2)/Zool 2 (1993)(Gremlin)(AGA)[cr CLS](Disk 1 of 2).adf:z2: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- zool 2 (1993)(gremlin)(aga)[cr cls](disk 2 of 2)/Zool 2 (1993)(Gremlin)(AGA)[cr CLS](Disk 2 of 2).adf:installer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 32183 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Classic

- `init_and_check_pattern`: Saves the stack pointer at offset 60 on the stack, then calls a routine three times with the values one, zero and three in d6.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `increment_counter_and_copy_buffer`: Increments a 16-bit counter in d6, calls a routine twice, then runs a short loop that calls the routine and copies a computed value into a destination buffer.
- `load_address_set_value`: Loads the address at offset 1fc into a1 and places the value two into d0, then returns.
- `copy_byte_run_from_buffer`: Copies a run of bytes from the source buffer in a0 to the destination buffer in a3, decrementing both pointers, using a length counter in d2 and a repeat counter in d5.
- `push_values_and_branch`: Pushes a zero and several register values onto the stack, then restores the caller's registers and branches back to an earlier instruction of the calling routine.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Zool 2 (1993)(Gremlin)(AGA)[cr CLS](Disk 1 of 2).adf:z2

- `extract_flag_bit_from_table_3`: Walks a table of word pairs until a global mask no longer matches, then reads a byte from the table, subtracts it from a global value, and calls another routine to process the result.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a global mask no longer matches, then reads a byte from the table, subtracts it from a global value, and calls another routine if a borrow is detected.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: Masks a word register with a global mask and, if a global byte is less than another byte register, calls another routine to supply the result; otherwise it returns the masked word.
- `combine_table_bytes`: Combines bytes from a table into d6, adjusts d1 by a value in d7, and returns without modifying memory.
- `fill_pattern_table`: Reads a count from a global variable, decrements it, and returns immediately if it was negative.
- `process_zero_terminated_list`: Walks a linked list of zero-terminated entries while clearing them, then for each entry follows a chain of pointers and adds four to the stored value at an indexed location.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Zool 2 (1993)(Gremlin)(AGA)[cr CLS](Disk 2 of 2).adf:installer

- `extract_and_copy_compressed_block`: Walks a linked list of header blocks, and for each block whose identifier matches a fixed address, copies its data into a destination buffer while aligning it to even addresses and applying a byte mask, then copies a trailer of bytes.
- `extract_flag_bit_from_table_2`: Scans a table of word pairs, advancing two words each time, until the global mask ANDed with the first word differs from the second word.
- `set_values_in_registers`: Places the value two hundred and fifty-five into d0 and the value sixteen into d1, then returns.
- `mask_and_conditional_combine`: Masks the 16-bit value in d0 with the 16-bit value in d6, then subtracts the byte in d1 from the byte in d7; if that byte is greater, it calls combine_table_bytes and returns its result, otherwise it returns the masked value.
- `return_empty_routine`: It contains no instructions that modify memory or registers and immediately returns.
- `fill_figure_table`: Fills a stack buffer with a run of values from a global mask routine, then walks a key list in that buffer and, for each matching key, writes a bit mask, a shifted value, and three derived bytes at fixed offsets into a table.
- `accumulate_linked_records`: Walks through a chain of linked records pointed to by a4, clearing each record's first three longwords and, for every record, adding a computed pointer plus four to the longword at the offset given by the record's value in the table in a5, repeating until a record's value is zero.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### Zool 2 (1993)(Gremlin)(AGA)[cr CLS](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_memory_and_raster_loop`: Allocates 224 bytes of memory and copies a 56-word table from a fixed location into it, then returns if the allocation fails.
- `store_two_words_global`: Initializes a counter to two, then repeatedly calls another routine and stores its returned word into a global color register and a source table, advancing each destination pointer after the store.
- `adjust_nibbles_combine`: Extracts the low nibble, the next nibble and the third nibble of d0 and d3, then increments or decrements each differing nibble toward the target before combining them into a single value and returning.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `clear_memory_array`: Loads a pointer from a global variable and writes zero to each of its eight thousand entries in a loop, then returns.
- `call_open_libraries`: Reads a global pointer, calls the routine stored at offset 552 from it twice, and stores each returned value into consecutive memory locations.
- `skip_string_lines`: Calls another routine, then loops over a string advancing the pointer until it finds a newline or a zero terminator, incrementing a global word each time it skips a line.
- `count_string_bytes`: Walks a byte string from a0, incrementing a counter for each byte until it reaches a newline or a zero terminator, then calls a routine at a global offset when the count is non-zero.
- `fill_stack_buffer_and_call`: Loads several fixed addresses into a stack buffer at fixed offsets, then calls a routine at a global library offset and jumps to another routine at a different global offset.
- `call_and_jump_globals`: Loads a global pointer into a1, calls a routine at a global library offset, then loads the same pointer again and jumps to another routine at a different global offset.
