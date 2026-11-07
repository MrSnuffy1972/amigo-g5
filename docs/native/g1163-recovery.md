# Game 1163 (F29Retaliator v1.1 0022): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01163). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4110059 top-level bytes in 21 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 38 of 38 units identical (bytes and relocations), 159 raw-word lines |
| Functions named and described | pass, 208 of 208 named (0 author symbols), 208 of 208 described (18 by rule, 189 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1448 of 1482 (97.71%) |
| Recovered without load address (counted in proof 1, flagged) | 4608 bytes, 5 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4056 of 4345 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 526942 |
| code | 18830 |
| data | 3564221 |
| padding | 66 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 358 | 35 |
| Disk.1:bootblock | 1 | 125 | 4 |
| Disk.2:bootblock | 1 | 125 | 4 |
| logged disk load from Disk.1 boot-block load | 1 | 413 | 24 |
| logged disk load from Disk.1 boot-block load | 1 | 0 | 0 |
| logged disk load from Disk.2 boot-block load | 1 | 413 | 24 |
| logged disk load from Disk.2 boot-block load | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 539 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 889 | 41 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1017 | 45 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 1 |
| Disk.1 bytes 896512-901120 (code never loaded by a reference run) | 1 | 369 | 5 |

## Cases handled

- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.1: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.2: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022.hdf: hard-disk image (AmigaDOS OFS, 4384 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree F29Retaliator_v1.1_0022_hdf, which is recovered file by file; no block outside the filesystem holds data
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.1: the boot block reads 5632 bytes from disk offset 12288 to an address (found statically); recovered as a unit
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.1: the boot block reads 204800 bytes from disk offset 18944 to an address (found statically); recovered as a unit
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.2: the boot block reads 5632 bytes from disk offset 12288 to an address (found statically); recovered as a unit
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.2: the boot block reads 204800 bytes from disk offset 18944 to an address (found statically); recovered as a unit
- 21 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 107 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 609 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f300): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f300): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 466 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 11 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 74 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.1: bytes 896512-901120 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying code at an address (10 bytes change between runtime segment an address and runtime segment an address (f300)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address and runtime segment an address (f300)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (f300) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (f300) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address and runtime segment an address (f300)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (f300) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.1: 20 logged loads cover 294912 bytes of the image
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.1: 1 track-loaded blocks found in recovered runtime segments and classified through them
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.1: blocks 35-36 (bytes 17920-18944) outside the filesystem hold data, not code (opcode density 0.014, entropy 4.49)
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.1: blocks 437-967 (bytes 223744-495616) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.77)
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.1: blocks 1001-1143 (bytes 512512-585728) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.36)
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.1: blocks 1199-1407 (bytes 613888-720896) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.62)
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.1: blocks 1463-1616 (bytes 749056-827904) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.53)
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.2: 2 logged loads cover 210432 bytes of the image
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.2: blocks 2-23 (bytes 1024-12288) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.71)
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.2: blocks 35-36 (bytes 17920-18944) outside the filesystem hold data, not code (opcode density 0.014, entropy 4.49)
- F29Retaliator_v1.1_0022/F29Retaliator_v1.1_0022_hdf/F29Retaliator/Disk.2: blocks 437-1759 (bytes 223744-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.73)
- 2 packed track region(s), 72704 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_ciab_and_loop` (Disk.1 boot-block load +an address to an address (5632 bytes)): Writes 0 to the CIAB CRA register, writes 100 to the CIAB timer A high byte and 8 to the CIAB timer A low byte, clears bit 0 of the CIAB port B register three times and sets bit 0 once, clears bit 2 of a global byte, writes 25 to the CIAB CRA register, tests bit 2 of that global byte, and loops forever unless the bit is set, in which case it returns. Kind: CIAB timer/PRB-based timing check (long-track/sync style). Pass path: The routine configures CIAB (timer + PRB input) and loops; the check passes by taking the bne.b branch at sub_0610 off 648 (after subq.w #an address,an address.l; cmp.w an address.l,d0) when the measured timing matches, i.e. the loop exits via the passing br.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `install_rotation_trap_handler`: Reads a 16-bit value from the structure passed in a0 and, if it equals 15318 or 15529, rotates the 32-bit words starting at offset 29214 by a computed amount, then installs the trap vector 1253 at that address and stores the routine's own address at offset 1548.
- `install_trap_vector`: Stores the trap vector constant into a custom register, pushes a program-relative address onto the stack, pops it into another custom register, then jumps away.
- `process_input_and_fill_vector_table`: Computes a value from the 16-bit input in d0, then XORs successive results into a run of global words at offsets 1548 through 1560.
- `initialize_vector_table_and_clear_memory`: Initializes a table of jump vectors with trap and return opcodes, then clears a block of memory at the video register base by writing zero words in a loop.
- `store_byte_and_call_handler`: Stores the decimal value 16 into the byte at global offset 1586, then branches to the vtable handler routine.
- `set_global_and_call_handler`: Stores the decimal value 16 into a global variable, then invokes the handler at offset 52 of the structure referenced by the global at offset 1586 with the fixed address an address as its first argument, and returns.
- `store_one_call_handler`: Stores the value 1 into a global variable, calls a routine that sets a global byte and invokes a vtable handler, then branches to another routine.
- `invoke_vtable_handler_2`: Sets a global byte to one, calls a handler routine, then invokes a vtable handler at offset 52 with a fixed address argument and returns.
- `set_bytes_and_loop_check`: Branches to the vtable handler routine, then sets the byte at offset 1586 and the byte at offset 1587 of the global structure to 255, and loops while the word at offset 1586 stays below 58881.
- `loop_until_byte_zero`: Branches to the vtable handler routine, then loops until the byte at offset 1586 of the global structure becomes zero, and otherwise returns.
- `call_indirect_stub`: Jumps to the routine at the address held at offset a fixed address and never returns from this instruction.
- `invoke_vtable_handler`: Invokes the handler at offset 52 of the structure referenced by the global at offset 1586 with the word a fixed address as its first argument, then returns the values left in registers d0 and d1.
- `swap_registers_between_stacks`: Copies the registers d1, d2, d6, d7 and a1 from the stack into registers, then stores those same registers back onto the stack at a different address, and finally branches to the routine at offset 51e.
- `call_library_entry`: Switches to supervisor mode, loads the library base from the global at offset 1586 into a6, calls the entry point at offset 32, restores the previous state, and returns.
- `fill_pointer_table`: Stores the address in a0 at the two consecutive locations pointed to by a1, then stores the address of this routine at the next location and returns.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a3, so the target is only known at run time.
- `init_and_jump`: Writes the word 17017 to the two memory locations at offsets 30910 and 31584, sets bit a fixed address of the long at offset -4 from a0, loads a constant into d0, and jumps to the address at offset a fixed address.
- `set_flag_and_jump`: Writes the word 17017 to the location at offset 31366, loads a constant into d0, and jumps to the address at offset a fixed address.
- `set_flag_11e8`: Stores the word 1 at the location at offset 4584 and returns.
- `set_flag_1200`: Stores the word 1 at the location at offset 4608 and returns.
- `clear_flag_11ee`: Clears the word at the location at offset 4590 and returns.
- `clear_flag_1206`: Clears the word at the location at offset 4614 and returns.
- `store_one_and_jump`: Loads the address of a global variable at offset 1592 into a0, loads the value 1 into d0, stores that 32-bit value through the pointer in a0, then jumps to the routine at 32152.
- `store_one_and_jump_2`: Loads the address of a global variable at offset 1592 into a0, loads the value 1 into d0, stores that 32-bit value through the pointer in a0, then jumps to the routine at 31996.
- `store_two_and_jump`: Loads the address of a global variable at offset 1592 into a0, loads the value 2 into d0, stores that 32-bit value through the pointer in a0, then jumps to the routine at 32452.
- `store_two_and_jump_2`: Loads the address of a global variable at offset 1592 into a0, loads the value 2 into d0, stores that 32-bit value through the pointer in a0, then jumps to the routine at 32296.
- `branch_through_two_routines`: Jumps to a routine at offset 1472 and then jumps to a routine at offset 1504, passing control to each in turn.
- `call_multiply_branch_clear`: Calls a routine that multiplies a value by 5632 and invokes a handler, then branches to another routine that stores the result and clears a global word.
- `multiply_and_call_routine`: Saves all registers on the stack, multiplies the value in d0 by the constant 5632 held in d1, loads a base address from a global variable into d2 and the routine table into a6, then calls the routine at offset 40 in that table before restoring all registers and returning.
- `store_handler_result`: Calls a handler routine, stores the result in a global word, clears another global word, adjusts a pointer by adding 1600, and returns.
- `invoke_and_clear_handler`: Stores the result of the handler routine into a global variable, clears the 16-bit value at another global offset, and then returns.
- `invoke_handler_no_args`: Invokes the handler at offset 56 of the structure referenced by the global at offset 1586 with no arguments, then returns.
- `tiny_ori`: Small routine of 1 instruction that merges 1 into d0.
- `decrement_and_load_word`: Decrements the 16-bit value at offset 29547 from a1 by one, then loads a 32-bit word from the address computed by that decremented value plus offset -94 into d7.

### Disk.1:bootblock

- `store_address_and_raise_trap`: Loads the address of a nearby table into a register, stores that address into a global variable at offset 140, and raises a processor trap.
- `install_trap_vectors`: Copies a fixed 15-word table into a global buffer, then copies a 145-word table into the system vector table at offset a fixed address and writes a trap vector constant into the COP1LCH register.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Disk.2:bootblock

- `install_system_traps`: Loads the address of a 15-word table into a register, stores that address into a global variable, then invokes trap 3 to install the system vector table.
- `fill_and_setup_structures`: Fills a 16 KiB region of memory with a repeating 179-byte pattern, then copies a 19-byte table into a second memory region and writes five fixed values after it.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `load_and_verify_checksum`: Initializes a global variable from a passed value, then copies a block of 32-bit words from a source table into the buffer at address 2056 using a loop that counts down from 255.

### logged disk load from Disk.1 boot-block load

- `do_nothing_branch_out`: Does nothing, since the single instruction is an unconditional branch that leaves the routine.
- `adjust_counter_and_call`: Saves several registers on the stack, loads a value from a global variable, calls another routine, restores the registers, adjusts a counter, and calls a further routine.
- `copy_register_then_call`: Copies the contents of register d1 into register d2, calls another routine, then branches away.
- `copy_palette_bytes`: Loads a palette register address, copies a byte from a source block into it while a counter decrements, then restores registers, calls another routine, and returns when a counter reaches 160.
- `write_color00_infinite_loop`: Decrements d7, writes it to the COLOR00 palette register, and branches back to an earlier instruction to form an infinite loop that does nothing else.
- `set_flags_and_call`: Sets bit 7 of a global variable, calls another routine, sets bits 6 through 3 of the same global, and then returns.
- `process_decompressed_stream`: Clears a 512-bit flag table by testing each bit of a 64-bit value and setting the matching flag, then copies a decompressed byte stream into a destination buffer until a counter reaches zero.
- `copy_byte_compare_pointer`: Copies a byte from a source pointer into a register, compares the pointer with a constant, and returns if the comparison is below, otherwise setting the pointer to a fixed value.
- `scale_and_copy_table`: Reads a long from a global variable, divides it by 512, keeps the low 16 bits, and calls a routine with that value.
- `branch_infinite_loop`: Loads a value into a register and branches to another routine, forming an infinite loop that never returns.
- `clear_color00_infinite_loop`: Writes d0 to the COLOR00 palette register, clears that register, and branches back to an earlier instruction to form an infinite loop that does nothing else.
- `sum_table_entry_pairs`: Initializes a running total to zero, then loops 128 times adding both longs of each consecutive pair from the table at 2056 to the total, and returns with the sum.
- `clear_ciab_portb_bit7`: Clears bit 7 of the CIAB port B register twice in a row and runs straight into the next routine without returning.
- `set_ciab_bits_clear_one`: Sets bits 3 to 6 of the CIAB port B register, then clears the bit selected by the input register plus three, restores d1, and returns.
- `set_port_bits_call`: Reads a value into a register, sets a bit in a port register twice, calls another routine, and sets several bits in the same port register before returning.
- `set_globals_and_call`: Stores the value 4 into a global word, stores the value 166 into another global word, calls a routine passing a register value, then restores that value from the stack and returns.
- `adjust_global_value_bits`: Reads a global byte and an input value, toggles a couple of its bits, and returns early if the values match, otherwise stepping a global value up or down by two while comparing it to a routine result until the bits no longer match.
- `configure_ciab_and_loop`: Writes 0 to the CIAB CRA register, writes 100 to the CIAB timer A high byte and 8 to the CIAB timer A low byte, clears bit 0 of the CIAB port B register three times and sets bit 0 once, clears bit 2 of a global byte, writes 25 to the CIAB CRA register, tests bit 2 of that global byte, and loops forever unless the bit is set, in which case it returns.
- `wait_and_process_disk_flags`: Initializes several global variables, waits in a loop until a hardware register bit clears, then repeatedly calls a routine while decrementing a counter and testing its bits before returning when the counter reaches zero.
- `wait_and_configure_disk_dma`: Waits in a loop until a global byte has bit 5 clear, stores 38144 in a DMA control register and 17545 in a global long and a pointed-to short, then waits until the global long has bit 1 set.
- `skip_marker_longwords`: Walks a run of input longwords, skipping past every value equal to 17545, and returns the two longwords found at the ends of the run.
- `decode_interleaved_longword_pairs`: Reads two consecutive input longwords and writes one output longword whose even bits come from the first value and whose odd bits come from the second value doubled, repeating until the count in d0 reaches zero.
- `run_blitter_with_masks`: Waits for the blitter-free flag in the blit-status register to clear, then programs the mask, modulo, pointer and control registers and starts a blit whose size is derived from the input value before waiting for it to finish.
- `xor_checksum_longwords`: XORs together a run of longwords whose count is the input value divided by four, and returns the result.

### logged disk load from Disk.2 boot-block load

- `branch_to_other_routine`: Performs a single unconditional branch to another routine and does nothing else.
- `call_vector_and_check_range`: Loads a vector from a system table into a register and calls that routine, then returns immediately if the returned value is less than or equal to 5632, otherwise subtracts 5632 from the value and calls another routine.
- `set_flags_and_call_2`: Sets bit 7 of a global variable, calls another routine, then sets bits 3 through 6 of the same global variable before returning.
- `copy_bytes_set_palette`: Loads a palette register address, copies bytes from a1 to it in a loop, then sets up registers, calls a routine, and returns.
- `decrement_color_loop`: Decrements a register, writes its value to the color register, then jumps back to itself to decrement and write the value again, forming an endless loop.
- `flag_global_bits`: Sets bit 7 of a global variable, calls another routine, then sets bits 3, 6 and 7 of that same global variable before returning.
- `clear_and_copy_bitmap_block`: Clears a byte at the fixed address 4310 and then, for each of 255 iterations, sets or clears a bit of a bitmap based on a 64-bit value and decrements a counter, calling another routine when the counter is zero and the low three bits are zero.
- `advance_decompressed_stream_pointer`: Reads a byte from the source pointed to by a3 into a data register, advances that pointer, and compares the pointer against a constant; when the pointer is less than or equal to the constant it branches back to read the next byte, otherwise it resets the pointer to a fixed address.
- `divide_copy_longwords_add`: Divides a longword stored in a global variable by 512, calls a routine at address 4786, then copies 128 longwords from the source in a0 into a table starting at address 3080 using a decrementing loop, and finally adds 512 to that global variable.
- `loop_clear_color00`: Loads a fixed longword into d0, writes it to the COLOR00 palette register, clears that register, and branches back to an earlier instruction to form an infinite loop.
- `clear_color_register_loop`: Writes the byte in d0 to the COLOR00 hardware register, clears that same register, and then branches back to its own start, forming an endless loop that only touches the color register.
- `sum_table_entry_pairs_2`: Initializes a running total to zero, then loops 128 times adding both longwords of each consecutive pair in a table at address 2056, and returns the accumulated sum.
- `clear_ciab_port_b_bit`: Clears bit 7 of the CIAB port B register twice in a row and then falls through into the following routine without returning.
- `set_ciab_bits_clear_one_2`: Sets bits 3 to 6 of the CIAB port B register, then clears one bit selected from the input byte plus three, and returns.
- `set_ciab_portb_bits`: Sets bit 7 of the CIAB port B register, calls a routine that sets bits 3 to 6 of that register, then sets bits 3, 4, 5 and 6 of the same register before returning.
- `store_constants_adjust_value`: Stores the value 4 into a global variable and the value 166 into another global variable, then calls a routine that adjusts a global value by toggling bits, and returns the input value.
- `process_joystick_ciab`: Reads a joystick value, updates a global flag, then loops calling a routine that configures the CIAB timer and port while incrementing or decrementing a counter based on the sign of the difference.
- `init_ciab_check_flag`: Initializes several CIAB and timer registers, manipulates bits in the CIAB port B register, checks a flag at address 7154, and branches to address 1800 when that flag is clear, otherwise it returns.
- `configure_disk_dma_and_skip_markers`: Initializes a global variable to a fixed value and calls a handler routine, then waits until a status byte's bit 5 clears before writing a DMA control register value and a global long, and finally walks a run of longwords skipping every value equal to 17545 to return the longwords at each end.
- `wait_program_dma_interrupts`: Waits until a flag at address 32 of a global variable is clear, programs the DMA and interrupt registers, waits until bit 2 of that global variable is set, then writes a sequence of values into the global variable and a longword through the input pointer, and returns.
- `decode_interleaved_checksum`: Scans a table of short values looking for the marker 17545, then decodes interleaved longword pairs, runs the blitter with masks, and computes an XOR checksum before returning a combined longword.
- `decode_interleaved_longword_pairs_2`: Computes a count from the input value divided by four minus one, then loops over consecutive longword pairs, combining the odd-bit parts of each pair doubled into the destination buffer, and returns.
- `run_blitter_with_masks_2`: Waits until a flag at address 64 of a global variable is clear, programs the blitter masks, data, modulo, pointer and control registers using values derived from the input and pointers, starts a blit whose size comes from the input, waits until the blitter finishes, and returns.
- `xor_checksum_longwords_2`: Divides the input value by four and subtracts one to get a count, then loops XORing each longword from the source pointer into a running value, masks the result with a fixed address, and returns it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_address_and_trap`: Loads the address of a small block of data that follows the instruction into a0, stores that address into a global variable at offset 140, then invokes the operating system via trap 3 and never returns from this routine.
- `run_ticker_loop_2`: Initializes a global counter to a sentinel value, then repeatedly decrements it and calls a series of subroutines whose behavior depends on the counter's value.
- `run_ticker_loop`: Initializes a global variable to a sentinel value, then repeatedly decrements it and calls a series of subroutines, branching on its value to update a structure field and invoke a handler, looping until the counter reaches zero.
- `return_immediately_2`: Returns immediately without reading or writing any data.
- `store_table_word_via_index`: Loads a 16-bit value from a table indexed by a global counter, then stores the low byte at offset 92 of the structure in a6 and the high byte at offset 93, decrementing the counter and repeating until it reaches zero.
- `decrement_counter_and_call`: Decrements the 16-bit counter at offset 32026 by one and returns early if it goes negative.
- `swap_pairs_and_call`: Swaps the two 32-bit values stored at offsets a fixed address and a fixed address of a global structure, saves the original value of a fixed address to offset a fixed address, swaps the two 32-bit values at offsets 192 and 196, then calls the routine at a fixed address and returns.
- `swap_global_longwords`: Swaps the two consecutive longwords at offsets 219 and 220 of a global variable, then swaps the two consecutive longwords at offsets 192 and 196 of the same variable, and finally calls another routine before returning.
- `copy_prev_long_from_global`: Loads a long from a global variable into a pointer, copies the four bytes preceding that pointer into the pointer's target, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_longword_backward`: Loads a longword from the address in a1 minus 4 bytes and stores it back at the address in a1, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_and_clear_memory`: Returns from an exception, calls a routine that clears a memory block by repeatedly storing a byte through a pointer while a counter decrements, and then performs additional operations before returning.
- `fill_three_tables_incrementally`: Reads a long from a global variable, adds 32000 and 64000 to it, sets a counter to 199, then repeatedly stores the three running values into three tables at increasing offsets while advancing all three pointers until the counter reaches zero, and finally stores the table base addresses back into the global variable.
- `fill_two_tables_from_global`: Reads a long from a global variable, adds 40000 to it, sets a counter to 199, then repeatedly stores the two running values into two tables at increasing offsets while advancing both pointers until the counter reaches zero.
- `init_globals_call_routine`: Stores a constant into a global byte, saves a stack address into another global, calls a routine several times while storing its result into a global variable, and returns.
- `save_pointer_call_routines`: Loads a global pointer into a register, copies it into another register, calls a routine, decrements the second register, stores it through a pointer, saves the pointer into a global variable, calls another routine, and returns.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `clear_memory_block`: Clears a byte at a global pointer, increments that pointer, and repeats while a counter decrements from 32764 to zero, then restores the pointer.
- `clear_memory_blocks`: Clears a byte at a global pointer, increments that pointer, and repeats while a counter decrements from 32764 to zero, then restores the pointer and clears three more byte blocks of sizes 1, 94186 and 62456.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 98 into d1, then merges 186 into d1.
- `build_move_instruction_table`: Toggles a global flag and builds a sequence of 68000 move instructions in a buffer, using a routine address that depends on the flag, then fills the buffer with immediate and move word instructions targeting a table.
- `write_video_setup_constants`: Writes a series of fixed constants into global variables and custom video registers, then returns.
- `toggle_interrupt_enable_mask`: Enables interrupts by writing to the interrupt enable register, calls another routine, then writes two masks to the interrupt registers before returning.
- `set_interrupt_enable_and_call`: Enables interrupts by writing a value with bit 15 set into the interrupt enable register, clears a stored value, copies two global values into adjacent globals, then calls another routine before returning.
- `store_color_byte`: Stores the byte value 240 into the color register at offset 180 of the chipset, then returns.
- `advance_audio_frame_counters`: Increments two counters in a global variable, clears the byte at offset 226, and decrements the counter at offset 220, resetting it to ten and inverting a flag when it goes negative.
- `handle_audio_timer`: Reads a button/status byte and returns early unless a flag bit is set, then sets audio timer registers and decrements a counter while the bit is nonzero, and finally stores a value into a lookup table indexed by that value before waiting for a button bit.
- `wait_position_byte_set`: Sets a status byte at offset 212 to one, then returns immediately if that byte is zero, otherwise it keeps looping while the vertical/horizontal position register is less than 58880.
- `setup_blitter_loop`: Computes a blitter destination address from global values, sets the blitter address, mode and size registers, then loops a fixed number of times calling the blitter wait routine between each setup before returning.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `init_blitter_loop`: Initializes the blitter control, mode and size registers, then loops a fixed number of times calling the blitter wait routine between each setup before returning.
- `configure_blitter_loop`: Reads a global value, sets the blitter mask, control, mode and size registers, then loops a fixed number of times calling the blitter wait routine between each setup before returning.
- `test_word_then_jump`: Tests a global word, and if it is nonzero it calls a routine, calls another routine, and jumps to a third location, otherwise it returns.
- `call_and_check_flag`: Calls a routine, tests a global word, and returns immediately if that word is zero, otherwise it falls through and returns.
- `check_flag_then_store_result`: Checks whether a global byte is zero, calls another routine, and then stores one into a global byte if the check was false, otherwise it stores zero.
- `compare_table_adjust_pointer`: Loads a table address, compares it with a register, and if the register is greater it decrements the pointed long, saves the pointer, increments the register, clears bit 0 of the register, adds the register to the pointed long, moves the saved pointer into the return register, and returns.
- `noop_d0_or_zero`: Performs a bitwise OR of the value in data register d0 with the immediate constant 0, which leaves the register's contents unchanged, then returns.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 242 into d7, then merges 0 into d0.
- `set_d1_and_call_routines`: Sets bits in register d1, then returns immediately if d1 is zero.
- `save_registers_call`: Copies the contents of registers a4 and a5 into registers d1 and d2, calls another routine, and returns if that routine reports zero, otherwise falling through to the next routine.
- `store_global_pointer`: Stores the address of a global variable into a global pointer at offset 120a, then returns.
- `read_joystick_and_dispatch`: Compares a 16-bit global variable against zero and one, branching to different routines for each match, then loads a hardware joystick data register into a register and calls a routine using a computed address.
- `read_input_and_update`: Reads the fire-button bits from a register and sets or clears two byte fields of a structure while clamping a counter between zero and 319, then reads the mouse movement bits and adjusts a second counter between zero and 199.
- `update_structure_fields`: Combines an input value with a constant, increments a counter clamped at 319, and updates four byte fields of a structure based on bit tests, then returns early if a global flag is zero.
- `fill_table_with_pointer`: Fills a table of seven 32-bit entries with the address of a global variable, using a loop that decrements a counter before returning.
- `return_byte_in_d0`: Returns the byte value passed in the data register without modifying it.
- `copy_and_merge_stream`: Loads two longwords from the stack into a global variable, then repeatedly tests a source byte and calls a routine while advancing through a block until the byte is zero.
- `copy_bytes_and_check`: Copies four bytes from the address in a0 to consecutive hardware register locations, loads the next source byte into a register, and returns if that byte is nonzero.
- `call_and_branch_back`: Calls a routine and then branches back to an earlier instruction, transferring control without performing any other operation.
- `merge_bits_into_structure_2`: Computes a table index from a global byte, then for each of six bytes in a source block ORs the byte into one location and ANDs its complement into three others of a structure, advancing the pointers each pass.
- `merge_bits_into_structure`: Merges an input bit pattern into five byte locations in a structure, advances the structure pointer, repeats for each byte of a block using a decrementing counter, and increments a global byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `mask_and_increment_byte`: Clears bit 7 of the byte at offset 24000 in a1, applies the same mask to the byte at offset 32000, advances a1 by 40, and repeats until d1 counts down, then increments a global byte at offset a fixed address and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `mask_both_registers_d0`: ANDs the byte at offset 24000 in a1 with d0, ANDs the byte at offset 32000 with d0, advances a1 by 40, and repeats until d1 counts down, then increments a global byte at offset a fixed address and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `advance_pointer_and_decrement_loop`: Clears the low byte of d0, advances a pointer by 40 bytes, then loops decrementing d1 until it reaches zero and increments a byte at offset an address of the data register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `combine_byte_and_decrement_loop`: Combines the low byte of d0 with a value in the data register, advances a pointer by 40 bytes, then loops decrementing d1 until it reaches zero and increments a byte at offset an address of the data register.
- `call_and_accumulate_table`: Calls a routine, loads a table address, clears a register, loads a constant, adds the pointed long into it, decrements a counter, and loops while the counter is nonzero, then stores the accumulated value into a global variable and returns.
- `compute_scaled_quotient_remainder`: Computes a scaled product from the input word stored into a global variable, extracts its high word, and returns the quotient and remainder of dividing that value by the input word.
- `call_routines_and_set_flag`: Calls another routine, then calls a second routine and clears a global variable.
- `disable_dma_and_clear_registers`: Clears a global variable, then calls another routine and, if that routine returns zero, disables all custom DMA channels, clears a run of custom registers in a loop, and calls several other routines.
- `copy_longwords_and_modify_flag`: Copies twelve longwords from a source area to a destination area, then, if a global flag is set, calls a routine and increments a byte at offset minus fifty-three of the stack.
- `adjust_d0_and_pop_d7`: Calls a routine at a fixed address, adds 2114 to the 16-bit value in d0, then pops a longword off the stack into d7.
- `increment_counters_and_dispatch`: Increments a 16-bit counter at offset 17783 of the structure in a1 and, when it overflows, increments a byte in a global variable.
- `dispatch_register_add_constant`: Returns if the register is negative, jumps if it is zero, duplicates the register contents, calls a routine, and adds a constant through a pointer indexed by a register before returning.
- `call_and_dispatch_pointer`: Calls a routine, adds a large constant to a register, increments a value indexed by a register, pops a pointer, and returns if the register was non-negative, otherwise it loads a constant, saves the register, calls a routine, and adds a constant through the pointer before returning.
- `call_routine_push_constant`: Calls a routine and pushes an immediate constant onto the stack.
- `dispatch_on_global_command_byte`: Dispatches on the value of a global variable, calling setup routines and copying data blocks for codes 1 through 6, and otherwise returns.
- `call_and_fall_through`: Calls a routine via a branch-and-jump sequence and then falls through into the following routine without returning.
- `copy_bytes_enable_dma`: Copies a block of bytes from one global variable to another using a decrementing counter, then sets up DMA channels and enables them.
- `copy_block_and_clear_registers`: Runs several setup routines, then copies a block of bytes from one global variable to another in a loop that counts down from seven, and finally clears a run of custom registers.
- `copy_fixed_block_from_table`: Runs several setup routines, then copies a fixed block of bytes from a source table into a global variable using a loop that counts down from 199.
- `copy_block_and_run_setup_repeatedly`: Copies a block of bytes counted by a global variable from one address to another, runs a setup routine, and repeats the copy-and-setup sequence three times before returning.
- `copy_byte_block`: Calls a routine, then another routine, then loops calling a third routine until it returns a nonzero flag.
- `store_and_call_routines`: Calls a routine, then another, then a third, loads the destination buffer address at an address into a0, stores the value 159 in d0, and calls two more routines before returning.
- `initialize_global_variables`: Clears sixteen consecutive global variables, then stores the value 7 in the next seven, the value 1 in the following one, the value 7 in the next, and the value 5 in the last, then calls a routine.
- `clear_globals_and_registers`: Clears a global variable, calls another routine, sets a global to a constant, and then clears a run of custom registers in a loop that counts down from seven.
- `copy_blocks_increment_byte`: Copies twelve 32-bit values from a source block at offsets -5 through -45 of a6 to a destination block at offsets -4 through -40 of a1, then increments a byte at offset 11841 of a0 by the character '1' and returns.
- `increment_global_then_dispatch`: Reads a global variable, sets it to that value plus 50, and branches back to that same location while the stored value is 50 or greater.
- `call_routines_then_loop`: Calls two routines and then enters an infinite loop that never returns.
- `call_and_jump`: Calls a routine, then another routine, and finally jumps to the address at an address without returning.
- `validate_and_initialize`: Checks whether the incoming d0 value equals a specific constant and clears a global variable in that case, otherwise sign-extends it.
- `setup_registers_and_call`: Loads four constants into registers d0 through d3, sets up two address registers from global variables, then calls two routines and branches into a third.
- `dispatch_and_configure`: Calls a routine and returns immediately if the register is nonzero.
- `dispatch_and_loop_handlers`: Calls a routine, then reads a global variable to index into a table and stores the result into a register before entering a loop that calls multiple routines and dispatches based on a global value until that value becomes zero.
- `dispatch_indexed_table`: Stores the incoming d0 and d1 values into a global variable and dispatches to a routine selected from a jump table indexed by those values.
- `dispatch_via_jumptable`: Loads a global variable, multiplies it by four, uses it as an offset into a jump table, and jumps to the address stored there, returning indirectly.
- `dispatch_from_table`: Reads a global byte and returns early unless it is zero or two, then computes a table offset from a global word, loads two structure fields, calls a routine, and performs a masked store before an indirect jump.
- `invoke_from_global_table`: Reads a 16-bit value from a global variable, multiplies it by two, adds the result to the address of that same global variable, shifts the sum left by two bits, and uses it to index a word in a table before jumping to the routine stored at that entry.
- `conditional_second_call`: Calls a routine, and if the status register flag is clear it calls a second routine before returning.
- `compare_value_ranges`: If a global variable is nonzero, it compares the high and low halves of the incoming d0 and d1 values against stored bounds and returns 255 when the first pair falls within range, otherwise it clears d0 when the second pair falls within range, and finally returns the original d0 value.
- `handle_character_state`: Compares a global byte against the character 'D' and, if equal, clears it and returns 255.
- `process_buffer_character`: Reads a global byte and, when it is a nonzero value other than the characters 'D' and 'E', clears the global and either decrements a counter while storing a space at an offset, or copies a character from a table into a buffer and increments the counter, then returns.
- `decrement_and_reset_counter`: Decrements a global counter and, when it goes negative, resets it to ten and inverts it.
- `index_and_call_routine`: Looks up a pair of 16-bit values from a table indexed by a global variable, then loads a routine entry point from a jump table and branches to it, passing the table values as arguments.
- `conditional_clear_global`: Calls a routine, and if the status flag is clear it performs four further calls, otherwise it clears a global variable.
- `increment_short_value`: Loads the 16-bit value stored at the address in a0, adds 1 to it, and stores the incremented result back to the same location, then returns.
- `setup_and_invoke_routine`: Loads the constants 20 and 200 into d0 and d1, clears d2 and d3, fetches two pointers from global variables into a0 and a2, then jumps to another routine passing those values as arguments.
- `fill_bytes_with_space`: Fills eleven consecutive bytes of a global variable with the decimal value 32 and then returns.
- `format_number_and_sort_records`: Copies 44 bytes from a source table into a destination buffer, then copies two more blocks of 4 and 5 bytes, each followed by a space byte.
- `clear_global_word_tables`: Clears 69 consecutive 16-bit words of a global variable, then clears 12 consecutive 16-bit words of two global variables in parallel, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_byte_buffer`: Sets a counter to 95 and clears 95 consecutive bytes of a global variable starting at offset 70287, then returns.
- `translate_string_table`: For each of 96 entries it writes a backspace byte to offset 12 of a structure at an address when the matching byte in a table at 70287 is zero, otherwise a byte value of 15, then copies the following null-terminated string forward by three bytes and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `index_dma_control_table`: Doubles the 16-bit value in a register, indexes a table by that doubled value, stores the resulting word into a global variable, then writes that value to the DMA control register and returns.
- `call_routine_four_times`: Calls a routine four times, passing the incoming d1 value to each call, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately`: Returns immediately without modifying any data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_constant_in_global`: Stores the decimal constant an address into a global variable and then returns.

### Disk.1 bytes 896512-901120 (code never loaded by a reference run)

- `validate_and_adjust_table_slot`: Compares a global counter against a range of values and returns early if it is outside that range, otherwise it scans a table of 14-bit entries and decrements a matching slot along with a second counter.
- `merge_tables_and_call`: Sets bits of two byte locations, stores constants into global variables, calls another routine three times, merges five spaced words of a global table into consecutive words, then runs a countdown loop that calls two other routines and returns.
- `configure_registers_and_refresh_tables`: Sets bits 0, 2 and 4 of the two control registers pointed to by a1 and a3, then repeatedly writes the OR of five consecutive 32-bit words from a table into successive 32-bit locations in the same table.
- `merge_longword_buffers`: Sets bits in two byte locations, initializes several global variables, merges five longwords from a buffer into another buffer in a loop, then runs a series of setup calls.
- `check_byte_until_limit`: Stores the value 65279 combined with a global variable into a byte at offset 0AE2, then repeatedly checks that variable, returning as soon as it becomes zero or once it reaches 58881.
