# Game 1775 (cannon fodder (1993)(virgin)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01775). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2706528 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 33 of 33 named (0 author symbols), 33 of 33 described (2 by rule, 27 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 393 of 393 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 546 of 546 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5331 |
| code | 1866 |
| data | 2678802 |
| padding | 49 |
| unused | 20480 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3).adf:bootblock | 1 | 20 | 1 |
| logged disk load from Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3).adf boot-block load | 1 | 116 | 3 |
| logged disk load from Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3).adf boot-block load | 1 | 22 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 388 | 27 |

## Cases handled

- cannon fodder (1993)(virgin)[cr flt](disk 1 of 3)/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3).adf: custom-format (non-DOS) disk, 1734 non-empty blocks: matched against track-loaded segments
- cannon fodder (1993)(virgin)[cr flt](disk 2 of 3)/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 2 of 3).adf: custom-format (non-DOS) disk, 1747 non-empty blocks: matched against track-loaded segments
- cannon fodder (1993)(virgin)[cr flt](disk 3 of 3)/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 3 of 3).adf: custom-format (non-DOS) disk, 1752 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 445 of the 734 PCs the canonical run logged (121382 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 98304 of its 121382 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference crashes at an address (HLE illegal instruction an address at an address: through vector 0, as before (vector 4 is 0 again)): 47 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- cannon fodder (1993)(virgin)[cr flt](disk 1 of 3)/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3).adf: the boot block reads 5632 bytes from disk offset 2048 to an address (found statically); recovered as a unit
- cannon fodder (1993)(virgin)[cr flt](disk 1 of 3)/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3).adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address (play-rs-m1500): 247 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- cannon fodder (1993)(virgin)[cr flt](disk 1 of 3)/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3).adf: 2 logged loads cover 6656 bytes of the image
- cannon fodder (1993)(virgin)[cr flt](disk 1 of 3)/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3).adf: blocks 22-23 (bytes 11264-12288) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.55)
- cannon fodder (1993)(virgin)[cr flt](disk 1 of 3)/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3).adf: blocks 1474-1759 (bytes 754688-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.70)
- cannon fodder (1993)(virgin)[cr flt](disk 2 of 3)/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 2 of 3).adf: blocks 2-30 (bytes 1024-15872) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.06)
- cannon fodder (1993)(virgin)[cr flt](disk 2 of 3)/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 2 of 3).adf: blocks 33-1618 (bytes 16896-828928) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.75)
- cannon fodder (1993)(virgin)[cr flt](disk 2 of 3)/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 2 of 3).adf: blocks 1628-1759 (bytes 833536-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.35)
- cannon fodder (1993)(virgin)[cr flt](disk 3 of 3)/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 3 of 3).adf: blocks 2-30 (bytes 1024-15872) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.04)
- cannon fodder (1993)(virgin)[cr flt](disk 3 of 3)/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 3 of 3).adf: blocks 33-1579 (bytes 16896-808960) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.67)
- cannon fodder (1993)(virgin)[cr flt](disk 3 of 3)/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 3 of 3).adf: blocks 1584-1759 (bytes 811008-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.35)
- 1 packed track region(s), 734208 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3).adf:bootblock

- `setup_boot_dma_and_interrupts`: Boot-block entry, called by the system loader, that issues an operating system I/O request and writes the DMA control and interrupt enable registers plus one custom chip register.

### logged disk load from Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3).adf boot-block load

- `copy_block_and_rotate_word`: Copies a block of longwords from a source table to a destination buffer in chip memory, then rotates a second longword right by a bit count taken from a table entry and stores the result back into the destination buffer.
- `return_half_or_input_word`: Returns one word shifted right by one bit when a second register holds the value one, and otherwise returns the input word unchanged.
- `lookup_table_word_masked`: Reads a word from a fixed table in memory, indexed by a register value doubled, and ANDs it with a second register before returning it.

### logged disk load from Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3).adf boot-block load

