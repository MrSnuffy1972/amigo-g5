# Game 1335 (flood (1990)(electronic arts)(m4)[cr scx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01335). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (923792 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 32 of 32 named (0 author symbols), 32 of 32 described (4 by rule, 24 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 650 of 650 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 20480 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1394 of 1423 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 13590 |
| code | 9754 |
| data | 811856 |
| padding | 16 |
| unused | 88576 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Flood (1990)(Electronic Arts)(M4)[cr SCX].adf:bootblock | 1 | 141 | 5 |
| logged disk load from Flood (1990)(Electronic Arts)(M4)[cr SCX].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 832 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 142 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 277 | 9 |
| Flood (1990)(Electronic Arts)(M4)[cr SCX].adf bytes 11264-13824 (code never loaded by a reference run) | 1 | 31 | 1 |
| Flood (1990)(Electronic Arts)(M4)[cr SCX].adf bytes 24576-28672 (code never loaded by a reference run) | 1 | 0 | 0 |
| Flood (1990)(Electronic Arts)(M4)[cr SCX].adf bytes 58368-72192 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: custom-format (non-DOS) disk, 1585 non-empty blocks: matched against track-loaded segments
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 302 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 119 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 229 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: bytes 11264-13824 look like code no run loaded: recovered position-independently (load address unknown)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: bytes 24576-28672 look like code no run loaded: recovered position-independently (load address unknown)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: bytes 58368-72192 look like code no run loaded: recovered position-independently (load address unknown)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: 4 logged loads cover 21504 bytes of the image
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: 38 track-loaded blocks found in recovered runtime segments and classified through them
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.75)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 38-46 (bytes 19456-24064) outside the filesystem hold data, not code (opcode density 0.010, entropy 4.22)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 60-62 (bytes 30720-32256) outside the filesystem hold data, not code (opcode density 0.048, entropy 5.43)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 67-94 (bytes 34304-48640) outside the filesystem hold data, not code (opcode density 0.056, entropy 5.66)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 143-154 (bytes 73216-79360) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.59)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 165-400 (bytes 84480-205312) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.16)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 407-602 (bytes 208384-308736) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.59)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 605-616 (bytes 309760-315904) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.25)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 627-814 (bytes 321024-417280) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.06)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 825-862 (bytes 422400-441856) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.54)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 869-899 (bytes 444928-460800) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.62)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 902-938 (bytes 461824-480768) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.40)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 946-948 (bytes 484352-485888) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.53)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 957-971 (bytes 489984-497664) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.07)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 979-1008 (bytes 501248-516608) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.51)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1012-1028 (bytes 518144-526848) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.41)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1034-1069 (bytes 529408-547840) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.40)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1078-1094 (bytes 551936-560640) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.24)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1100-1113 (bytes 563200-570368) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.37)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1122-1141 (bytes 574464-584704) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.32)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1144-1159 (bytes 585728-593920) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.39)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1166-1173 (bytes 596992-601088) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.92)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1177-1244 (bytes 602624-637440) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.27)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1254-1263 (bytes 642048-647168) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.73)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1265-1293 (bytes 647680-662528) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.47)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1298-1302 (bytes 664576-667136) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.41)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1507-1525 (bytes 771584-781312) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.30)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1529-1546 (bytes 782848-792064) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.95)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1551-1567 (bytes 794112-802816) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.71)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1569-1587 (bytes 803328-813056) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.65)
- flood (1990)(electronic arts)(m4)[cr scx]/Flood (1990)(Electronic Arts)(M4)[cr SCX].adf: blocks 1589-1750 (bytes 813568-896512) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.32)
- 3 never-loaded region(s), 20480 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 97792 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Flood (1990)(Electronic Arts)(M4)[cr SCX].adf:bootblock

