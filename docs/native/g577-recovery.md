# Game 577 (Dragon's Lair 2 - Time Warp): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00577). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (6308256 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 48 of 48 named (0 author symbols), 48 of 48 described (0 by rule, 46 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 18 of 18 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 518 of 518 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 14305 |
| code | 1614 |
| data | 6111089 |
| unused | 181248 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7).adf:bootblock | 1 | 19 | 2 |
| logged disk load from Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7).adf boot-block load | 1 | 320 | 34 |
| logged disk load from Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7).adf boot-block load | 1 | 94 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 85 | 9 |

## Cases handled

- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7).adf: custom-format (non-DOS) disk, 1750 non-empty blocks: matched against track-loaded segments
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 2 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 2 of 7).adf: custom-format (non-DOS) disk, 1721 non-empty blocks: matched against track-loaded segments
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 3 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 3 of 7).adf: custom-format (non-DOS) disk, 1718 non-empty blocks: matched against track-loaded segments
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 4 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 4 of 7).adf: custom-format (non-DOS) disk, 1721 non-empty blocks: matched against track-loaded segments
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 5 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 5 of 7).adf: custom-format (non-DOS) disk, 1721 non-empty blocks: matched against track-loaded segments
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 6 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 6 of 7).adf: custom-format (non-DOS) disk, 1732 non-empty blocks: matched against track-loaded segments
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: custom-format (non-DOS) disk, 1584 non-empty blocks: matched against track-loaded segments
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7).adf: the boot block reads 12288 bytes from disk offset 4608 to an address (found statically); recovered as a unit
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7).adf: the boot block reads 3584 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7).adf: 2 logged loads cover 15872 bytes of the image
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7).adf: blocks 33-214 (bytes 16896-110080) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.80)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7).adf: blocks 216-304 (bytes 110592-156160) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.92)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7).adf: blocks 306-880 (bytes 156672-451072) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.47)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7).adf: blocks 882-1759 (bytes 451584-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.60)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 2 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 2 of 7).adf: blocks 33-326 (bytes 16896-167424) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.43)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 2 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 2 of 7).adf: blocks 329-457 (bytes 168448-234496) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.75)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 2 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 2 of 7).adf: blocks 461-464 (bytes 236032-238080) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.80)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 2 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 2 of 7).adf: blocks 466-1759 (bytes 238592-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.51)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 3 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 3 of 7).adf: blocks 33-220 (bytes 16896-113152) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.49)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 3 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 3 of 7).adf: blocks 222-314 (bytes 113664-161280) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.60)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 3 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 3 of 7).adf: blocks 319-578 (bytes 163328-296448) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.60)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 3 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 3 of 7).adf: blocks 580-1547 (bytes 296960-792576) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.43)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 3 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 3 of 7).adf: blocks 1549-1659 (bytes 793088-849920) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.67)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 3 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 3 of 7).adf: blocks 1661-1677 (bytes 850432-859136) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.15)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 3 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 3 of 7).adf: blocks 1679-1759 (bytes 859648-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.56)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 4 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 4 of 7).adf: blocks 33-39 (bytes 16896-20480) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.57)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 4 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 4 of 7).adf: blocks 41-848 (bytes 20992-434688) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.49)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 4 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 4 of 7).adf: blocks 851-1200 (bytes 435712-614912) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.58)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 4 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 4 of 7).adf: blocks 1202-1236 (bytes 615424-633344) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.08)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 4 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 4 of 7).adf: blocks 1239-1759 (bytes 634368-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.38)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 5 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 5 of 7).adf: blocks 33-807 (bytes 16896-413696) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.35)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 5 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 5 of 7).adf: blocks 811-1122 (bytes 415232-574976) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.56)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 5 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 5 of 7).adf: blocks 1124-1127 (bytes 575488-577536) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.56)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 5 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 5 of 7).adf: blocks 1129-1469 (bytes 578048-752640) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.63)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 5 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 5 of 7).adf: blocks 1471-1759 (bytes 753152-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.72)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 6 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 6 of 7).adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.09)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 6 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 6 of 7).adf: blocks 33-323 (bytes 16896-165888) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.44)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 6 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 6 of 7).adf: blocks 326-364 (bytes 166912-186880) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.03)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 6 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 6 of 7).adf: blocks 367-1759 (bytes 187904-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.72)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 2-74 (bytes 1024-38400) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.06)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 77-81 (bytes 39424-41984) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.85)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 83-225 (bytes 42496-115712) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.78)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 231-259 (bytes 118272-133120) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.15)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 262-266 (bytes 134144-136704) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.12)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 275-275 (bytes 140800-141312) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.02)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 277-353 (bytes 141824-181248) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.51)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 356-371 (bytes 182272-190464) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.97)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 374-446 (bytes 191488-228864) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.06)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 448-506 (bytes 229376-259584) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.02)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 517-792 (bytes 264704-406016) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.90)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 803-878 (bytes 411136-450048) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.02)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 880-1078 (bytes 450560-552448) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.94)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 1089-1364 (bytes 557568-698880) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.64)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 7 of 7)/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 7 of 7).adf: blocks 1375-1650 (bytes 704000-845312) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.09)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7).adf:bootblock

