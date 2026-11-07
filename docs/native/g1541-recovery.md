# Game 1541 (golden axe (1990)(virgin)[cr angels - genesis - defjam][t +2 angels]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01541). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904416 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 41 of 41 named (0 author symbols), 41 of 41 described (5 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 81 of 81 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 403 of 403 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2410 |
| code | 1274 |
| data | 896124 |
| unused | 4608 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Angels].adf:bootblock | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 390 | 39 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |

## Cases handled

- golden axe (1990)(virgin)[cr angels - genesis - defjam][t +2 angels]/Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Angels].adf: custom-format (non-DOS) disk, 1749 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 74 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- golden axe (1990)(virgin)[cr angels - genesis - defjam][t +2 angels]/Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Angels].adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- golden axe (1990)(virgin)[cr angels - genesis - defjam][t +2 angels]/Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Angels].adf: blocks 2-10 (bytes 1024-5632) hold ASCII text
- golden axe (1990)(virgin)[cr angels - genesis - defjam][t +2 angels]/Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Angels].adf: blocks 13-25 (bytes 6656-13312) outside the filesystem hold data, not code (opcode density 0.009, entropy 4.23)
- golden axe (1990)(virgin)[cr angels - genesis - defjam][t +2 angels]/Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Angels].adf: blocks 28-1742 (bytes 14336-892416) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.07)
- 1 packed track region(s), 5632 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `write_constant_then_return_zero` (runtime segment an address): Stores a fixed constant into a control byte, calls a subroutine and returns zero. Kind: checksum trap / integrity check. Pass path: return 0.
- `bit_bang_pulses_while_flagged` (runtime segment an address): While a flag bit in a global byte is set, it sets a bit in a control byte, calls a subroutine, and waits until a second flag bit clears. Kind: disk hardware timing/sync check. Pass path: the loop terminates and execution continues to the instruction following the call (clr.b (a2)).
- `alternate_control_bit_by_value` (runtime segment an address): Stores the input byte into a memory location and branches on its lowest bit. Kind: disk hardware/sync check. Pass path: the branch where (in_D0b & 1) != 0 (the 'else' path).
- `clear_control_bit_then_call` (runtime segment an address): Clears bit two of a control byte and calls a subroutine. Kind: control bit check/trap. Pass path: u1__sub_083C() continues execution normally.
- `pulse_control_bit_and_wait` (runtime segment an address): Clears and then sets the lowest bit of a control byte, calls a subroutine, and waits until a flag bit in a global byte is clear. Kind: disk hardware timing/sync check. Pass path: the loop terminates when bit 5 of an address is cleared, allowing execution to proceed to the next instruction.
- `emit_signed_step_pulses` (runtime segment an address): Compares a stored byte with a new value and sets a direction bit on a control byte to match the sign of the difference. Kind: disk hardware timing/pulse check. Pass path: the code following the call site (bsr.w an address, bsr.w an address, nop).
- `emit_step_pulses_from_registers` (runtime segment an address): Compares two register values after halving them and sets a direction bit on a control byte to match the sign of the difference. Kind: disk hardware timing/pulse control. Pass path: the routine executes the loop to emit the required number of pulses/cycles to synchronize the disk head or motor timing.
- `emit_pulses_with_clear_direction` (runtime segment an address): Clears bit one of a control byte, then calls a subroutine once per unit of a count register. Kind: disk hardware timing/pulse check. Pass path: the code following the loop/subroutine completion.
- `write_fixed_control_then_call` (runtime segment an address): Writes a fixed control value to a port register and calls a subroutine; nothing else is done. Kind: checksum or integrity trap. Pass path: the instruction immediately following the call (the nop sequence).
- `retry_bit_pulse_until_ready` (runtime segment an address): Clears and sets a bit in a control byte around a series of subroutine calls, repeating while a subroutine reports a nonzero result. Kind: disk hardware/timing check. Pass path: the loop continues to u1__sub_09E2 and u1__sub_0A6C, eventually returning when unaff_A5 <= extraout_A0.

## Functions


### Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Angels].adf:bootblock