- `clear_and_fill_memory_blocks`: Boot block entry of 50 instructions that clears a block of memory, then fills a second block with a repeated character and word pattern.
- `tiny_sub`: Small routine of 1 instruction that subtracts d2 from d1.
- `write_four_word_entry`: Stores four 16-bit words through an address register, two of them fixed values and two taken from input registers, and returns an input word offset by a fixed amount.
- `call_exec_io_and_set_colors`: Calls exec DoIO and four other library routines at fixed offsets, reads the disk status and fire button port, and writes two colour register words.
- `call_library_pair_twice`: Calls two library functions at fixed offsets from a base, alternating between them twice.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_and_call_table`: Iterates through a table pointed to by a structure in a6, checking bytes at an offset, incrementing two counters in the structure, and calling another routine with parameters pushed to the stack.
- `check_bit_and_increment`: Checks a bit in a byte at an offset from a pointer in a0 using a value from the stack, then increments a value on the stack if the bit is not set.
- `return_stack_word`: Returns the 16-bit word stored at an offset of minus two from the address in a5.
- `update_state_and_bytes`: Calls several routines, checks a global variable, and if not equal to 43, calls more routines and writes four bytes from a table to a global array.
- `execute_nested_delay_loops`: Calls a routine twice using pointers stored in global variables, then executes a nested loop structure where an inner loop counts to 5000 and an outer loop counts to 16, before copying a global pointer to a destination and calling another routine.
- `initialize_global_values`: Calls another routine with two parameters, stores the result in a global variable, performs arithmetic to update two other global variables, clears three additional global longwords, and sets a global word to one.
- `clear_memory_blocks`: Clears a block of memory starting at a global address with 128 entries of 4 bytes each, then clears a second block with 8 entries of 6 bytes each, and finally calls another routine.
- `execute_nested_calls`: Calls a routine, pushes a word from a global pointer onto the stack, and then calls another routine.
- `call_subroutine_with_parameter`: Pushes the word value of six onto the stack and calls another routine.
- `calculate_and_update_sprite_data`: Calculates a memory offset using bitwise operations and arithmetic on values from a structure, then updates a global pointer.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d7.
- `copy_d0_to_d7`: Copies the contents of d0 to d7 and returns.
- `check_interrupt_request_flag`: Checks an interrupt-request flag bit, increments a counter when it is set, and resets the flag register to the fixed value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_adda_rts`: Small routine of 3 instructions that merges 170 into d0, then adds 6 to a7, then returns.
- `advance_random_seed`: Updates a 16-bit state word with a multiply-and-add step, producing the next pseudo-random value.
- `blit_clipped_sprite`: Checks if a rectangular area is within visible bounds, calculates the necessary clipping dimensions and offsets, and programs the blitter registers to draw the resulting rectangle.
- `update_interrupt_mask`: Increments a value from a stack parameter by 4, stores it in a hardware register, and then checks a bit in a hardware register to decide whether to return or continue.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_counter`: Adds a value from a global variable to a value at a global address, calls another routine, and then masks the global address value with three.
- `process_data_table`: Loops through a table to call another routine with values from the table, then calculates an offset based on a global variable and calls another routine with that offset.
- `read_joystick_direction_state`: Reads joystick and button registers, maps the direction bits through a table, and updates several global variables including a counter that is clamped to 255.
- `copy_video_port`: Copies a 16-bit value from a hardware register to a global variable.
- `clear_memory_block`: Fills a block of memory starting at a pointer from a global variable with zeros using a loop.
- `map_character_byte`: Compares a byte from a global variable against a long list of constants and stores a corresponding 16-bit value in d0 before returning.
- `clear_memory_buffer_loop`: Clears two 16-bit values in a global structure and then executes a loop that clears 120 blocks of 4-byte words in a memory buffer pointed to by a global pointer.
- `call_subroutine_conditionally`: Calculates offsets for a table and calls another routine multiple times based on the status of bits in a value provided from the stack.
- `copy_eight_bytes`: Copies eight bytes from a source pointed to by a0 to specific offsets in a destination pointed to by a1.

### Flood (1990)(Electronic Arts)(M4)[cr SCX].adf bytes 11264-13824 (code never loaded by a reference run)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
