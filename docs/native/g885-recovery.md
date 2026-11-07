# Game 885 (hybris (1988)(discovery software)[cr 9009 xad][m ccs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00885). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902352 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 27 of 27 named (0 author symbols), 27 of 27 described (0 by rule, 14 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 107 of 107 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2048 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 641 of 766 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 306 |
| code | 3118 |
| data | 730734 |
| padding | 1282 |
| unused | 166912 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf:hybris | 2 | 25 | 2 |
| Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf:bootblock | 1 | 112 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 234 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 31 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf bytes 443904-444416 (code never loaded by a reference run) | 1 | 92 | 2 |
| Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf bytes 481792-482816 (code never loaded by a reference run) | 1 | 143 | 2 |
| Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf bytes 737280-737792 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address (play-long-joy): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 53 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 25 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- hybris (1988)(discovery software)[cr 9009 xad][m ccs]/Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf: bytes 443904-444416 look like code no run loaded: recovered position-independently (load address unknown)
- hybris (1988)(discovery software)[cr 9009 xad][m ccs]/Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf: bytes 481792-482816 look like code no run loaded: recovered position-independently (load address unknown)
- hybris (1988)(discovery software)[cr 9009 xad][m ccs]/Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf: bytes 737280-737792 look like code no run loaded: recovered position-independently (load address unknown)
- hybris (1988)(discovery software)[cr 9009 xad][m ccs]/Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf: 3 logged loads cover 2048 bytes of the image
- hybris (1988)(discovery software)[cr 9009 xad][m ccs]/Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- hybris (1988)(discovery software)[cr 9009 xad][m ccs]/Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf: blocks 836-864 (bytes 428032-442880) outside the filesystem hold data, not code (opcode density 0.035, entropy 6.33)
- hybris (1988)(discovery software)[cr 9009 xad][m ccs]/Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf: blocks 902-938 (bytes 461824-480768) outside the filesystem hold data, not code (opcode density 0.031, entropy 6.40)
- 1 never-loaded region(s), 512 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 14 packed track region(s), 598528 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `verify_boot_marker_then_branch` (bootblock): Waits for a marker word to match, sends a device request, and checks a system field before branching. Kind: checksum/data-validation trap. Pass path: the branch that executes the loop (psVar11 or sVar4 check) and eventually calls u2__sub_02F0 and the subsequent indirect jump, avoiding the branch that triggers the secondary logic or the specialized conditional logic involving bVar5/XORing.
- `manipulate_global_bits` (runtime segment an address (play-long-joy)): Performs a sequence of bitwise operations on a global variable, including clearing and setting certain bits, while calling another routine. Kind: track/sync check or manual/code-wheel lookup. Pass path: The loop executes a sequence of bit-clearing and bit-setting operations (clearing bit 1, setting bit 0) that depend on the value of `bVar2` (returned by `u4__sub_028C`) and a loop counter `sVar3` (derived from `bVar2`). A successful check a.
- `set_global_and_poll_bit` (runtime segment an address (play-long-joy)): Writes two constant values to a global variable, calls another routine, and then polls a global variable until a specific bit is cleared. Kind: disk/track sync or presence check. Pass path: the loop terminates when bit 1 of the status register (DSKSYNC or similar) or a hardware flag is set, allowing execution to proceed to the next instruction (setting the next register/address or proceeding to the next stage of the loader or .
- `boot_step_to_next_track` (runtime segment an address (play-long-joy)): Boot loader: switches side or steps one cylinder in. Kind: track/sector check or conditional loader. Pass path: the branch where (in_D0 & 1) == 0 (the if-block executes).
- `boot_drive_off` (runtime segment an address (play-long-joy)): Boot loader: stops the motor and deselects the drive. Kind: sabotage path. Pass path: The routine executes a sequence of bit-toggling or state-changing operations (setting/clearing bits in `DAT_00bfd100`) followed by calls to `u4__sub_0360`. Because the routine modifies a shared state variable in a specific, non-linear patte.

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longwords_then_call`: When the destination pointer is set, copies sixteen longwords from a block at a fixed offset into it, then calls the routine stored at a pointer.

### Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf:hybris

- `enter_supervisor_and_configure_hardware`: Enters supervisor state through exec, writes the interrupt enable, DMA control and CIA B port registers, then calls one further routine.
- `run_short_handler_callback`: A seven-instruction handler whose address is stored in data and which is called from one routine.

### Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf:bootblock

- `verify_boot_marker_then_branch`: Waits for a marker word to match, sends a device request, and checks a system field before branching.
- `open_libraries_and_resources`: Uses library calls to open a disk resource and a DOS library, then updates a system state.
- `calculate_checksum_and_initialize_memory`: Calculates a checksum by summing values in a memory range and stores the bitwise-inverted result at the end of that range, while also initializing certain memory locations and looking up the intuition library.
- `store_inverted_checksum_of_block`: Sums 24 words of a block starting at a fixed offset and stores the bitwise inverse of the sum in the following word.
- `look_up_intuition_library`: Refers to the intuition.library name and is called from one routine.
- `disable_all_interrupts_and_dma`: Writes an all-bits-clear mask to the interrupt enable and DMA control registers, turning off every interrupt source and DMA channel.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block`: Clears a block of memory by storing zero at an offset from a pointer for a fixed number of iterations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_address_and_clear_memory`: Checks if a register contains any of a set of specific addresses and loops until a certain address is reached, then assigns a register value to a global variable, calls another routine, and clears a block of memory starting from a fixed address up to a certain limit.
- `search_and_skip_pattern`: Searches through a memory range for a specific pattern and skips a fixed number of bytes if the pattern is found.
- `store_if_within_bounds`: Checks if a pointer or address is less than a value derived from a global variable and stores a register value to that pointer if the condition is met.
- `manipulate_global_bits`: Performs a sequence of bitwise operations on a global variable, including clearing and setting certain bits, while calling another routine.
- `wait_for_bit_clear`: Polls a global variable until a specific bit is cleared.
- `set_global_and_poll_bit`: Writes two constant values to a global variable, calls another routine, and then polls a global variable until a specific bit is cleared.
- `boot_step_to_next_track`: Boot loader: switches side or steps one cylinder in.
- `boot_drive_off`: Boot loader: stops the motor and deselects the drive.
- `return_from_subroutine`: the routine is empty and performs no operations before returning.
- `decrement_counter`: Decrements a register or variable from a fixed starting value down to zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_state`: Sets a global variable, checks a hardware register, and copies a block of data from one memory location to another based on a condition.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_unspecified_routine`: This routine contains seven instructions and is entered from outside the static code flow.

### Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf bytes 443904-444416 (code never loaded by a reference run)

- `call_two_library_vectors`: Loads a value from a fixed data word and calls two library routines through their vector offsets, storing the results in globals.
- `call_library_with_string_argument`: Walks a null-terminated string and, if the string is not empty, calls a library routine with the position just past its terminator.

### Hybris (1988)(Discovery Software)[cr 9009 Xad][m CCS].adf bytes 481792-482816 (code never loaded by a reference run)

- `call_library_vectors_after_load`: Loads a value from a fixed data word and calls library routines through vector offsets, storing results in globals.
- `find_string_and_call_function`: Traverses a null-terminated string and calls a function pointer if a non-zero byte is found.
