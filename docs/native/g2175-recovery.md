# Game 2175 (great giana sisters, the (1988)(time warp)[cr rsi][a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02175). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901520 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 29 of 29 named (0 author symbols), 29 of 29 described (2 by rule, 18 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 17 of 17 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 526 of 526 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 76966 |
| code | 2136 |
| data | 776850 |
| unused | 45568 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf:bootblock | 1 | 12 | 1 |
| logged disk load from Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf boot-block load | 1 | 441 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 73 | 8 |

## Cases handled

- great giana sisters, the (1988)(time warp)[cr rsi][a3]/Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf: custom-format (non-DOS) disk, 1669 non-empty blocks: matched against track-loaded segments
- great giana sisters, the (1988)(time warp)[cr rsi][a3]/Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf: the boot block reads 81920 bytes from disk offset 337920 to an address (found statically); recovered as a unit
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- great giana sisters, the (1988)(time warp)[cr rsi][a3]/Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf: 1 logged loads cover 81920 bytes of the image
- great giana sisters, the (1988)(time warp)[cr rsi][a3]/Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf: blocks 2-654 (bytes 1024-335360) hold 2 packed streams (BK: ByteKiller) found by scanning, 89% of the bytes
- great giana sisters, the (1988)(time warp)[cr rsi][a3]/Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf: blocks 657-659 (bytes 336384-337920) outside the filesystem hold data, not code (opcode density 0.040, entropy 4.33)
- great giana sisters, the (1988)(time warp)[cr rsi][a3]/Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf: blocks 820-880 (bytes 419840-451072) outside the filesystem hold data, not code (opcode density 0.014, entropy 2.14)
- great giana sisters, the (1988)(time warp)[cr rsi][a3]/Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf: blocks 902-1693 (bytes 461824-867328) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.82)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `issue_io_write_cia_port` (bootblock): Boot block entry of twelve instructions that issues an Exec DoIO request while writing the CIA B port register and the DMA control register. Kind: disk hardware/timing check. Pass path: The branch taken when the CIA port write/read sequence completes without triggering a hardware exception or timing mismatch (likely the instruction immediately following the call if it leads to the main loader)..
- `toggle_cia_bits` (Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf boot-block load +an address to an address (81920 bytes)): Sets the CIA port B register to 255, clears bit 7 and bit 3 of the CIA port B register through calls to another routine, sets bit 1 of the CIA port B register, and enters a loop that clears bit 0 of the CIA port B register and calls two other routines while bit 4 of the CIA port A register is set. Kind: disk hardware timing/sync check. Pass path: the loop continues and eventually exits via the while condition (CIAA_PRA bit 4 cleared) or the sVar3 loop completes normally.
- `process_cia_port_bits` (Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf boot-block load +an address to an address (81920 bytes)): Sets bit 2 of the CIA port B register, then enters a loop that shifts a byte from d0 into d2, clears bits 2, 1, or 0 of the CIA port B register based on the shifted value, and calls two other routines. Kind: disk hardware/timing check. Pass path: the loop executing u1__sub_02E0 and u1__sub_02EA.
- `read_raw_track_patch_site` (Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf boot-block load +an address to an address (81920 bytes)): Writes the value from d2 to the interrupt request register, sets the disk length to 16384, and sets the custom address control register to 400, then returns. Kind: disk hardware/track length check. Pass path: execution continues to the next instruction after the routine returns.
- `boot_step_to_next_track` (Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf boot-block load +an address to an address (81920 bytes)): Boot loader: switches side or steps one cylinder in. Kind: disk hardware/register check. Pass path: the branch where (in_D0 & 1) == 0 is false (the code continues past the if block).
- `poll_input_and_wait` (runtime segment an address): Checks the status of joystick fire buttons and disk status via CIA registers and executes a countdown wait. Kind: disk hardware/sync check. Pass path: the instruction immediately following the call (the code continues execution normally).
- `boot_step_to_next_track_2` (runtime segment an address): Boot loader: switches side or steps one cylinder in. Kind: disk hardware/status check. Pass path: the branch where (in_D0 & 1) != 0.
- `boot_drive_off_2` (runtime segment an address): Boot loader: stops the motor and deselects the drive. Kind: hardware register manipulation/drive state control. Pass path: the routine sets specific bits in the CIAB_PRB register (likely a custom disk controller register) to toggle drive states or modes, suggesting it is part of a sequence to prepare the hardware for a specific non-standard read or to verify th.

## Functions


### Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf:bootblock

- `issue_io_write_cia_port`: Boot block entry of twelve instructions that issues an Exec DoIO request while writing the CIA B port register and the DMA control register.

### logged disk load from Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf boot-block load

- `execute_boot_sequence`: Saves registers, sets the interrupt enable register to 32767, calls another routine, toggles bit 1 of the disk drive selection register, calls another routine, calls a routine at a pointer in memory, and then enters a loop that decrements d1 and calls another routine.
- `toggle_cia_bits`: Sets the CIA port B register to 255, clears bit 7 and bit 3 of the CIA port B register through calls to another routine, sets bit 1 of the CIA port B register, and enters a loop that clears bit 0 of the CIA port B register and calls two other routines while bit 4 of the CIA port A register is set.
- `process_cia_port_bits`: Sets bit 2 of the CIA port B register, then enters a loop that shifts a byte from d0 into d2, clears bits 2, 1, or 0 of the CIA port B register based on the shifted value, and calls two other routines.
- `wait_disk_status`: Polls bit 5 of the CIAA PRA register in a loop until it is cleared, then returns.
- `initialize_disk_read`: Sets the custom address control register to 33888, the disk sync register to 17769, calls another routine, sets the disk length to 16384, the disk path to an address, and the disk length to 39317, then waits for bit 1 of the interrupt request register to be set while decrementing d3 from an address.
- `read_raw_track_patch_site`: Writes the value from d2 to the interrupt request register, sets the disk length to 16384, and sets the custom address control register to 400, then returns.
- `boot_step_to_next_track`: Boot loader: switches side or steps one cylinder in.
- `boot_drive_off`: Boot loader: stops the motor and deselects the drive.
- `nop_padding`: Padding: four no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `delay_by_countdown`: Sets d7 to 10240 and decrements it in a loop until it reaches negative one, then returns.
- `process_data_table`: Uses a value from a2 to iterate through a table, performs bitwise operations on pairs of longwords to combine them, stores the result in a global variable, and uses a loop to process a sequence of data before returning.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `update_table_values`: Performs a loop that uses d0 to iterate through a table, performing bitwise rotations on values from three different memory locations and updating a pointer in a global variable.
- `update_and_call_routine`: Performs arithmetic on d0, copies seven words from one memory location to another in a loop, modifies specific bytes in a table, and calls a routine at a fixed address before jumping to another address.
- `compare_memory_byte`: Clears d1, loads a byte from a pointer in a6 into d1, clears d0, compares a byte at a memory address in a0 with d1, and branches based on the result.
- `calculate_bit_counts`: Calculates a memory offset using d0, then enters nested loops to read words from a table and count set bits, storing the results into a destination pointed to by a register.
- `restore_registers_and_jump`: Restores the status register and several registers from the stack, then performs an absolute jump.
- `configure_hardware_and_close_library`: Polls bit 6 of the CIAA PRA register, modifies the interrupt enable and DMA control registers, updates a global variable with a value from a pointer, and calls two routines to close a library and release a permit.
- `initialize_system_data_structures`: Opens a library, calculates a size from a table, and performs multiple memory operations including zeroing large blocks, copying data from source tables to destination buffers, and populating a series of 8-byte structures with specific byte patterns and integer values.
- `call_fixed_routine_then_store`: Three-instruction routine that makes one call to a fixed-address routine, then stores a constant into a global variable; the listed facts also note a DMA control register touch.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_input_and_wait`: Checks the status of joystick fire buttons and disk status via CIA registers and executes a countdown wait.
- `wait_while_cia_bit_set`: Three-instruction routine that loops while one bit of a global status variable remains set.
- `wait_for_disk_sync`: Sets the custom address control register to 33888 and the disk sync register to 17769, calls another routine, sets the disk length to 16384, the disk path to a specific address, and the disk length to 39317, then waits for bit 1 of the interrupt request register to be set while decrementing d3 from an address.
- `set_disk_hardware`: Writes a word from d2 to the interrupt request register, clears d7, sets the custom address control register to 16384, and sets the disk length to 1024.
- `boot_step_to_next_track_2`: Boot loader: switches side or steps one cylinder in.
- `boot_drive_off_2`: Boot loader: stops the motor and deselects the drive.
- `return_without_action`: Routine whose shown body is only a return, so it does no work in the pseudo-code.
- `busy_wait_word_countdown`: Counts a word from a fixed starting value down until it wraps, a busy-wait loop that other routines call.