- `initialize_boot_io`: Calls a routine at address 3c, sets four fields in a structure pointed to by a1, calls a routine at an offset from a6, and jumps to address 40008.
- `perform_io_operation`: Sets four fields in a structure pointed to by a1, calls a routine at an offset from a6, calls a routine at address 40000, and returns.

### logged disk load from Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7).adf boot-block load

- `copy_byte_and_jump`: Copies a single byte from a source address to a destination address at a fixed location, then jumps to the address stored at that destination.
- `load_and_dispatch_blocks`: Copies a block of data from a0 to a1, stores several calculated addresses into a table at a global address, copies a byte from a0 to a1, and then calls three different routines sequentially before performing an indirect jump using an address from a table.
- `copy_memory_block`: Copies bytes from a source pointed to by a0 to a destination at 7f000, looping until the destination pointer matches the source pointer.
- `initialize_jump_table`: Calls another routine, fills a table at a global address with the addresses of several other routines using a loop, and then populates a jump table at a global address using three specific routine addresses before performing an indirect jump based on a byte in a0.
- `execute_jump_table_entry`: Loads a byte from a0, shifts it left by two, and jumps to an address found in a table at an offset from a6.
- `copy_byte_and_branch`: Copies a byte from the address pointed to by a0 minus 1 to the address pointed to by a2, increments a2, and then branches.
- `check_byte_and_copy`: Clears d0, loads a byte from the address in a0 into d0, and if the byte is not zero and not one, copies a byte from the address in a4 to the address in a2 and increments a2 before branching.
- `load_byte_and_branch`: Loads a byte from the address in a0 into d1, increments a0, and then branches.
- `call_load_byte_and_branch`: Calls another routine, loads a byte from the address in a0 into d1, increments a0, and then branches.
- `check_byte_and_copy_offset`: Clears d0, loads a byte from the address in a0 into d0, and if the byte is not zero and not one, copies a byte from the address at offset 1 of a4 to the address in a2 and increments a2 before branching.
- `branch_with_zero_d1`: Sets d1 to zero and then branches.
- `write_zero_and_loop`: Calls another routine, writes a zero byte to the address in a2 and increments a2, decrements d0, and loops if d0 is not zero, otherwise it branches.
- `triple_copy_byte`: Loads a byte from a0 into d0 and increments a0; if d0 is not zero, it writes that byte three times to the address in a2 and increments a2 each time, then branches.
- `copy_offset_byte`: Copies a byte from an offset of 2 from the address in a4 to the address in a2 and increments a2, then branches.
- `read_two_bytes`: Reads two consecutive bytes from a pointer in a0 and returns them as a 16-bit value.
- `read_scaled_byte`: Reads a byte from a pointer in a4, shifts it left by two, and returns the result.
- `copy_three_bytes`: Copies three consecutive bytes from a pointer in a0 to a pointer in a4.
- `populate_extended_jump_table`: Calls another routine twice, fills a table at a global address with the addresses of six different routines using a loop, and then populates a jump table at a global address using those six routine addresses before performing an indirect jump based on a byte in a0.
- `dispatch_via_jump_table`: Reads a byte from the address in a0, shifts it left by two bits, and uses the result as an offset into a table pointed to by a6 to call another routine.
- `copy_byte_and_branch_alt`: Copies a byte from the address in a0 minus 1 to the address in a2 and increments a2, then branches.
- `copy_byte_on_non_zero`: Loads a byte from the address in a0 into d0, and if the byte is not zero, copies a byte from the address in a4 to the address in a2 and increments a2 before branching.
- `call_subroutine_and_calculate_address`: Copies a byte from d0 to d1, clears d0, calls another routine, and then calculates a destination address by subtracting d0 from a pointer in a2 before branching.
- `copy_byte_with_offset`: Loads a byte from the address in a0 into d0, and if the byte is not zero, copies a byte from the address at offset 1 of a4 to the address in a2 and increments a2 before branching.
- `load_byte_and_calculate_address`: Copies a byte from d0 to d1, clears d0, loads a byte from the address in a0 into d0 while incrementing a0, and calculates a destination address by subtracting d0 from a pointer in a2 before branching.
- `copy_byte_with_two_offset`: Clears d0, loads a byte from the address in a0 into d0, and if the byte is not zero, copies a byte from the address at offset 2 of a4 to the address in a2 and increments a2 before branching.
- `calculate_address_with_constant`: Calculates a destination address by subtracting d0 from a pointer in a2, sets d1 to 3, and then branches.
- `copy_byte_with_three_offset`: Clears d0, loads a byte from the address in a0 into d0, and if the byte is not zero, copies a byte from the address at offset 3 of a4 to the address in a2 and increments a2 before branching.
- `shift_and_load_byte_to_address`: Shifts d0 left by 8 bits, loads a byte from the address in a0 into d0 while incrementing a0, calculates a destination address by subtracting d0 from a pointer in a2, sets d1 to 4, and then branches.
- `copy_byte_with_four_offset`: Clears d0, loads a byte from the address in a0 into d0, and if the byte is not zero, copies a byte from the address at offset 4 of a4 to the address in a2 and increments a2 before branching.
- `mask_and_calculate_address_with_offset`: Masks d0 with 15 and adds 1, calculates a destination address by subtracting d0 from a pointer in a2, loads a byte from the address in a0 minus 1 into d1, shifts d1 right by 4, adds 3 to d1, and then branches.
- `branch_on_byte_or_copy_data`: Clears d0, loads a byte from the address in a0 into d0 while incrementing a0, and branches if the byte is not zero; otherwise it copies a byte from an offset of 5 in a4 to the address in a2 and increments a2 before branching.
- `copy_data_with_mask`: Performs a bitwise AND on d0 with 15, shifts d0 left by 8, loads a byte from a0 into d0, calculates a destination address by subtracting d0 from a3, and then enters a loop that copies bytes from the calculated address to a2 based on a count in d1.
- `decompress_bitstream_data`: Reads a series of length-prefixed data blocks from a source buffer into a destination buffer by performing bit-level extraction and table-based lookups.
- `process_bitstream`: Calculates an index and bit-shift values using several tables and registers, then enters a loop that copies bytes from a table into the destination pointed to by a1 until a1 reaches the address in a2.