- `disable_interrupts_copy_table`: Disables interrupts and DMA channels, then copies sixteen longwords from a source table to a global table using a loop, and finally jumps to an address stored in a global variable.
- `stub_unresolved_external_entry`: Seven-instruction routine reached from outside the static control flow in the reference run.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_table_sequence_2`: Saves all caller registers on the stack, calls a setup routine, and checks bit 1 of a word at offset 296 of the global base register.
- `configure_and_loop_flags_2`: Clears bits in a word at a pointer, toggles a global flag, writes a series of constant words to offsets of another pointer, then loops while a flag in a global variable stays set.
- `jump_to_routine`: Unconditionally branches to another routine and never executes any instructions of its own.
- `configure_and_loop_flags`: Toggles a bit in a global structure and stores a series of words at fixed offsets from a pointer, then loops while flags in two locations remain clear, calling another routine each iteration.
- `compare_word_and_return`: Saves all caller registers on the stack, loads a pointer from offset 20 of the structure in a4, compares a word at that address with the constant 17545, and returns the incoming value from d1 if they differ.
- `copy_and_scan_tables`: Copies words from one table to another while applying masks and combining them, then scans a second table for a matching value and loops until a byte counter reaches zero.
- `restore_registers_pass_through`: Restores saved registers from the stack, loads two addresses from a global structure, clears a data register, and returns the original argument.
- `tiny_move_move`: Small routine of 2 instructions that copies (a0)+ to d2, then copies (a0)+ to d3.
- `combine_and_mask_words`: Applies a mask to two data registers, combines them with exclusive or, shifts one left, and ORs the results together before returning.
- `initialize_hardware_and_loops`: Saves custom-chip and memory-mapped register values into a global structure, disables interrupts, sets up pointers, and calls a series of setup routines.
- `wait_for_flag_and_set_flag`: Spins until a specific bit in a word at offset six from a pointer in a5 is set, then writes a constant word to another offset from that same pointer and returns the observed value.
- `wait_for_flag_toggle`: Spins on a flag byte at offset four from a pointer in a5 until it is set, then spins until that flag is cleared again, and then returns.
- `increment_and_set_values`: Adds a constant to a word at an offset from the base register, writes a fixed word to two following offsets, and loops until that word reaches a specific value.
- `decrement_and_set_values`: Subtracts a constant from a word at an offset from the base register, writes a fixed word to two following offsets, and loops until that word reaches zero.
- `increment_until_limit`: Calls another routine several times in an inner loop, then increments a word at an offset from the base register and repeats until it reaches a specific value.
- `decrement_until_limit`: Calls another routine several times in an inner loop, then decrements a word at an offset from the base register and repeats until it reaches a specific value.
- `wait_for_register_match`: Loads a word at offset six from a pointer in a5 twice, masks both copies, and loops while the masked values are equal, effectively creating an infinite loop before returning.
- `check_serial_input_and_wait`: Tests a flag in a structure via a pointer and, if set, toggles a global variable while repeatedly calling another routine until a short condition holds, then restores the global variable.
- `handle_library_operations`: Calls a routine via a pointer from a global variable, and if the result is non-zero, calls two more routines via offsets from that result and from a global variable.
- `call_supervisor`: Wrapper that calls the exec library function Supervisor and returns its result to the caller.
- `initialize_system_state`: Clears a word in a structure, compares a value at offset 530 of the global base register with the constant 12850, and stores a result at offset 19434.
- `execute_indirect_table_sequence`: Reads a global variable, calls three routines via offsets from that value and from a second global, then stores a different global variable into the first global and writes to the beam control register.
- `process_data_tables`: Walks through a sequence of null-terminated strings counting them, computes an index from that count, and stores a computed address at offset 19436 of the structure in a6.
- `copy_struct_fields_2`: Computes an index from a register, scales it and adds it to a table base, then calls another routine to copy fields and stores the resulting pointer into a location at an offset from the base register.
- `copy_struct_fields`: Copies one byte from the source pointer to the destination pointer, then copies seven more bytes from successive source offsets to fixed destination offsets before incrementing the destination pointer.
- `store_pointer_parts`: Takes a long pointer at offset 41c from the base register and splits it into its low and high words, storing them at offsets 4c58 and 4c54, then does the same for the pointer at offset 4c7a, storing its words at offsets 4c60 and 4c5c.
- `clear_zero_table`: Clears a block of memory by writing zero to a sequence of longwords, advancing a pointer and looping 4599 times while decrementing a counter.
