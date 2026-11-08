# Game 1263 (RevengeOfDefender v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01263). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4043396 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 30 raw-word lines |
| Functions named and described | pass, 33 of 33 named (0 author symbols), 33 of 33 described (7 by rule, 26 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 170 of 173 (98.27%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 453 of 453 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 368476 |
| code | 2186 |
| data | 3580572 |
| padding | 2 |
| unused | 92160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 154 | 14 |
| Disk.1:bootblock | 1 | 6 | 1 |
| logged disk load from Disk.1 boot-block load | 1 | 252 | 10 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 40 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.1: custom-format (non-DOS) disk, 1682 non-empty blocks: matched against track-loaded segments
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.2: custom-format (non-DOS) disk, 1635 non-empty blocks: matched against track-loaded segments
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0.hdf: hard-disk image (AmigaDOS OFS, 4352 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree RevengeOfDefender_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.1: the boot block reads 4096 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- 9 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.1: 9 logged loads cover 432640 bytes of the image
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.1: blocks 30-43 (bytes 15360-22528) outside the filesystem hold data, not code (opcode density 0.007, entropy 1.81)
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.1: blocks 46-95 (bytes 23552-49152) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.85)
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.1: blocks 100-103 (bytes 51200-53248) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.16)
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.1: blocks 877-878 (bytes 449024-450048) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.04)
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.1: blocks 880-880 (bytes 450560-451072) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.18)
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.1: blocks 883-1199 (bytes 452096-614400) outside the filesystem hold data, not code (opcode density 0.014, entropy 6.91)
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.1: blocks 1203-1208 (bytes 615936-619008) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.28)
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.1: blocks 1210-1671 (bytes 619520-856064) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.50)
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.2: blocks 2-22 (bytes 1024-11776) hold ASCII text
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.2: blocks 24-797 (bytes 12288-408576) outside the filesystem hold data, not code (opcode density 0.009, entropy 5.85)
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.2: blocks 854-871 (bytes 437248-446464) outside the filesystem hold data, not code (opcode density 0.029, entropy 3.46)
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.2: blocks 875-1586 (bytes 448000-812544) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.79)
- RevengeOfDefender_v1.0/RevengeOfDefender_v1.0_hdf/RevengeOfDefender/Disk.2: blocks 1650-1759 (bytes 844800-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.88)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_system_and_boot`: Resets the custom chip registers and clears the interrupt enable and request flags, then installs the game's interrupt vectors and handler addresses into a global jumptable before jumping to the boot routine.
- `initialize_globals_and_boot`: Writes several constants and handler addresses into global variables and a jumptable, initializes three global counters to eight, then jumps to the boot entry point.
- `increment_global_counter`: Increments the 32-bit value stored at the global variable at offset 300 by one and returns without modifying any register.
- `call_handler_then_reroute`: Copies the second argument into a register, loads a handler address from a global variable, calls it, then calls another routine before restoring all saved registers and returning.
- `store_self_and_enable_interrupts`: Stores the address of the routine itself into a global variable, disables interrupts via the CIA interrupt register, programs the interrupt mask register, then returns.
- `dispatch_via_jumptable`: Loads the address of a jump table from a global variable into a0 and performs an indirect jump through the entry at offset four, dispatching to one of many subroutines.
- `read_and_enable_interrupts`: Calls a routine that reads a joystick button state and dispatches it to a handler, then sets the interrupt enable register to a value of 8 and returns.
- `read_joystick_dispatch`: Reads a joystick button state from the CIA port, inverts it, and dispatches through a jumptable handler when the value matches a stored or known button code, otherwise waits a delay loop and clears the button flag.
- `fill_sequence_and_conditional_call`: Calls the handler at offset thirty-six of the object in a2, then fills eight 32-bit slots in a global array with the descending sequence an address, an address, an address, 80000, 40000, 0, -40000, -80000.
- `call_handler_with_constants_2`: Saves registers, loads a handler object and a constant argument, calls the handler at offset forty, then compares a value against a threshold and calls a second handler at offset eight when they match.
- `call_handler_with_constants`: Loads the value 23040 into d0 and 10240 into d1, then calls the handler at offset forty of the object in a2 before restoring the caller's registers and returning.
- `conditional_call_and_set_flag_2`: Saves registers, compares a value against a threshold and, only when they are equal, loads a constant into a register and branches to a handler routine.
- `conditional_call_and_set_flag`: Compares d1 with an address and, only when they are equal, calls the handler at offset twelve of the object in a2.

### Disk.1:bootblock

- `init_boot_and_jump`: Writes the value 2 to the word at offset 28 of the structure in a1, then stores 4096, an address, and 11264 into the words and longs at offsets 36, 40, and 44.

### logged disk load from Disk.1 boot-block load

- `setup_audio_and_bitmaps`: Initializes the audio and blitter hardware by clearing the audio channel registers and disabling DMA, then opens the trackdisk device and waits for the first vertical blank.
- `xor_decompress_blocks`: XORs four-byte blocks from a source table into a destination buffer, resetting the counter and source pointer after each 512-byte pass, then traps with a breakpoint.
- `build_byte_sequence`: Builds a sequence of byte values in d0 by OR-ing in 5, 164, 244, and 91, writing the intermediate results into d4, d2, and d4 respectively, with no branch or call present so the routine simply executes its eleven instructions and falls through.
- `continue_byte_sequence`: Continues building the same byte sequence by OR-ing in 153 into d0 and storing the results into d6, with no branch or call present so the routine simply executes its twenty instructions and falls through.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `copy_blocks_and_call_lib`: Copies four-byte blocks from a source table to a destination table until a counter reaches zero, then initializes a structure and calls the exec library routine at offset -456, checking a flag at offset 31.
- `init_struct_and_delay_loop_2`: Initializes a structure with a version and flag, clears a field, calls the exec library routine at offset -456, then spins a delay loop before clearing interrupts.
- `open_trackdisk_device`: Calls the exec library routine at offset -294 to find a task, stores the result, opens a device by calling the routine at offset -354, then initializes a disk device structure and stores a pointer.
- `wait_raster_copy_table`: Waits for a vertical blank by polling the raster position, then copies 200 words from a source table to a destination table, repeating for 49 iterations.
- `configure_bitmaps_and_wait`: Configures bitmap registers by clearing control words, setting modulo values, and defining window boundaries, then waits for a raster position and copies 32 half-words to a register table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `copy_four_dwords`: Copies four 32-bit words from the source table pointed to by a0 to the destination table pointed to by a1, then returns.
- `init_struct_and_doio_loop`: Initializes a structure at address an address with a version and flag, stores three values into its fields, then repeatedly clears interrupts, calls the exec library routine at offset -456, and checks a status byte until it becomes nonzero before returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `init_struct_and_delay_loop`: Initializes a structure with the version nine and a flag, clears a field, calls the exec library routine at offset -456, then spins a delay loop of 28672 iterations before re-enabling interrupts and returning.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
