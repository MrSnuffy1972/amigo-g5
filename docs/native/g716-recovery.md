# Game 716 (soccer kid (1994)(krisalis software)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00716). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3606388 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 29 of 29 named (0 author symbols), 29 of 29 described (4 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 301 of 301 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 689 of 728 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3283 |
| code | 2626 |
| data | 3599949 |
| padding | 18 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf:bootblock | 1 | 176 | 13 |
| logged disk load from Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf boot-block load | 1 | 116 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 47 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 42 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 84 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 92 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 149 | 4 |

## Cases handled

- soccer kid (1994)(krisalis software)[cr flt](disk 1 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- soccer kid (1994)(krisalis software)[cr flt](disk 2 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 2 of 4).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- soccer kid (1994)(krisalis software)[cr flt](disk 3 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 3 of 4).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- soccer kid (1994)(krisalis software)[cr flt](disk 4 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 4 of 4).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- soccer kid (1994)(krisalis software)[cr flt](disk 1 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf: the boot block reads 3584 bytes from disk offset 618496 to an address (found statically); recovered as a unit
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 299 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 6 first-run image unit(s) in 1 version group(s); 652 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- soccer kid (1994)(krisalis software)[cr flt](disk 1 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf: 1 logged loads cover 3584 bytes of the image
- soccer kid (1994)(krisalis software)[cr flt](disk 1 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf: blocks 1017-1207 (bytes 520704-618496) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.95)
- 5 packed track region(s), 3498496 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf:bootblock

- `setup_interrupts_and_dma`: Configures interrupt and DMA settings by enabling specific bits in hardware control registers, sets up an I/O request structure with command code 2, and executes a library call to perform the I/O operation.
- `install_vectors_copy_block`: Writes three routine addresses into a fixed low memory vector slot, clears a shared word, and copies a block of twenty-seven words from a routine to a fixed memory location.
- `copy_word_block`: Calls a routine that copies words from a source pointer to a destination pointer, advancing both by one word per step until a counter in the data register runs out.
- `copy_word_block_forward`: Copies words from a source pointer to a destination pointer, advancing both by one word per step until a counter in the data register runs out.
- `write_text_marker_by_flag`: Writes the four-character text marker FAT0 to a destination pointer, or FAT1 when a bit of a shared state word is set.
- `detect_refresh_rate_hz`: Resets the CIA port and timer registers, then reads a counter from two bytes and compares it with a fixed threshold.
- `wait_until_status_byte_clear`: Repeatedly checks a status byte at a fixed offset from a base register and returns minus one once its top bit is clear.
- `clear_buffer_and_fill_tags`: Clears a fixed-size buffer, writes two four-character text tags into it, then calls two other routines.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (movea.l, movea.l) and then falls through into the code that follows, with no return of its own.
- `walk_list_from_given_node`: Walks the same kind of linked list starting from the pointer already held in a register, stopping at the first node with the selected bit set or at the end.
- `store_control_word_or_halt`: If a shared flag is clear, it stores a computed control word into a structure and returns.
- `copy_longword_and_tag`: Copies one longword from a source pointer to a destination pointer, then stores a tagged word that combines the data register D0 with a fixed two-letter tag into a structure field.
- `setup_io_request`: Configures an I/O request structure with command code 2, sets up several memory fields, and then executes a library call to perform the I/O operation.

### logged disk load from Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf boot-block load

- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `lookup_and_mask`: Performs a lookup in a table using a calculated index and applies a bitwise AND operation with a value from another register before returning the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_ori_bne_movem_rts`: Short code fragment that merges 128 into d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `frag_tst_beq_movea_bra`: Short code fragment that tests the memory at a1, then branches if the last result was zero, then copies the memory at a1 to a1, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `render_loop`: Contains a loop that repeatedly calls other routines to perform rendering operations, with conditional logic based on various global variables and bit tests.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_joystick_input`: Reads joystick input from hardware registers and performs a series of bit tests on the input data to determine which buttons are pressed, storing the result in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_interrupt_status`: Reads interrupt status from a hardware register, masks specific bits, writes the result back to another register, and then executes a loop that processes interrupt flags.
- `check_and_set_flag`: Tests a byte value at a specific memory address and sets a flag in the data register if the test fails.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_clear`: Decrements a byte-sized global variable and clears a word-sized variable at another global address when the counter goes below zero.
- `clear_interrupts`: Reads interrupt status from a hardware register, masks specific bits, writes the result back to another register, and then executes a loop that decrements a counter while performing no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_interrupt_flags`: Reads a hardware register repeatedly while checking for a specific bit, clears interrupt flags in multiple global variables, and updates structured data with values from registers before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_address_table`: Updates a series of memory offsets with new address values while waiting for a global flag to clear.
- `set_address_offsets`: Writes a fixed value to a memory offset and then updates a pointer-based table with incremental address values.
- `set_address_offsets_alt`: Writes a fixed value to a memory offset and then updates a pointer-based table with incremental address values using a different source register.
- `mask_and_shift_value`: Applies a bitmask to an input value and shifts the result right by three bits.
