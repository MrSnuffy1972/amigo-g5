# Game 564 (Revenge of Defender): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00564). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1810064 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 29 raw-word lines |
| Functions named and described | pass, 18 of 18 named (0 author symbols), 18 of 18 described (1 by rule, 17 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 25 of 25 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 26112 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 410 of 410 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9390 |
| code | 2686 |
| data | 1696100 |
| unused | 101888 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf:bootblock | 1 | 6 | 1 |
| logged disk load from Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf boot-block load | 1 | 252 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 146 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf bytes 588288-614400 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: custom-format (non-DOS) disk, 1682 non-empty blocks: matched against track-loaded segments
- revenge of defender (1988)(epyx)(disk 2 of 2)/Revenge of Defender (1988)(Epyx)(Disk 2 of 2).adf: custom-format (non-DOS) disk, 1635 non-empty blocks: matched against track-loaded segments
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: the boot block reads 4096 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- runtime segment an address: 22 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: bytes 588288-614400 look like code no run loaded: recovered position-independently (load address unknown)
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: 2 logged loads cover 30208 bytes of the image
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: blocks 30-43 (bytes 15360-22528) outside the filesystem hold data, not code (opcode density 0.007, entropy 1.81)
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: blocks 46-95 (bytes 23552-49152) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.85)
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: blocks 100-103 (bytes 51200-53248) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.16)
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: blocks 110-858 (bytes 56320-439808) outside the filesystem hold data, not code (opcode density 0.004, entropy 7.11)
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: blocks 877-878 (bytes 449024-450048) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.04)
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: blocks 880-880 (bytes 450560-451072) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.18)
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: blocks 883-1146 (bytes 452096-587264) outside the filesystem hold data, not code (opcode density 0.003, entropy 7.02)
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: blocks 1203-1208 (bytes 615936-619008) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.28)
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: blocks 1210-1672 (bytes 619520-856576) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.50)
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: blocks 1676-1682 (bytes 858112-861696) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.21)
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: blocks 1684-1699 (bytes 862208-870400) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.01)
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: blocks 1706-1709 (bytes 873472-875520) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.52)
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: blocks 1711-1741 (bytes 876032-891904) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.37)
- revenge of defender (1988)(epyx)(disk 1 of 2)/Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf: blocks 1750-1759 (bytes 896000-901120) outside the filesystem hold data, not code (opcode density 0.003, entropy 7.12)
- revenge of defender (1988)(epyx)(disk 2 of 2)/Revenge of Defender (1988)(Epyx)(Disk 2 of 2).adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- revenge of defender (1988)(epyx)(disk 2 of 2)/Revenge of Defender (1988)(Epyx)(Disk 2 of 2).adf: blocks 2-22 (bytes 1024-11776) hold ASCII text
- revenge of defender (1988)(epyx)(disk 2 of 2)/Revenge of Defender (1988)(Epyx)(Disk 2 of 2).adf: blocks 24-282 (bytes 12288-144896) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.96)
- revenge of defender (1988)(epyx)(disk 2 of 2)/Revenge of Defender (1988)(Epyx)(Disk 2 of 2).adf: blocks 285-797 (bytes 145920-408576) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.66)
- revenge of defender (1988)(epyx)(disk 2 of 2)/Revenge of Defender (1988)(Epyx)(Disk 2 of 2).adf: blocks 854-871 (bytes 437248-446464) outside the filesystem hold data, not code (opcode density 0.029, entropy 3.46)
- revenge of defender (1988)(epyx)(disk 2 of 2)/Revenge of Defender (1988)(Epyx)(Disk 2 of 2).adf: blocks 875-1586 (bytes 448000-812544) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.79)
- revenge of defender (1988)(epyx)(disk 2 of 2)/Revenge of Defender (1988)(Epyx)(Disk 2 of 2).adf: blocks 1650-1759 (bytes 844800-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.88)
- 1 never-loaded region(s), 26112 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf:bootblock

- `init_lib_base_and_jump`: Initializes a library base by storing a version word at offset 28, a flag at offset 30, and two pointers at offsets 32 and 36, then calls the exec library routine at offset -456 and jumps to the address at offset 40.

### logged disk load from Revenge of Defender (1988)(Epyx)(Disk 1 of 2).adf boot-block load

- `setup_bitmaps_and_wait_raster`: Clears the first 31 words of the blitter/audio register block, then configures two bitmaps with their modulo and boundary values and copies several memory blocks into them.
- `xor_tables_until_target`: XORs successive 32-bit words from a source table into a block starting at a global address, resetting the counter and advancing the source pointer in chunks of 512 words until the destination reaches the target address, then returns the remaining counter value if it is not zero.
- `load_byte_pattern_registers`: Loads a byte from a table into register d0 and writes it to the data registers d2, d4, and d6, repeating the pattern for each of the three registers.
- `init_and_call_loop`: Initializes a global structure with version, flags, and pointers, then loops calling a routine at offset -456 until a condition byte at offset 31 of a structure in a1 is zero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `copy_blocks_and_call_lib`: Copies four-byte blocks from a source table to a destination table until a counter reaches zero, then initializes a structure and calls the exec library routine at offset -456, checking a flag at offset 31.
- `init_struct_and_delay_loop`: Initializes a structure with a version and flag, clears a field, calls the exec library routine at offset -456, then spins a delay loop before clearing interrupts.
- `open_trackdisk_device`: Calls the exec library routine at offset -294 to find a task, stores the result, opens a device by calling the routine at offset -354, then initializes a disk device structure and stores a pointer.
- `wait_raster_copy_table`: Waits for a vertical blank by polling the raster position, then copies 200 words from a source table to a destination table, repeating for 49 iterations.
- `configure_bitmaps_and_wait`: Configures bitmap registers by clearing control words, setting modulo values, and defining window boundaries, then waits for a raster position and copies 32 half-words to a register table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `shift_table_fields`: Waits for a raster position, then shifts 18-bit fields within a table by moving words from offset 26 to offset 6, handling a special case at index 57, and restoring the first value.
- `fill_index_write_tables_2`: Copies 24 consecutive 16-bit values from a table into 24 locations in the structure pointed to by a1, using an index computed from a byte in d4, and writes zeros instead when that byte is a space.
- `fill_index_write_tables`: Computes an index from a byte in d4 and copies 24 consecutive 16-bit values from a table into 24 locations in a structure pointed to by a1, then decrements a1 and repeats the copy for a count in d1.
- `compute_index_write_tables`: Computes an index from input registers, then loops five times writing the same value to four locations in two tables spaced 44 bytes apart, adjusting the index each iteration.
- `store_initial_globals`: Stores the 16-bit values 47576 and 55576 into two global words at offsets 2 and 10, and the value 7 into the words at offsets 6 and 14, then returns.
- `check_cia_and_call_init`: Checks a global byte and the fire-button bits in the CIA port, then returns early if the global is nonzero, both tested bits are set, or the input register is zero, otherwise it calls two other routines and stores the values 5, 0, an address, and 4000 into registers d0, d1, a0, and d2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_sequence_branch`: Calls the routine at a fixed address, then the routine at a fixed address, and if that call set the condition code it calls the routine at a fixed address again, otherwise it jumps to the routine at a fixed address.