### logged disk load from Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7).adf boot-block load

- `write_decrementing_global_until_wrap`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_global_counter`: Adds a value from a register to a global variable, and when that variable reaches 512 it resets it to zero and increments the following variable, otherwise it returns.
- `compare_and_call_routine`: Computes a difference between two global variables, and if the result is not less than a threshold it calls a routine at a fixed address, otherwise it returns.
- `loop_and_compare_status`: Calls a routine via a pointer, updates two global variables using a value from d1, and if the difference between them is zero, calls another routine; otherwise, it enters a loop that increments d0 and calls another routine until d0 equals 44, then calls another routine and returns.
- `reset_status_values`: Clears a 32-bit value at a global variable, sets a 16-bit value at another global variable to 2, and loads the address of a local constant into a0 before returning.
- `process_address_offset`: Calculates a value from d2, compares it to 1600, stores a pointer in a global variable, and if a comparison with 84320 matches, subtracts 86464 from a pointer, stores the result in a global variable, clears a 32-bit and a 16-bit global variable, and calls another routine.
- `increment_and_check_status_counters`: Increments a 16-bit value and a 32-bit value in global memory, then checks a status byte to conditionally call another routine and update a function pointer.
- `update_and_test_status`: If a byte in a global variable is non-zero, it copies two 32-bit values from one set of global variables to another; it then sets status flags based on two other global bytes and jumps to a fixed address if the second byte is non-zero.
- `update_pointers_on_condition`: Tests a byte in a global variable and, if it is zero, copies two 32-bit values from one set of global variables to another before jumping to a fixed address.
- `test_status_and_jump`: Sets the status flags based on a byte in a global variable, tests another byte in a global variable, and if that byte is zero, jumps to a fixed address.
