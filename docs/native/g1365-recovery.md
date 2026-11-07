# Game 1365 (vroom (1991)(lankhor)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01365). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903696 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 19 raw-word lines |
| Functions named and described | pass, 32 of 32 named (0 author symbols), 32 of 32 described (2 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 165 of 165 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 491 of 491 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 26492 |
| code | 1844 |
| data | 614852 |
| padding | 924 |
| unused | 259584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Vroom (1991)(Lankhor).adf:bootblock | 1 | 16 | 1 |
| logged disk load from Vroom (1991)(Lankhor).adf boot-block load | 1 | 0 | 0 |
| logged disk load from Vroom (1991)(Lankhor).adf boot-block load | 1 | 35 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 233 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 207 | 12 |

## Cases handled

- vroom (1991)(lankhor)/Vroom (1991)(Lankhor).adf: custom-format (non-DOS) disk, 1251 non-empty blocks: matched against track-loaded segments
- vroom (1991)(lankhor)/Vroom (1991)(Lankhor).adf: the boot block reads 20480 bytes from disk offset 584192 to an address (found statically); recovered as a unit
- vroom (1991)(lankhor)/Vroom (1991)(Lankhor).adf: the boot block reads 7680 bytes from disk offset 576512 to an address (found statically); recovered as a unit
- runtime segment an address: 78 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 87 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- vroom (1991)(lankhor)/Vroom (1991)(Lankhor).adf: 2 logged loads cover 28160 bytes of the image
- vroom (1991)(lankhor)/Vroom (1991)(Lankhor).adf: blocks 1584-1682 (bytes 811008-861696) outside the filesystem hold data, not code (opcode density 0.015, entropy 7.09)
- vroom (1991)(lankhor)/Vroom (1991)(Lankhor).adf: blocks 1749-1749 (bytes 895488-896000) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.07)
- 3 packed track region(s), 561152 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `read_joystick_and_mouse_ports` (runtime segment an address): Reads the joystick and mouse port registers JOY0DAT and JOY1DAT, the CIA port registers and POTGOR, and updates a small counter tracking the fire and right and middle buttons. Kind: track/sync check or manual/code-wheel lookup. Pass path: the code following the call where the routine executes and logic continues without resetting or altering control-flow-critical counters or flags used for validation.
- `pulse_cia_bit_while_flagged` (runtime segment an address): While a flag bit is set it sets a second bit on a CIA port and calls an unnamed routine, then zeroes the status once the flag clears. Kind: disk hardware/timing/sync check. Pass path: loop terminates, DAT_0001d916 becomes 0.
- `step_drive_by_position_difference` (runtime segment an address): Compares a stored position with a target value, sets direction bits on CIAB_PRB, and calls an unnamed routine once for each step needed. Kind: position-based track/sector check. Pass path: the branch where uVar1 becomes 0 (loop terminates without extra logic, or specifically the branch where the difference logic executes without triggering a fault/re-check).
- `set_cia_flag_then_delay` (runtime segment an address): Clears bit 0 of a flag and then sets it, then runs a nested delay loop. Kind: disk hardware/sync/timing check. Pass path: the code following the routine executes without the delay-induced timing or flag-state corruption, or specifically the bsr.w an address/sub_003C branch logic if certain flags or timing-dependent bit-tests pass..
- `read_mfm_track_sectors` (runtime segment an address): Splits a disk byte offset into track and sector using the 5632-byte track size, then writes the MFM sync word and the DSKLEN disk length register to start reading the track. Kind: track/sector-based check/sync-check. Pass path: the branch that continues the loop or exits successfully via the return or a valid loop iteration, rather than a failure or a trap/sabotage path or an error-driven exit (implied by the check against `unaff_D7` or `u4__d0_03DE` or `cVar3 == .

## Functions


### Vroom (1991)(Lankhor).adf:bootblock

- `initialize_and_copy_memory`: Configures a structure with a pointer and or certain values, calls a function, configures a second structure, calls a function again, and then copies a block of memory from a global variable to a fixed destination.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_return_stack_value`: Two-instruction routine that only returns a value taken from the stack.
- `dispatch_interrupt_request_bits`: Reads the interrupt enable and request registers, testing the master enable bit and the request masks 64, 32 and 16 to dispatch on the first set bit through a handler call or a counter update.
- `select_first_pending_interrupt_bit`: When the master interrupt enable bit is set, it checks request masks 128, 256, 512 and 1024 in that order and stores the first one found set in a global.
- `call_handler_for_request_bits`: Checks request masks 256, 512 and 1024 in order and, for the first one set, stores that mask in a global and calls the handler at a fixed slot of a handler table.
- `call_handler_for_request_512`: Checks request masks 512 and 1024 and calls the matching handler slot from the same table for whichever is set first.
- `call_handler_for_request_1024`: Checks request mask 1024 and, when it is set, stores it in a global and calls the handler at its table slot.
- `stub_return_stack_value_2`: Two-instruction routine that only returns a value taken from the stack.
- `stub_return_only`: This routine is a bare return and performs no actions.
- `stub_touch_cia_return`: A three-instruction routine that touches the CIA hardware register block and returns a stack value with no other work.
- `run_indexed_handler_once`: Tests a signed index and, when it is non-negative, stores a table entry, sets the index to a sentinel and calls through the entry; otherwise it calls one unnamed routine and clears the index.
- `run_indexed_handler_once_2`: Same pattern as its neighbour: it tests a signed index, then either calls through a stored table entry or calls once and clears the index.
- `call_routine_and_clear_index`: When a signed index is non-negative, calls one unnamed routine and then sets the index to minus one.
- `write_stepped_longword_table`: Writes a sequence of longword entries to a buffer, each holding a running value that increases by a fixed step and split into two 16-bit words.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `set_display_fetch_registers`: Writes the bitplane control register and the display data fetch start and stop registers with fixed values.
- `mask_byte_with_fixed_bits`: Returns its byte argument with the bits of the constant 238 ORed in, and has no other effect.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `read_joystick_and_mouse_ports`: Reads the joystick and mouse port registers JOY0DAT and JOY1DAT, the CIA port registers and POTGOR, and updates a small counter tracking the fire and right and middle buttons.
- `stub_bad_instruction_data`: The decoded body is bad instruction data, so no real behaviour can be recovered from it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_flag_with_bounded_retries`: Polls a status value for bit 2 and returns once it is set, decrementing a retry counter while bit 4 is set.
- `toggle_cia_port_until_ready`: Clears and sets bits on a CIA port register in a loop, polling a level bit until it reads set, then waits while another bit stays high.
- `delay_busy_loop`: Burns time in nested countdown loops, with a fixed outer count and an inner count taken from a global.
- `pulse_cia_bit_while_flagged`: While a flag bit is set it sets a second bit on a CIA port and calls an unnamed routine, then zeroes the status once the flag clears.
- `clear_status_global`: Sets one global status value to zero and returns.
- `step_drive_by_position_difference`: Compares a stored position with a target value, sets direction bits on CIAB_PRB, and calls an unnamed routine once for each step needed.
- `set_cia_flag_then_delay`: Clears bit 0 of a flag and then sets it, then runs a nested delay loop.
- `split_offset_to_track_sector`: Adds two stored offsets, subtracts a fixed base, and splits the result by 5632, the byte size of one Amiga track, into a quotient and a remainder.
- `read_mfm_track_sectors`: Splits a disk byte offset into track and sector using the 5632-byte track size, then writes the MFM sync word and the DSKLEN disk length register to start reading the track.
- `split_offset_and_call_reader`: Adds two stored offsets, rebases the sum, and splits it by 5632 bytes into track and sector values.
- `decode_mfm_sector_with_checksum`: Skips MFM sync words and decodes sector data with alternating-bit masks, accumulating an exclusive-or checksum and stopping when it is non-zero.
- `merge_mfm_odd_even_bits`: Combines two longwords with a bit mask and shifts one result left by one bit, the usual odd and even bit merge for MFM data.