- `copy_boot_data_low_memory`: Copies one kilobyte of boot data into low memory at a fixed destination, one longword at a time.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `build_bit_reversed_byte_table`: Fills a 256-entry table in reverse order with the bit-reversed form of each byte value, shifting one bit at a time for eight steps per entry.
- `run_init_steps_entry_zero`: Runs four subroutines in sequence, then calls another subroutine only when a flag byte in table entry zero is nonzero.
- `run_init_steps_for_index`: Runs four subroutines in sequence, then calls another subroutine when a flag byte in a table, indexed by an input register, is nonzero.
- `write_constant_then_return_zero`: Stores a fixed constant into a control byte, calls a subroutine and returns zero.
- `tiny_nop_rts`: Small routine of 2 instructions that does nothing for one instruction, then returns.
- `call_subroutine_return_zero`: Calls one subroutine and returns zero.
- `write_three_bytes_then_wait`: Writes three constants to a control location in turn, then polls a status byte until its lowest bit is set.
- `program_cia_timer_and_wait`: Writes three values to the CIA B timer control and timer count registers, then polls the interrupt control status until its lowest bit is set.
- `bit_bang_pulses_while_flagged`: While a flag bit in a global byte is set, it sets a bit in a control byte, calls a subroutine, and waits until a second flag bit clears.
- `tiny_clr_rts`: Small routine of 2 instructions that clears (a2), then returns.
- `alternate_control_bit_by_parity`: Increments a counter byte and branches on its parity.
- `alternate_control_bit_by_value`: Stores the input byte into a memory location and branches on its lowest bit.
- `clear_control_bit_then_call`: Clears bit two of a control byte and calls a subroutine.
- `pulse_control_bit_and_wait`: Clears and then sets the lowest bit of a control byte, calls a subroutine, and waits until a flag bit in a global byte is clear.
- `emit_signed_step_pulses`: Compares a stored byte with a new value and sets a direction bit on a control byte to match the sign of the difference.
- `emit_step_pulses_from_registers`: Compares two register values after halving them and sets a direction bit on a control byte to match the sign of the difference.
- `emit_pulses_with_clear_direction`: Clears bit one of a control byte, then calls a subroutine once per unit of a count register.
- `emit_pulses_with_set_direction`: Sets bit one of a control byte, then calls a subroutine once per unit of a negated count register.
- `write_fixed_control_then_call`: Writes a fixed control value to a port register and calls a subroutine; nothing else is done.
- `retry_bit_toggle_until_done`: Repeatedly checks two conditions by calling a subroutine, and while either returns nonzero it clears and sets a bit in a control byte through a series of further calls.
- `stub_two_instruction_call`: A two-instruction routine that makes one call.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `retry_bit_pulse_until_ready`: Clears and sets a bit in a control byte around a series of subroutine calls, repeating while a subroutine reports a nonzero result.
- `wait_for_disk_sync_word`: Sets up the disk controller for reading with the standard MFM sync pattern, then waits for the sync interrupt flag with a timeout.
- `start_disk_read_with_sync`: Points the disk DMA address at a buffer, sets the disk sync and DMA control registers, then waits for the sync interrupt flag with a timeout.
- `retry_disk_sync_until_match`: Repeatedly sets the disk sync and DMA registers and waits for the sync flag, checking the data for the standard MFM sync word until it matches.
- `decode_mfm_sector_with_checksum`: Checks the sync word in memory and then reads blocks of longwords through the sector-read routine, XOR-combining pairs into a checksum masked to every other bit.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `copy_decoded_sector_bytes`: Copies decoded bytes into a destination range, XOR-combining longwords into a checksum for each block of 128 longwords and comparing it with an expected value.
- `copy_decoded_sector_bytes_2`: Runs the same block checksum loop with a different copy stride, comparing each 128-longword block against the expected value.
- `return_failure_status`: Returns a fixed failure value of minus one with no other work.
- `return_error_status`: returns a value representing a failure status to the caller.
- `scan_for_disk_sync_marker`: Reads 16-bit words from the input pointer until it finds the standard MFM sync word, then calls two subroutines and returns zero.
- `call_two_subroutines_return_zero`: Calls two subroutines in sequence and returns zero, with no other work of its own.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `merge_odd_even_bit_streams`: Masks every other bit of two longwords, shifts the first masked value left by one bit, and combines the two with a bitwise OR. The mask pattern is the usual split of MFM data bits into odd and even streams.
- `decompress_back_reference_stream`: Decodes a bit stream of literal and back-reference codes, copying data backwards into the destination until the output pointer reaches the start.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_wrapping_counter_loop`: Decrements a byte counter in a loop for a count taken from a register, resetting it to the decimal value 32 when it reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_counter_clear_flag`: Decrements a 16-bit counter in memory and clears the top bit of a global flag word, then returns the register value it was given.
