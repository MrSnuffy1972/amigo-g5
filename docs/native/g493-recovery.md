# Game 493 (Fighter Bomber): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00493). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1803040 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 10 of 10 named (0 author symbols), 10 of 10 described (2 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 59 of 59 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 4608 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 248 of 248 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9532 |
| code | 830 |
| data | 1714850 |
| padding | 4 |
| unused | 77824 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf:bootblock | 1 | 16 | 1 |
| logged disk load from Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf boot-block load | 1 | 107 | 2 |
| logged disk load from Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 105 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf bytes 611328-615936 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1604 non-empty blocks: matched against track-loaded segments
- fighter bomber (1991)(activision)[cr vf](disk 2 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: the boot block reads 5632 bytes from disk offset 0 to an address (found statically); recovered as a unit
- runtime segment an address: 50 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: bytes 611328-615936 look like code no run loaded: recovered position-independently (load address unknown)
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: 3 logged loads cover 15872 bytes of the image
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: blocks 22-106 (bytes 11264-54784) outside the filesystem hold data, not code (opcode density 0.008, entropy 4.96)
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: blocks 580-580 (bytes 296960-297472) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: blocks 588-588 (bytes 301056-301568) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: blocks 595-595 (bytes 304640-305152) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: blocks 603-605 (bytes 308736-310272) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.00)
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: blocks 616-770 (bytes 315392-394752) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.92)
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: blocks 779-780 (bytes 398848-399872) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.36)
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: blocks 784-784 (bytes 401408-401920) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.02)
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: blocks 846-846 (bytes 433152-433664) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.05)
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: blocks 852-852 (bytes 436224-436736) outside the filesystem hold data, not code (opcode density 0.055, entropy 2.82)
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: blocks 855-1164 (bytes 437760-596480) outside the filesystem hold data, not code (opcode density 0.059, entropy 5.80)
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: blocks 1167-1192 (bytes 597504-610816) outside the filesystem hold data, not code (opcode density 0.044, entropy 3.86)
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: blocks 1203-1328 (bytes 615936-680448) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.17)
- fighter bomber (1991)(activision)[cr vf](disk 1 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf: blocks 1332-1364 (bytes 681984-698880) outside the filesystem hold data, not code (opcode density 0.009, entropy 5.15)
- fighter bomber (1991)(activision)[cr vf](disk 2 of 2)/Fighter Bomber (1991)(Activision)[cr VF](Disk 2 of 2).adf: blocks 2-1759 (bytes 1024-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.11)
- 1 never-loaded region(s), 4608 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 425472 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf:bootblock

- `perform_io_operations`: Sets up an I/O request structure in a1 with specific lengths and offsets, calls a routine via a pointer from a global variable, calls another routine at a fixed address, and then performs a second I/O operation with different parameters before jumping to a fixed address.

### logged disk load from Fighter Bomber (1991)(Activision)[cr VF](Disk 1 of 2).adf boot-block load

- `store_byte_from_pointer`: Loads a longword from a pointer in a0, extracts a byte, stores it in a global variable via a pointer in a5, and rotates the longword in d0 before returning.
- `process_data_stream`: Uses a loop to call another routine and performs data manipulation by reading bytes from a source, comparing them against a constant, and writing modified bytes to a destination table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_counters`: Modifies a global variable using arithmetic and checks a secondary counter to reset values or update a third global variable.
- `process_joystick_input`: Reads the status of the joystick port and CIAA registers to perform bitwise logic and updates a global variable based on the input state.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_clamped_value`: Performs arithmetic on several global variables, applying minimum and maximum constraints to a value before performing bitwise checks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_global_variable`: Decrements a global variable and sets it to a specific value if a condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_video_register`: Sets bits 0 and 1 of a global variable in a loop, then checks a hardware register and writes a value to the video port register if the register is zero.
