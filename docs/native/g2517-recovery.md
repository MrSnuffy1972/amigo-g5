# Game 2517 (escape from colditz (1990)(digital magic)(m4)[cr skr][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02517). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903152 top-level bytes in 45 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 43 of 43 named (0 author symbols), 43 of 43 described (10 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 196 of 196 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 568 of 568 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 710988 |
| code | 1920 |
| data | 167931 |
| padding | 11049 |
| unused | 11264 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SKR | 2 | 24 | 4 |
| Escape from Colditz (1990)(Digital Magic)(M4)[cr SKR][a2].adf:SKR_COLD | 2 | 85 | 4 |
| Escape from Colditz (1990)(Digital Magic)(M4)[cr SKR][a2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 398 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- escape from colditz (1990)(digital magic)(m4)[cr skr][a2]/Escape from Colditz (1990)(Digital Magic)(M4)[cr SKR][a2].adf:LOADTUNE.MUS: high-entropy data file (7.82 bits/byte), classified as packed data
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 55 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 23 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `search_hashed_record_chain` (runtime segment an address): Reads bytes from a data stream, computes an index into a table of pointers from a running checksum, and walks a linked chain of records comparing characters until it finds a match. Kind: checksum/hash-based record validation. Pass path: The loop that validates the record chain and eventually reaches the block copy/data loading logic (u4__l0_0116) after matching the identifier and verifying the record contents..
- `reset_two_globals_after_calls` (runtime segment an address): Calls two unnamed routines, then sets two global variables to zero and returns. Kind: checksum or state reset trap. Pass path: the routine continues to the next instruction after the sub-calls, assuming the registers/globals were validated or reset to a known state.
- `read_track_sectors_with_checksum` (runtime segment an address): Reads a track using the disk DMA and sync-word registers, decoding each sector's odd and even bits into memory and folding them into a checksum. Kind: checksum trap / track verification. Pass path: the instructions immediately following the call: moveq #an address, d2; movea.l an address(pc), a0; lea.l an address.l, a2.

## Functions


### SKR

- `call_library_entry`: Calls a routine and tests the zero flag, and when the flag is clear it calls a second routine and then a function pointer taken from a global table.
- `open_dos_library`: Calls the exec library's OldOpenLibrary function through a table offset to open dos.library, and stores the returned library handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Escape from Colditz (1990)(Digital Magic)(M4)[cr SKR][a2].adf:SKR_COLD

- `unpack_bitstream_backwards`: Unpacks a compressed stream into memory by writing output from the end of a buffer toward the start.
- `countdown_store_global_loop`: Stores every value of a 16-bit counter into a global variable while counting down from minus one through a full wrap-around, then returns.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Escape from Colditz (1990)(Digital Magic)(M4)[cr SKR][a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_loop_routine`: The routine contains no reachable instructions and consists of an infinite loop that never executes any operation.
- `check_interrupt_status_and_acknowledge`: Reads the interrupt enable and request registers and, when the master interrupt enable bit is set, masks the request bits and acknowledges the pending ones by writing to the request register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `search_hashed_record_chain`: Reads bytes from a data stream, computes an index into a table of pointers from a running checksum, and walks a linked chain of records comparing characters until it finds a match.
- `set_copy_window_bounds`: Stores three global bounds values derived from an address register and a global base, then calls several small routines.
- `copy_longword_range`: Copies consecutive longwords from a source pointer to a destination pointer, advancing both, until the source reaches an upper bound held in a global variable.
- `copy_block_to_low_memory`: Copies a byte block into low memory at a fixed start address, using a source pointer computed from a global base and a counter initialised from the routine's own address.
- `restore_low_memory_block`: Copies the same kind of byte block in the reverse direction, from low memory back out to the source region, with the count computed the same way.
- `decode_backward_run_stream`: Decodes a packed stream into memory from the end toward the start: a clear flag copies literal bytes, while a set flag reads a length code and copies a run of earlier bytes.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `cycle_byte_counter_space_wrap`: Decrements a byte register once per pass of a loop whose count comes from the input value, resetting it to a space character when it reaches zero.
- `set_bit_four_of_byte`: Returns the low byte of the data register with bit four set, and changes nothing else.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `or_bit_four_into_byte`: Returns the low byte of the data register with bit four set, in a single instruction.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 16 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 208 into d2.
- `stub_immediate_return`: The pseudo-code only returns, so no work is visible in the evidence, although six instructions are recorded for the routine.
- `reset_two_globals_after_calls`: Calls two unnamed routines, then sets two global variables to zero and returns.
- `convert_offset_seek_track`: Converts a scaled offset into a track index and an offset within the track by dividing by a fixed track size, then steps the current track value toward the target by calling routines.
- `update_control_bits_if_even`: When the low bit of the data register is clear, it clears bits one and zero of the byte addressed by a register, sets bit zero, and calls one unnamed routine.
- `set_control_bits_if_odd`: When the low bit of the data register is set, it sets bit one of the byte addressed by a register, clears and then sets bit zero, and calls one unnamed routine.
- `poll_status_bit_while_set`: Sets bit one of the byte addressed by a register, then repeatedly toggles bit zero and calls one unnamed routine for as long as a status bit in a byte at a fixed offset stays set.
- `read_track_sectors_with_checksum`: Reads a track using the disk DMA and sync-word registers, decoding each sector's odd and even bits into memory and folding them into a checksum.
- `write_value_then_clear_bit`: Writes a fixed byte value into the byte addressed by a register, then clears bit three of it.
- `set_flag_bit_by_parity`: Sets bit 2 of the byte addressed by a register when the counter in a global variable is even, and clears that bit when it is odd.
- `delay_countdown_loop`: Counts a 16-bit value down from a fixed starting count to minus one and then returns.
- `measure_null_terminated_length`: Walks a zero-terminated byte string through an address register, counting the bytes before the terminator and keeping the running count in a global variable.
- `stub_single_instruction_routine`: The routine is one instruction long and is reached through address references from four unnamed routines.
- `tiny_ori_3`: Small routine of 1 instruction that merges 128 into d4.
- `stub_single_move_routine`: A one-instruction routine whose address is taken by moves from two unnamed routines.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 12 into d0, then merges 0 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_own_code_then_spin`: Copies a fixed number of longwords from the start of its own code into a global variable, then enters an infinite loop that never exits.
- `copy_own_code_block_loop`: Copies the same block of longwords from its own code into a global variable and then loops forever.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_minus_one_stub`: Returns negative one as its result and performs no other work.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
