# Game 1742 (xybots (1989)(domark)[cr band][t +2 band][b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01742). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905200 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 17 raw-word lines |
| Functions named and described | pass, 32 of 32 named (0 author symbols), 32 of 32 described (0 by rule, 27 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 360 of 360 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 664 of 664 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5138 |
| code | 2364 |
| data | 837102 |
| padding | 180 |
| unused | 60416 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf:bootblock | 1 | 22 | 3 |
| logged disk load from Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf boot-block load | 1 | 97 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 352 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 181 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |

## Cases handled

- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: custom-format (non-DOS) disk, 1640 non-empty blocks: matched against track-loaded segments
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: the boot block reads 5120 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 280 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 75 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: 1 logged loads cover 5120 bytes of the image
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.97)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 22-26 (bytes 11264-13824) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.64)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 28-133 (bytes 14336-68608) outside the filesystem hold data, not code (opcode density 0.032, entropy 6.64)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 141-141 (bytes 72192-72704) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.30)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 146-220 (bytes 74752-113152) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.10)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 222-222 (bytes 113664-114176) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.85)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 224-304 (bytes 114688-156160) outside the filesystem hold data, not code (opcode density 0.042, entropy 5.55)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 306-343 (bytes 156672-176128) outside the filesystem hold data, not code (opcode density 0.015, entropy 3.27)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 348-348 (bytes 178176-178688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 355-415 (bytes 181760-212992) outside the filesystem hold data, not code (opcode density 0.005, entropy 3.68)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 419-610 (bytes 214528-312832) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.01)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 615-676 (bytes 314880-346624) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.77)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 683-742 (bytes 349696-380416) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.00)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 747-781 (bytes 382464-400384) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.18)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 813-1693 (bytes 416256-867328) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.58)
- xybots (1989)(domark)[cr band][t +2 band][b]/Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf: blocks 1716-1737 (bytes 878592-889856) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.35)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_hardware_status` (runtime segment an address): Enters a loop that manipulates a byte at a memory location using bitwise operations and checks a global flag to determine whether to exit or continue. Kind: disk hardware/sync check. Pass path: beq.w an address.

## Functions


### Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf:bootblock

- `forward_boot_entry_to_routine`: Boot block entry consisting of a single instruction that calls one unnamed routine.
- `invoke_unnamed_routine_once`: Seven-instruction routine whose address is taken by a PC-relative load.
- `issue_io_then_clear_state`: Issues an Exec DoIO request, calls one unnamed routine, then clears three global variables.

### logged disk load from Xybots (1989)(Domark)[cr Band][t +2 Band][b].adf boot-block load

- `shift_long_and_store_word`: Reads the longword four bytes before the address register, writes a word built from its low byte and the high byte of a data register through a pointer, and returns the value shifted right one bit with the top bit set.
- `shift_bits_through_refill_loop`: Shifts a bit register right one place per step for a count taken from its argument, calling a refill routine whenever the register empties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop`: Calls another routine and then enters an infinite loop.
- `update_display_buffer`: This routine performs a bulk memory copy or a sparse pixel update to a destination buffer based on a source table.
- `process_frame_data`: Extracts values from a table into global variables and modifies a hardware register.
- `process_hardware_status`: Enters a loop that manipulates a byte at a memory location using bitwise operations and checks a global flag to determine whether to exit or continue.
- `call_interleave_routine`: A routine consisting of 38 instructions that calls another routine.
- `interleave_bits`: Combines bits from two different memory locations by masking every other bit and shifting one set to create an interleaved pattern.
- `increment_and_wait`: Increments a global variable, calls another routine, writes values to two memory locations, and then waits in a loop until a bit in a memory location is set.
- `write_word_bytes_then_poll`: Writes the low and high bytes of the data register word to two consecutive memory locations, then polls a status byte until its low bit is set.
- `mask_byte_and_store_value`: Keeps the top two bits of one memory byte, sets its bit 3, and writes the value 127 to a second memory byte.
- `update_hardware_state`: Modifies a byte in memory using bitwise operations and performs a loop that waits for a specific value in a global variable.
- `modify_hardware_register`: Performs bitwise operations on a value and stores the result into a hardware register.
- `wait_and_modify_register`: Decrements a counter in a loop until a specific bit in a memory location is set, then modifies a hardware register.
- `iterate_backwards_through_memory`: Iterates through a sequence of memory locations by calling another routine on each address, moving backwards through the sequence.
- `toggle_bit_via_memory`: Updates a byte in memory by masking bits, writes a value to another memory location based on a condition, and then restores the original byte value.
- `read_frame_word_by_index`: Returns a 16-bit word from a frame array whose index comes from a data register offset by minus three.
- `write_frame_word_by_index`: Writes the data register word into a frame array slot whose index comes from a data register offset by minus three, then returns.
- `increment_frame_word_by_index`: Adds one to a 16-bit frame array slot whose index comes from a data register offset by minus three, then returns the data register value.
- `no_op_routine`: The routine is empty and performs no instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_context_and_poll`: Saves the current state of all data and address registers into global memory locations.
- `store_context_and_return`: Saves the current state of several registers into global memory locations.
- `calculate_quotient_remainder`: Calculates a quotient and a remainder based on a value and stores them in global variables.
- `check_sign_and_return`: Calls another routine and returns a 64-bit value where the upper 32 bits are set to all ones if a register value is negative.
- `initialize_global_pointers`: Checks a global variable and, if it is zero, calls another routine to populate several global memory locations with the returned value.
- `update_global_counters`: Increments two global variables and decrements a third before overwriting a global variable with two different constant values.
- `poll_routine_loop`: Calls another routine and enters a loop that repeatedly calls that same routine until it returns a true value.
- `update_table_values`: Increments a global counter and, if a condition is met, iterates through a table to add a value to each entry and wrap it if it reaches a threshold.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_bit_and_trap`: Sets a specific bit in a byte at a memory location using a shift operation and then performs a software trap based on a comparison.
