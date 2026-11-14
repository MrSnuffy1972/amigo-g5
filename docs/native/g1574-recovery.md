# Game 1574 (no second prize (1990)(thalion)[cr ministry][t +2 supplex]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01574). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 25 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 40 of 40 named (0 author symbols), 40 of 40 described (2 by rule, 29 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1 of 1 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 548 of 548 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 42108 |
| code | 1824 |
| data | 850020 |
| unused | 7168 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf:bootblock | 1 | 36 | 2 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load | 1 | 94 | 3 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load | 1 | 330 | 32 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load | 1 | 88 | 3 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load | 1 | 0 | 0 |

## Cases handled

- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: custom-format (non-DOS) disk, 1741 non-empty blocks: matched against track-loaded segments
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: the boot block reads 16896 bytes from disk offset 805376 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: the boot block reads 2560 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: the boot block reads 19456 bytes from disk offset 867328 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: 4 logged loads cover 44544 bytes of the image
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: blocks 22-1564 (bytes 11264-801280) hold 24 packed streams (IMP: File Imploder, TPWM: Turbo Packer) found by scanning, 98% of the bytes
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: blocks 1606-1693 (bytes 822272-867328) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.70)
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: blocks 1732-1756 (bytes 886784-899584) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.07)
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: blocks 1758-1758 (bytes 900096-900608) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `disable_chips_then_idle_loop` (No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load +an address to an address (2560 bytes)): Entry of thirty-five instructions that loops forever if its argument is zero; otherwise it writes masks that clear the top bit, which disable DMA and interrupts, runs a delay countdown, and loops forever writing a fixed value to a global variable. Kind: checksum/data integrity trap. Pass path: The routine performs a bit-shifting loop that reconstructs a value (uVar3) from a data stream, likely a checksum or a signature embedded in the loaded data. If the reconstructed value matches the expected state (the `uVar2 == 0` branch lead.
- `initialize_blt_and_search` (No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load +an address to an address (2560 bytes)): Sets several hardware registers and structure values, calls another routine twice, and then enters a loop that calls another routine multiple times while searching for a specific sequence in a list at a2. Kind: disk hardware/track check. Pass path: the loop continues if cRam01000f66 != -2.
- `set_cia_bit` (No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load +an address to an address (2560 bytes)): Sets bit 0 of the byte at the address in a0, which points to a hardware register, and then branches to another routine. Kind: disk hardware/timing check. Pass path: the loop continues if CIAA_PRA bit 4 is set, implying a specific hardware state or timing condition is met during the bit manipulation.
- `modify_cia_and_branch` (No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load +an address to an address (2560 bytes)): Adds d0 to a 16-bit value in a global variable, sets a0 to a hardware register, clears bit 1 of that register, and branches to another routine. Kind: disk hardware/timing check. Pass path: the loop completes normally (in_D0w == 0).
- `subtract_cia_and_branch` (No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load +an address to an address (2560 bytes)): Subtracts d0 from a 16-bit value in a global variable, sets a0 to a hardware register, sets bit 1 of that register, and branches to another routine. Kind: disk hardware/timing check. Pass path: in_D0w == 0.
- `set_cia_bit_conditionally` (No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load +an address to an address (2560 bytes)): Checks a bit in d0 and, if it is not zero, clears bit 2 of the byte at the address in a0. Kind: disk hardware/sync check. Pass path: the branch where CIAB_PRB is modified by bit 2 (bit 4 in the mask) based on the condition of D0.
- `modify_cia_port_b` (No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load +an address to an address (2560 bytes)): Calculates a bit position from d0, sets bits 6 and 7 of the register at a fixed address, clears bit 7, clears a bit at the calculated position, and loops while bit 5 of a fixed address is set. Kind: hardware state manipulation/timing trap. Pass path: the code continues execution normally after the routine returns.
- `toggle_cia_bit_sequence` (No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load +an address to an address (2560 bytes)): Calculates a bit position from d0, sets bit 7 of the register at a fixed address, and performs a sequence of setting and clearing a bit at the calculated position in that register. Kind: disk hardware timing/sequence check. Pass path: the instruction following the call (rts).

## Functions


### No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf:bootblock

- `run_library_request_then_call`: Boot block entry of twenty-one instructions that makes one library request through a stored pointer and calls one unnamed routine.
- `issue_io_then_reset_global`: Fills in two Exec request blocks, issues each through a stored pointer, and calls a fixed-address routine between them, then clears a global variable before returning.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load

- `write_decrementing_global_until_wrap`: Writes a decrementing word to a global variable in a loop that runs until the counter wraps, then returns the value minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load

- `disable_chips_then_idle_loop`: Entry of thirty-five instructions that loops forever if its argument is zero; otherwise it writes masks that clear the top bit, which disable DMA and interrupts, runs a delay countdown, and loops forever writing a fixed value to a global variable.
- `read_capped_entry_field`: Reads a field from a linked structure referenced by a global pointer.
- `call_multiple_subroutines`: Calls four different routines in sequence and then returns.
- `call_toggle_routine`: Clears d0 and calls another routine.
- `store_registers_and_loop`: Stores d0 through d4 into a series of global variables, calls three different routines, and enters a loop that performs conditional checks on d0 and a global variable, branching to other routines based on the results.
- `decrement_global_and_branch`: Subtracts 10 from a 16-bit value in a global variable, calls another routine, and branches to another routine if the result is positive.
- `update_counters_and_branch`: Adds 512 to a 16-bit value in a global variable, decrements a 16-bit value in another global variable, and performs several conditional checks on a third global variable, calling another routine and branching to another routine if certain conditions are met.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_blt_and_search`: Sets several hardware registers and structure values, calls another routine twice, and then enters a loop that calls another routine multiple times while searching for a specific sequence in a list at a2.
- `write_constant_and_return_ff`: Writes the word 16384 to offset 36 of a structure in a6 and returns with d0 set to negative one.
- `write_constant_and_return_fe`: Writes the word 16384 to offset 36 of a structure in a6 and returns with d0 set to negative two.
- `write_constant_and_return_fd`: Writes the word 16384 to offset 36 of a structure in a6 and returns with d0 set to negative three.
- `write_constant_and_return_fc`: Writes the word 16384 to offset 36 of a structure in a6 and returns with d0 set to negative four.
- `process_bitmapped_data`: Decrements a counter in d7 and, if positive, enters a loop that reads bytes from a source at a1, performs bitwise operations using tables at a4 and a3, and stores the result in a2.
- `search_for_marker_sequence`: Loops through a list at a1, skipping words equal to negative 61680, until it finds the word 17545, then continues to skip words equal to 17545 and returns.
- `negate_and_branch`: Subtracts a 16-bit value from a global variable using d0, and if the result is positive, negates d0 and branches to another routine.
- `set_cia_bit`: Sets bit 0 of the byte at the address in a0, which points to a hardware register, and then branches to another routine.
- `toggle_bit_and_wait`: Toggles bit 0 of the byte at a0, calls another routine, and enters a loop that repeats these actions while bit 4 of a hardware register is set, finally clearing a global word.
- `modify_cia_and_branch`: Adds d0 to a 16-bit value in a global variable, sets a0 to a hardware register, clears bit 1 of that register, and branches to another routine.
- `subtract_cia_and_branch`: Subtracts d0 from a 16-bit value in a global variable, sets a0 to a hardware register, sets bit 1 of that register, and branches to another routine.
- `toggle_memory_bit_loop`: Clears bit 0 of a byte at the address in a0, sets bit 0 of the same byte, calls another routine, and repeats this loop while decrementing d0.
- `loop_subroutine_sixty_seven_times`: Sets d1 to 67 and enters a loop that calls another routine 68 times, decrementing d1 each time.
- `compare_byte_with_itself`: Compares a byte at offset 6 of a structure in a6 with itself and returns if they are not equal.
- `set_cia_bit_conditionally`: Checks a bit in d0 and, if it is not zero, clears bit 2 of the byte at the address in a0.
- `set_byte_bit`: Sets bit 2 of a byte at the address pointed to by a0 and returns.
- `modify_cia_port_b`: Calculates a bit position from d0, sets bits 6 and 7 of the register at a fixed address, clears bit 7, clears a bit at the calculated position, and loops while bit 5 of a fixed address is set.
- `toggle_cia_bit_sequence`: Calculates a bit position from d0, sets bit 7 of the register at a fixed address, and performs a sequence of setting and clearing a bit at the calculated position in that register.
- `transform_data_loop`: Decrements a counter in d6 and, in a loop, reads a byte from a1, performs bitwise operations with d0, and uses the result to fetch a word from a table at a3.
- `generate_crc_table`: Calculates a series of 16-bit values using a loop that performs a bitwise shift and a conditional XOR with a constant, then stores the results in a table pointed to by a0.
- `populate_dual_tables`: Fills two separate memory areas with values derived from a loop that performs bit shifts and additions, storing a byte to one location and a shifted version of that byte to another.
- `call_two_routines`: Calls two different routines and then returns.
- `configure_hardware_registers`: Writes specific 16-bit values to several hardware registers and a global variable, then performs tests on two memory locations.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `shift_packed_longword_right`: Returns the longword four bytes before the address register shifted right one bit, with the top bit set when a flag bit in a data register is set.
- `shift_bits_from_packed_longwords`: Shifts bits out of a register for a count taken from its argument, reloading from the previous packed longword whenever the register empties and feeding a flag into the top bit.
