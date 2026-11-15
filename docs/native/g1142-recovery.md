# Game 1142 (chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01142). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1803530 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 34 raw-word lines |
| Functions named and described | pass, 287 of 287 named (0 author symbols), 287 of 287 described (32 by rule, 146 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2532 of 2573 (98.41%) |
| Recovered without load address (counted in proof 1, flagged) | 29696 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6018 of 6019 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 32319 |
| code | 22672 |
| data | 1522140 |
| padding | 95 |
| unused | 226304 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf:bootblock | 1 | 9 | 1 |
| Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| logged disk load from Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf boot-block load | 1 | 710 | 39 |
| logged disk load from Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf boot-block load | 1 | 72 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 130 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| logged disk load from Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf | 1 | 379 | 17 |
| logged disk load from Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf | 1 | 2968 | 169 |
| logged disk load from Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf | 1 | 1569 | 33 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 66 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 64 | 4 |
| Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf bytes 41984-42496 (code never loaded by a reference run) | 1 | 0 | 0 |
| Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf bytes 46080-75264 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1734 non-empty blocks: matched against track-loaded segments
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 2 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1340 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 2046 of the 2555 PCs the canonical run logged (533 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- 99 first-execution byte records contradict the logged opcode word (first an address): bad captures, the opcode word is used
- 143 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: the boot block reads 28672 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: the boot block reads 1536 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- 4 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-rs-m1500): 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 89 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: 134 executed PCs whose RAM was overwritten before any snapshot match its bytes 42000-46112: recovered as a unit run at an address
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: 590 executed PCs whose RAM was overwritten before any snapshot match its bytes 74768-90384: recovered as a unit run at an address
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: 989 executed PCs whose RAM was overwritten before any snapshot match its bytes 102048-108816: recovered as a unit run at an address
- 64 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 5 first-run image unit(s) in 1 version group(s); 464 unexecuted gap bytes taken from the RAM snapshot; 40 not decodable
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: bytes 41984-42496 look like code no run loaded: recovered position-independently (load address unknown)
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: bytes 46080-75264 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 immediate
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: 7 logged loads cover 85376 bytes of the image
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: blocks 9-10 (bytes 4608-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.84)
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: blocks 78-81 (bytes 39936-41984) outside the filesystem hold data, not code (opcode density 0.021, entropy 6.15)
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: blocks 177-198 (bytes 90624-101888) outside the filesystem hold data, not code (opcode density 0.051, entropy 6.89)
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: blocks 213-351 (bytes 109056-180224) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.67)
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: blocks 353-463 (bytes 180736-237568) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.23)
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: blocks 467-468 (bytes 239104-240128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.00)
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: blocks 472-1629 (bytes 241664-834560) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.83)
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 1 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf: blocks 1632-1759 (bytes 835584-901120) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.55)
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 2 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 2 of 2).adf: blocks 2-891 (bytes 1024-456704) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.02)
- chase h.q. ii - special criminal investigation (1990)(ocean)[cr skr - valhalla][t +3 skr](disk 2 of 2)/Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 2 of 2).adf: blocks 894-1343 (bytes 457728-688128) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.53)
- 2 never-loaded region(s), 29696 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_system_tables_and_hardware` (Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf +an address run at an address (4112 bytes, matched to executed PCs)): Copies a block of data from one memory location to another, clears a series of global variables, and performs a memory test by writing and reading bit-flipped values to a specific range. Kind: disk hardware/sync check. Pass path: the loop continues until sVar3 reaches -1, then proceeds to u9__visit_type_four_records.

## Functions


### Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf:bootblock

- `load_boot_stages_and_jump`: Fills in the I/O request in A1 twice (data address, length and offset fields) and calls the same library vector at offset -456 from A6 each time.

### Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 2 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### logged disk load from Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf boot-block load

- `load_track_data_and_decrunch`: Calls a trap, copies a block of longwords from a fixed source, then decodes a packed stream into a display buffer while computing track and sector positions with division by 11.
- `load_track_decrunch_b`: Shares the loader logic of the preceding routine, entered from a second call site.
- `copy_loader_blocks_until_limit`: Loops while a pointer is below a fixed limit, copying a block of longwords per pass and calling four unnamed routines.
- `setup_copper_lists_and_decode`: Sets the two copper list pointers, the copper jump and the background colour, then decodes a packed stream into the display buffer, calling unnamed routines.
- `map_sector_run_to_track`: Splits an input position into a track and sector using division and remainder by 11, then calls the blitter wait routine and other routines to consume bounded steps while updating a counter.
- `write_sync_marks_and_check`: Clears four frame fields and, when a flag is set, writes the two MFM sync words into a buffer.
- `start_disk_read_with_retries`: Repeats up to five times: it points disk DMA at a buffer, sets the MFM mode and a read length that is longer for large sector counts, enables disk DMA, and calls an unnamed routine until its completion flag is set.
- `validate_sector_header_with_retries`: Retries up to 11 times to read a sector header, checking each read's flag.
- `read_sector_data_and_verify`: Loops reading sector data, waiting for the disk ready bit, comparing sector and track values, and checking an allocation table.
- `frag_move_sub_mulu_rts`: Short code fragment that copies d3 to d1, then subtracts the memory at an offset from a6 from d1, then copies 512 to d0, then multiplies d1 by d0 (unsigned), then returns.
- `set_flag_increment_counter`: Sets one bit, chosen by the low five bits of data register D3, in a 16-bit word four bytes below the A6 base.
- `clear_table_entries`: Writes zero to a 32-bit entry at ten successive positions, moving the address pointer in A5 forward by 272 bytes before each write.
- `combine_low_word_top_byte`: Calls another routine and takes its 32-bit result.
- `interleave_alternate_bits`: Reads two consecutive 32-bit values from the address in A0.
- `call_and_forward_result`: Calls one routine, then passes its 32-bit return value as the argument to a second routine.
- `compute_fixed_xor_checksum`: XORs ten 32-bit values, starting eight bytes past the A5 base, into an accumulator.
- `compute_xor_checksum`: XORs a run of 32-bit values starting at the address in A0 into an accumulator.
- `fill_block_alternating_bits`: Fills a 1024-byte block with a repeating 32-bit pattern of alternating ones and zeros.
- `store_derived_structure_fields`: Calls a routine and computes three offsets from its result and two address-register outputs.
- `write_structure_transform_fields`: Calls several helper routines, one of which waits for the blitter, and writes groups of fields into a structure pointed to by A2.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `reset_blitter_state`: Waits for the blitter to finish, then loads a constant into the blitter C-data register and zeroes three blitter modulo registers.
- `refresh_high_bit_after_calls`: Calls two routines, then reads the byte at the address in A0 and the flag byte just before it.
- `apply_high_bit_flag`: Reads the byte at the address in A0 and the flag byte just before it.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `setup_disk_transfer`: Stores the address in A0 in the disk pointer register and writes the disk sync word.
- `check_status_bit_after_call`: Calls a helper routine, then checks bit one of the status byte at offset 31 from the address in A1.
- `set_global_flags`: Writes two constants to one global variable in sequence, so only the final value 16384 remains.
- `store_ones_if_negative`: Writes 1024 to a global variable.
- `set_bit_in_global_flag`: Reads a 16-bit value 36 bytes below A6, adds 3, and uses its low five bits as a bit index.
- `set_global_flag_byte`: Writes an all-ones byte to a global variable, then calls two other routines.
- `update_indexed_table_entry`: Reads a 16-bit table entry indexed by the value 36 bytes below A6.
- `wait_for_bit_clear_entry`: Polls bit four of a global variable, calling a helper between checks for up to 86 passes.
- `store_masked_flag_byte`: Calls a helper and takes its byte result, clearing bit one of that byte when a second helper output is non-negative.
- `store_call_result_global`: Calls a helper routine and writes its return value to a global variable.
- `compute_masked_flag_byte`: Builds a byte from a global flag value with the bit selected by the low byte of a value 36 bytes below A6 plus three cleared.
- `wait_for_flag_and_result`: Repeatedly calls a helper until it returns one, waiting between calls while bit zero of a global variable is set.
- `conditionally_write_global_sequence`: If bit zero of a global variable is clear and the input value in D0 is not one, it writes the constants 8, 204 and 2 to that global in sequence.
- `write_global_three_constants`: Writes the constants 8, 204 and 2 to a single global variable in sequence, so the final value is 2.

### logged disk load from Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf boot-block load

- `store_self_address`: Loads the address of the next instruction into a pointer register and stores that address into a global variable at offset 32.
- `decode_backward_byte_runs`: Runs a loop that copies counted runs of bytes backward from one buffer to another, with run lengths and branch decisions taken from bit-shifted data words.
- `poll_until_result_zero`: Calls a helper routine repeatedly until the 16-bit value it leaves in D1 becomes zero.
- `shift_bits_with_refill`: Writes the input word to a global variable, then shifts it right by one bit.
- `read_long_before_pointer`: Returns the 32-bit value stored four bytes before the address in A0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_parameters`: Checks two consecutive words in a data table to conditionally write specific values to two global variables, updates a global counter by adding one to a value from the table, and then jumps to a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `disable_interrupts_copy_loader`: Disables interrupts and clears the DMA and interrupt enable registers, then copies a 128-byte block from a fixed source to a fixed destination.
- `install_traps_and_jump`: Sets a trap vector at 28624 and a system call vector at 35628 depending on the values of two global variables, increments a counter at 6010 by one plus a stored value, clears bit 0 of a byte through a register pointer, then jumps to the code at 2048.
- `setup_and_clear_track_data`: Copies two words from a source structure into a destination structure, writes a marker pattern to a byte area, stores a pointer and clears a large region, then calls a helper and stores a final word before returning.
- `copy_seven_bytes_at_stride`: Copies seven bytes from a source area into destination positions spaced 40 bytes apart, using the offsets in a0 and a1, then returns.
- `handle_flag_key_dispatch`: Clears bit 3 of a flag byte at offset 30 of the structure in a6 and returns zero if it was already clear.
- `process_track_data_in_loops`: Loops a helper call once using word values from two tables, then performs a second helper call with a fixed count and a bit value from a status byte before returning.
- `copy_seven_longwords_at_stride`: Copies seven longwords from a source area into destination positions spaced 40 bytes apart, using the offsets in a0 and a1, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `adjust_counter_and_clear_flag`: Clears a flag bit in a byte pointed to by a register using a shift amount from another register, then adjusts two counters based on a timer value and a table entry, clamping them to zero when they underflow or overflow.

### logged disk load from Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf

- `initialize_system_tables_and_hardware`: Copies a block of data from one memory location to another, clears a series of global variables, and performs a memory test by writing and reading bit-flipped values to a specific range.
- `frag_movem_bsr_move_cmp_bne_rts`: Short code fragment that saves registers on the stack, then calls another routine, then copies d0 to d3, then calls another routine, then compares d0 with d3, then branches if the last result was not zero, then compares d0 with 288, then restores saved registers from the stack, then returns.
- `update_vposr_and_interrupts`: Sets the interrupt enable and request registers to 32, calculates a value from the vertical position register by swapping bytes and performing bitwise operations, and updates a global variable with this value if it is greater than the current stored value before setting the interrupt enable register to 32768.
- `copy_word_table`: Copies 16 words from a source table to a destination table by iterating through a loop eight times, swapping bytes of the source words during the process.
- `frag_addq_mulu_moveq_movem_bsr_lea_dbra`: Short code fragment that adds 1 to d0, then multiplies d0 by 40 (unsigned), then loads 3 into d1, then saves registers on the stack, then calls another routine, then restores saved registers from the stack, then loads the address of the memory at an offset from a0 into a0, then loads the address of the memory at an offset from a1 into a1, then loops back while counting down d1, then returns.
- `setup_blt_parameters`: Checks if a range defined by d0, a0, and a1 is valid to set Blitter start and end positions, then configures the Blitter control and DMA registers to perform a transfer.
- `execute_blt_transfer`: Configures the Blitter by setting the source and destination addresses, mode, and control registers, then performs a series of size adjustments and updates a global variable before enabling DMA.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_offset_address`: Calculates an address by multiplying a 16-bit value in d0 by 40, adding a shifted value from d1, and applying bitwise masks.
- `frag_sf_cmpi_beq_st_rts`: Short code fragment that clears d0, then compares a fixed memory address with an address, then branches if the last result was zero, then sets d0 to all ones, then returns.
- `copy_byte_sequence`: Calls another routine, and if the result is zero, copies 20 bytes from a source address to a destination address and decrements a global variable.
- `frag_bsr_tst_bpl_rts`: Short code fragment that calls another routine, then tests d0, then branches if the last result was not negative, then returns.
- `frag_lea_move_clr_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 3999 to d0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `update_string_pointer`: If d0 is non-negative, compares it against a byte at a pointer stored in a global variable; if they match, it increments the pointer, otherwise it resets the pointer to a fixed address or sets it to specific values if d0 matches certain constants.
- `process_input_and_counters`: If a global variable is 8, it clears a byte at an offset, checks joystick and paddle hardware registers to set bits in a status byte, and updates two separate counters based on the difference between current and previous input values, clamping them to specific maximums.
- `update_joystick_state`: Clears a global word, and if a global byte is 20, reads the CIAA and JOY1DAT registers to update a global word with bitwise combinations of the input states.
- `configure_blitter_and_update_velocity`: Configures the Blitter with specific source, destination, and control registers based on a value from another routine, then performs arithmetic operations on a global variable to simulate velocity and position changes.

### logged disk load from Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf

- `update_velocity_and_position_2`: Calculates a deceleration value by multiplying a global 16-bit value by a constant, adds this to a velocity variable, and updates a position variable.
- `store_word_and_branch`: Stores a 16-bit word from register d0 into a global variable and performs a short branch to another routine.
- `update_velocity_and_position`: Decrements a 16-bit value in a global variable by a calculated amount based on its current magnitude, then updates a second global variable by adding a value derived from a multiplication and bit shifts.
- `evaluate_scaling_direction`: Checks a global flag and a specific byte value to determine a direction; it calculates a modified value from a global word by applying an offset and a bitmask, then stores either negative one or one into a global variable based on the result.
- `store_word_and_branch_2`: Stores a 16-bit word from register d0 into a global variable and then performs a short branch to a previous instruction.
- `clamp_scaled_value`: Compares a value from a global variable against a stored offset, then clamps the result between zero and one hundred twenty-eight before updating the stored offset.
- `calculate_scaled_value_2`: Compares register d0 to a value of negative 128; if d0 is positive, it loads a word from a global variable into d1, subtracts 128 from d1, stores d1 in a global variable, extends d0 to a longword, divides d0 by 42, and if the result is greater than 1, sets d0 to 1 and branches to another routine, otherwise it branches to another routine.
- `check_d0_value`: Compares register d0 to a value of 65535; if d0 is positive, it branches to another routine, otherwise it sets d0 to 65535 and branches to another routine.
- `clear_global_word`: Clears register d0 and stores the result into a word at a global memory location, then branches to another routine.
- `update_scaling_and_timer_parameters`: Calculates a non-linear scaling factor by performing a lookup in a table based on a value, then applies this factor and a secondary division-based adjustment to a global variable.
- `reset_game_state_on_condition`: Checks if a register contains 3 or 6 and if a specific global word is negative; if these conditions are met, it stores a pointer into a global variable, clears two 16-bit values in a global structure, and calls another routine.
- `update_timer_and_scaling_parameters_2`: Stores a byte from a pointer into a global variable, subtracts a word from a register, compares the result to 32, and either branches to another routine or sets the register to 3 and branches to another routine.
- `update_timer_and_scaling_parameters`: Updates a byte in a global variable based on the sign of a register, writes a constant to a global variable, and performs a non-linear reduction on a 16-bit value in a global variable using a bitwise shift and subtraction.
- `decrement_and_scale_timer`: Decrements a 16-bit value in a global variable and, if it reaches zero, resets it to 6400 and performs a calculation involving a multiplication by 7 of another global value.
- `update_scaled_index_value`: Checks a global word for a negative value and, if positive, performs a lookup in a table to calculate a new value for a global word.
- `decrement_and_branch_on_value`: Subtracts 5 from a longword in a global variable, clears it if the result is negative, and then performs several conditional branches based on a byte from a global variable and the resulting longword value.
- `compare_word_and_branch`: Compares a word in a register to 80; if the value is less than 80, it branches to a local offset, otherwise it performs an unconditional branch to another routine.
- `update_global_value`: Compares a long value at a global address to 2, decrements a global byte if the comparison fails, and performs arithmetic on a word at a global address using a value shifted right by 2.
- `process_bytecode_table_and_execute`: Increments a global byte and performs arithmetic to update a global value, then iterates through a data structure to decode bytecodes, performing signed arithmetic on values and calling routines via an indirect table based on the decoded bytes.
- `compare_table_entry_and_call`: Calculates an index into a table using a value from a register, compares the resulting table entry against a value from a global variable, and if the comparison meets a specific condition, calls a routine via an indirect pointer from another table.
- `call_indirect_via_table`: Shifts register d4 left by two, loads a pointer from a table using the shifted value, calls the routine at that pointer, and then branches to another location.
- `decrement_scaled_value`: Sets a byte at a global address to one, calls another routine, and then subtracts a bit-shifted version of a word from a global variable.
- `update_table_entry`: Increments a value at a global address, and if it does not equal 11, it calculates an offset, copies two words from a0 to a table, clears a word, copies d0 to the table, and stores a signed multiplication result in the table.
- `process_masked_byte`: Loads a byte from a global variable into d2 and d3, masks d2 with 15 and d3 with 240, then branches to another routine if d2 is zero; otherwise, it checks a second global variable and, if that value is negative, negates d3 and subtracts 96 from it before branching to another routine.
- `calculate_masked_value`: Loads a word from a global variable, masks it with a constant, and if the result is not zero, checks a second global variable; if that variable is not negative, it adds a constant to the masked value before branching.
- `conditional_set_d3`: Clears register d3, tests bit 0 of register d2, and if the bit is not set, it sets d3 to 32 before returning; otherwise it branches to another instruction.
- `calculate_scaled_offset_and_decrement`: Performs a series of multiplications and shifts on a 16-bit value from a global variable to calculate a new value, which is then stored in another global variable.
- `update_offset_value`: Decrements a byte in a global variable and branches to another routine if it is less than or equal to zero; otherwise, it compares a word in a global variable to 100, and if it is not less than 100, it modifies d7 by subtracting 4096 or adding 8192 based on its sign, then stores d7 in a global variable and branches to another routine.
- `clear_byte_and_branch`: Clears a byte at a global address and branches to another location.
- `calculate_and_reset_object_index`: Calculates a new index based on a scaled value from a global variable and, if the result is negative, clears several global variables and resets specific memory locations to 160 before branching to another routine.
- `initialize_table_pointer`: Clears registers d0, d4, d6, and d7, loads a word from a global variable into d5, shifts d5 right by 2, and loads the address of a global variable into a6 before branching to another routine.
- `fill_division_table_bytes`: Calls two helpers, then walks a table of 8-byte entries until it reaches a negative entry.
- `store_division_table_values`: Walks a table of 8-byte entries from the address in A0 until it reaches a negative entry, without calling any helper.
- `convert_bcd_loop`: If a global variable is zero and a counter in d2 is non-negative, it breaks a 32-bit value into bytes and uses BCD arithmetic to add them to a table in a loop.
- `add_bcd_long_integer`: Checks if a global variable is zero and a counter is positive, then performs a multi-byte BCD addition of a 32-bit value in d0 to a 4-byte BCD value in memory, repeating the addition for the specified number of iterations.
- `clear_memory_block`: Clears 191 words starting at the address held in a6 using a loop.
- `update_target_object_data`: Iterates through a table of structures to find the entry with the largest value at offset 8, then copies data from a source structure into that selected entry.
- `update_object_properties`: Iterates through a table of structures to find the one with the largest value at offset 8, then copies data from a source structure into that structure.
- `branch_on_structure_byte`: Pushes the stack pointer to the stack, reads a byte from the structure in a6, and if that byte is equal to three, calls another routine; otherwise it returns.
- `iterate_structure_fields_with_subroutines`: Calls five different subroutines in sequence, compares a 16-bit word at offset 8 of the structure in a6 to 500, and then executes a loop that calls another routine a variable number of times while incrementing the pointer in a6 by 32 bytes each iteration.
- `check_structure_status`: Checks a byte at offset 30 of a structure in a6; if it is zero, it verifies that a byte at offset 0 is 1 and a byte at offset 13 is negative, then clears a word at offset 12, checks bit 0 of offset 0, and either sets a global variable and offset 30 to 1 or branches to another location.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `increment_and_compare_structure`: Increments a byte at offset 30 of a structure in a6, loads a word from a global variable into d0, compares a word at offset 20 of a6 with 165, and branches based on the result.
- `update_object_state_and_queue_event`: Checks a type byte at offset 28 of a structure and, depending on its value, performs arithmetic on a byte at offset 29 using a value derived from a global variable or a table, then calls another routine and, if successful, populates a new structure in a0 with specific parameters and stores it in a queue.
- `update_scaled_data`: Compares d0 to 2, and if equal, checks if a word at offset 6 of a6 is less than 301; it then performs arithmetic on a byte at offset 29 using a value from a global variable, calls another routine, and modifies a structure in a0 based on bit tests of a word at offset 4 of a6.
- `calculate_and_store_structure_data`: Compares a word at offset 8 of a6 with 250, performs arithmetic on a byte in d0 using a byte at offset 13 of a6, updates a byte at offset 29 of a6, calls another routine, and if the result is non-negative, stores several words into a structure pointed to by a0.
- `zero_table_entries`: Clears five 16-bit words at 6-byte intervals starting from a global table address using a loop.
- `update_table_entry_states`: Iterates through a table of four structures, performing conditional arithmetic on various fields based on a state value at offset zero.
- `calculate_and_store_value`: Negates a word from offset 8 of a structure in a0, adds 36, divides it by 20, adds 69, stores the result in offset a of the structure in a0, stores d1 in offset 4 of the structure in a0, and branches to another routine.
- `modify_structure_field`: Decrements a word in d1 and branches if not zero, otherwise it checks a bit in a global long value and conditionally increments a word at an offset of a structure in a0, potentially clearing it if it reaches 20.
- `decrement_and_modify_values`: Decrements d1 and branches if not zero; otherwise it compares a word at offset 4 of a structure in a0 to 20, and if it is below 20, subtracts 5 from that word; if not, it clears the word at offset 4, compares a byte at offset 6 of the structure in a0 to 47, and if it is not below 47, subtracts 25 from it before branching to another routine.
- `update_table_entry_2`: Subtracts a value derived from a global variable and a bit shift from a word at offset 8 of a structure in a0, then conditionally clears several fields in that structure and updates a pointer in a global variable with values from the structure.
- `process_table_entries`: Pushes a0 and d0 onto the stack, clears d4, loads words from a0 into d5, d0, and d7, sets a6 to a specific address, calls another routine, and then iterates through a list using d0 as a counter.
- `search_table_for_zero`: Iterates through a table at 6-byte intervals, checking if a word is zero, and returns if a zero is found or after five iterations.
- `update_structure_with_global_value`: Loads a word from a global variable, shifts it left by 6, and if a word at offset 2 of a6 is zero, updates offset 4 of a6 with the negative of a value from a global variable, updates offset 6 of a6, and modifies offset 4 of a structure using a pointer in d0.
- `update_structure_values`: Compares values within a structure in a6 against constants, updates a global variable, modifies structure offsets, and clears specific memory locations based on conditional checks.
- `check_structure_and_clear_register`: Compares a byte from a structure in a6 with 18 and a byte at offset 1 of the same structure with 2, then clears d0 and branches if a global byte matches 2; otherwise it branches to another location.
- `update_object_physics_and_position`: Increments a byte at offset 31 of a structure in a6 and, if it is less than 64, copies several fields from a global structure to the current structure.
- `add_to_structure_field`: Adds the value in d3 to a word at offset 4 of a structure in a6 and branches to another routine.
- `process_scaled_data`: Calls another routine, then checks if a bitwise AND of d0 and a byte from offset 23 of a structure in a6 is non-zero; if it is zero, it sets d1 to 1 or -1 based on bit 8 of d0 and adds d1 to a word at offset 14 of the structure in a6; otherwise, it performs several comparisons and arithmetic operations involving a pointer in a4 and a word at offset 8 of the structure in a6.
- `adjust_scaled_value`: Performs arithmetic on a word at offset 14 of a structure in a6 based on bytes from a4 and the sign of d3, then conditionally updates that word or sets it to a specific value.
- `calculate_scaled_offset`: Tests the sign of d3, and if negative, performs arithmetic on d6 and compares it to a word at offset 4 of a6; it then performs a multiplication of d3 and d0 based on comparisons with d5 and d4.
- `decrement_and_branch`: Tests the sign bit of d3, and if it is negative, decrements a word in d5 and branches to another location.
- `compare_and_write_field`: Writes a word from d0 to offset 14 of a6, tests that value, and if it is non-zero, compares a word at offset 8 of a6 with a value at a pointer in a3, potentially adding 4 to that pointer.
- `modify_structure_fields`: Checks a byte at offset 2 of a structure in a3; if it is 4, it writes a signed 16-bit value derived from a field in a6 to offset 14 of the structure in a6, then updates offset 18 of the structure in a6 based on a bit test in a4.
- `decrement_structure_field`: Compares a byte at a global address to 2, and if it is not equal, branches; otherwise, it checks bit 0 of offset 0 of a6 and either subtracts 8 from a word at offset 2 of a6 or clears it.
- `calculate_scaled_value`: Checks a byte at offset 48 of a6 and a word at offset 20 of a6, performs arithmetic involving a global variable and a field at offset 16 of a6, and if a bit is set in offset 34 of a6, updates a global variable and performs a multiplication and division to store a result in offset 2 of a6.
- `process_complex_math_and_table`: Performs a series of multiplications and shifts on values from a structure and global variables, calls another routine, and if a specific condition is met, copies several fields from a structure in a6 into a new structure pointed to by a global variable.
- `evaluate_and_call_subroutine`: Compares a word at offset 8 of a structure in a6 to 500, and if it is 500, increments it; it then performs several conditional checks on values at offsets 0, 4, 6, and 12 of the structure in a6, potentially modifying d4, d6, or d7, and calls another routine with a modified a6 and d7 value.
- `call_subroutine_with_offset`: Compares a word at offset 12 of a structure in a6 to specific constants, sets a value in d1, and calls another routine after adding 10 to a6.
- `adjust_a6_and_call`: Tests a byte at offset 30 of a6, and if it is non-zero, adds 10 to a6, calls another routine, subtracts 10 from a6, and returns.
- `render_clipped_sprite_layers`: Checks a global pointer for a specific sprite type and, if matched, calculates dimensions and modulo offsets to call setup_clipped_blit_modulos.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `compare_and_fetch_table_value`: Compares swapped words from two global longwords against specific values, and if they match, loads a word from a table at an offset determined by a value from a third global longword.
- `calculate_scaled_values`: Calculates a quotient by dividing 20480 by the sum of d7 and 40, stores the result at offset 4 of a structure in a6, stores a bit-shifted version of the extended quotient at offset 18 of the structure, and increments a pointer in a4 if a comparison with a word at offset 4 of a4 fails.
- `extract_and_clamp_byte`: Extracts a byte from offset 2 of a pointer in a4 and masks it, then checks if that byte is zero; if not, it clears a word at offset 8 of a structure in a6, compares d5 after subtracting a word from the pointer in a4 to 20, and if it is below 20, sets d5 to 32 before branching to another routine.
- `calculate_division_result`: Multiplies d5 by 32, divides the result by 20, moves d4 to d0, multiplies d0 by 32, subtracts d5 from d0, stores d5 in d2, and branches to another location.
- `clamp_d5_value`: Subtracts a word from a memory location pointed to by a4 from d5, and if the result is not less than 36, sets d5 to 32; otherwise it branches.
- `calculate_addition_result`: Multiplies d5 by 32, divides the result by 20, moves d4 to d0, multiplies d0 by 32, adds d5 to d0, stores d5 in d2, and branches to another location.
- `shift_d4_and_clear`: Moves d4 to d0, shifts d0 left by 5 bits, clears d4, and branches to another location.
- `store_scaled_values`: Shifts d4 left by 5, stores d4 in offset 8 of a6, stores the shifted result in offset 6 of a6, stores d2 in offsets 22 and 24 of a6, and increments a6 by 26.
- `update_pointer_tables`: Swaps the words of two 32-bit values from global variables and stores them in new locations, then stores a 16-bit value from a global variable into another location.
- `process_table_data`: Loads three addresses into a5, a4, and a6, then if a value from a global variable is not zero, it decrements that value and performs arithmetic and memory access using a word from offset 4 of a6 and a table at a5.
- `loop_table_search`: Performs a division of 32768 by a value derived from d7, performs an OR operation on a word at offset 8 of a6, and then enters a loop that compares a value to a table in a4 and increments the pointer.
- `loop_and_search_table`: Manipulates bits in a word at offset 2 of a6, performs arithmetic using a byte from a table in a4, calls a routine to search a sorted table, and repeats this process in a loop controlled by d3.
- `interpolate_interpolated_value_list`: Iterates through a list of structures to perform linear interpolation on values, updating a global variable and a list of interpolated coordinates by calculating scaled offsets and applying changes to structure offsets 2 and 6.
- `loop_interpolated_values`: Compares a word at offset 4 of a structure in a1 to d5, and if it is not less than zero, stores d1 at offset 6 of the structure, stores d1 at the address in offset 14 of the structure, and loops.
- `update_interpolated_value_list`: Checks if a pointer in a2 and a 16-bit value at offset 4 of a structure in a1 are both equal to 65535; if not, it increments d4, decrements d0 in a loop, and performs arithmetic on d1 using values from the stack and a call to another routine before branching.
- `add_to_registers`: Adds d0 to d5 and d0 to d4, then branches to an earlier instruction in this routine.
- `interpolate_coordinate_values`: Calculates a new 16-bit value by adding a register to a base value, then performs a series of arithmetic operations including a division via another routine, and stores the result at offset 2 of a structure pointed to by a2 before incrementing the pointer by 26.
- `loop_interpolated_values_copy`: Compares a word at offset 4 of a structure in a1 to d5, and if it is not less than zero, stores d1 at offset 6 of the structure, stores d1 at the address in offset 14 of the structure, and loops.
- `process_interpolated_coordinate_delta`: Checks if a pointer in a2 or a value at offset 4 of a1 is equal to 65535, otherwise decrements d4 and d0 in a loop, pops a value from the stack into d6, calls another routine, and performs arithmetic involving negation and addition to update d6 before branching.
- `adjust_registers_and_loop`: Adds d0 to d5, subtracts d0 from d4, and branches to an earlier instruction in this routine.
- `interpolate_and_update_structure`: Calculates a scaled value using division and multiplication, then updates a structure by storing the result at offset 2 and offset 6 of a pointer in a2, while conditionally updating a second structure's fields based on a comparison with a word at offset 4.
- `update_structure_fields`: Stores d1 at offset 6 of a structure in a1, stores d1 at the address in offset 14 of a1, and if a word at offset 4 of a1 matches a word at the address in a2, stores d1 at offset 2 of a2 and branches.
- `compare_and_update_structure_field`: Compares a 16-bit value in register d7 with a 16-bit value at offset 4 of a structure pointed to by a1; if they match, it stores the 16-bit value from d1 into offset 6 of the structure, increments the pointer in a1 by 18, and branches to another routine, otherwise it branches to another routine.
- `test_and_rotate_value`: Tests the sign bit of d7, loads a word from a pointer in a2 into d6, compares it with a word at offset 4 of a1, and if the comparison passes, rotates a longword in d1 and branches based on a comparison with 65535.
- `calculate_scaled_offset_and_branch`: Performs a multiplication and addition to derive a value, subtracts a value from a pointer, and shifts the result right by 8 bits before adding it to two values retrieved from the stack.
- `tiny_addq_addq_rts`: Small routine of 3 instructions that adds 2 to a7, then adds 2 to a7, then returns.
- `calculate_and_store_division`: Increments the stack pointer, writes a 16-bit value of all ones to a location pointed to by a2, performs a division on a value derived from a1, stores the result in a global variable, and branches to another routine.
- `calculate_squared_difference`: Squares the word in d4, subtracts the value in a4 from the result, and if the result is non-zero and within a specific range, enters a loop that shifts the value left by 2 bits while checking bits 23 through 25.
- `process_indexed_value`: Swaps the high and low words of a value from a6, adds the result to itself, uses it as an index to read a word from a table, and performs bitwise operations on the result before returning.
- `tiny_moveq_add_rts`: Small routine of 3 instructions that loads 255 into d1, then adds d1 to d1, then returns.
- `fill_division_table_words`: Fills 256 words at descending addresses from a fixed start, with each word holding 20480 divided by a divisor that runs from 295 down to 40.
- `build_accumulator_table`: Builds a table in memory by repeatedly updating two accumulators, storing a running value at a stride of 13 words and stopping at a fixed end address.
- `continue_accumulator_table_build`: Continues a table build from a mid-point entry, advancing the accumulators and storing records at a 13-word stride until the end address is reached.
- `step_accumulator_table_build`: Stores one record in the table, then advances the accumulator until an addition carries, calling a helper on each pass.
- `finish_accumulator_table_build`: Writes a table record when a condition is met, then advances the accumulator until an addition carries, calling a helper on each pass.
- `advance_accumulator_until_carry`: Advances a running value by a quadratic-style update until an addition carries, calling a helper on each pass.
- `store_sentinel_and_globals`: Writes an all-ones sentinel word through the pointer in A6 and derives two global values from that pointer's position.
- `divide_offset_by_constant`: Compares the word in d7 to 256; if it is less than 256, it adds 40 to d7, divides 20480 by this new value, and branches.
- `get_indexed_decrement`: Doubles the value in d7, uses it as an index to read a word from a table pointed to by a5, and subtracts 1 from that word.
- `scan_sorted_table_for_value`: When the value in D7 is below 1000, it walks a table of 3-word entries until it reaches an entry greater than that value.
- `return_stack_word_pair`: Returns two 16-bit values read from stack argument slots as one 64-bit result.
- `frag_neg_asr_bra`: Short code fragment that negates d1, then shifts d6 right by d1, then jumps to another point in the code.
- `frag_lea_moveq_move_tst_bmi_asl_bra`: Short code fragment that loads the address of a fixed memory address into a0, then loads 255 into d6, then copies an indexed entry from a0 to d6, then tests d1, then branches if the last result was negative, then shifts d6 left by d1, then jumps to another point in the code.
- `calculate_clamped_ratio`: Performs arithmetic on d1 and d6 using values from global variables, compares d7 to 256, and if d7 is greater than or equal to 256, performs a division and compares the result to 18 to determine a branch.
- `frag_add_move_lsr_subq_muls_asr_andi_br`: Short code fragment that adds d7 to d7, then copies an indexed entry from a5 to d0, then shifts d7 right by 1, then subtracts 1 from d0, then multiplies d6 by d0 (signed), then shifts d6 right by 8, then shifts d6 right by 1, then adds a fixed memory address to d6, then masks the status register with 65534, then jumps to another point in the code.
- `frag_ori_movem_rts`: Short code fragment that merges 1 into the status register, then restores saved registers from the stack, then returns.
- `iterate_table_data`: Loads a pointer from a memory location, compares a word at offset 4 of that pointer to d0, and if the comparison is not below zero, enters a loop that iterates 68 times while incrementing the pointer by 18.
- `copy_and_update_structure`: Increments a global variable, calculates an offset based on d2, copies a series of words from a table to a destination pointed to by a2 using a loop, and then writes several registers into a structure pointed to by a0.
- `clear_and_copy_init_tables`: Sets a series of global variables to constants and clears several groups of memory in a loop.
- `set_mask_words_from_bits`: Clears two longwords at the pointer in A1, then sets each of four 16-bit slots to all ones when the matching low bit of D0 is set.
- `merge_bits_into_words`: Shifts the byte in d7 right three times and, for each zero bit produced, merges the word in d6 into d2 and then d3.
- `frag_move_mulu_addi_rts`: Short code fragment that copies a fixed memory address to d0, then multiplies d0 by 171 (unsigned), then adds 3847 to d0, then copies d0 to a fixed memory address, then returns.
- `update_clamped_value`: Modifies a global variable by adding a value derived from a table lookup and then clamps the result between two constant bounds.
- `interpret_command_stream`: Walks a stream of 16-bit commands, using the high byte of each word as an opcode.
- `update_damped_velocity_state`: Updates a set of 16-bit state variables each pass by adding a velocity term scaled by a global factor, with damping and clamping.
- `store_chained_result_if_nonzero`: Stores the result of a helper in a global variable and, when that result is non-zero, calls a second helper and stores its result there too.
- `or_block_bytes_when_zero`: Calls a helper and, when its result is zero, ORs together the 512 bytes starting at a fixed address and returns the combined byte.
- `copy_indexed_table_segment`: Uses an index to look up a 32-bit table entry, then splits its value by dividing by 5632 and multiplying by 11 to select a source segment.
- `setup_registers_and_pointers`: Clears d0 and d1, sets d2 to 1 and d3 to 32768, then loads the addresses of two tables into a0 and a1, after which execution falls through to the following routine.
- `split_by_eleven_and_consume`: Splits the input word into a quotient and remainder when divided by 11, then calls a helper repeatedly to consume a counted number of steps.
- `write_track_gap_and_sync`: Clears three counters on each of five passes, calling helper routines that check the disk-status bit.
- `retry_disk_read_transfer`: Repeatedly starts a disk read for up to five attempts, storing the buffer address in the disk pointer register and setting the disk sync word before each.
- `validate_read_and_store_count`: Calls helpers up to eleven times until a read check matches the expected value and the counters are in range.
- `scan_track_sectors_for_match`: Scans up to ten sector positions by calling helper routines and checking a status bit at each one.
- `frag_move_sub_mulu_rts_2`: Short code fragment that copies d3 to d1, then subtracts the memory at an offset from a6 from d1, then copies 512 to d0, then multiplies d1 by d0 (unsigned), then returns.
- `frag_move_bset_addq_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then sets bit d3 of d0, then copies d0 to the memory at an offset from a6, then adds 1 to the memory at an offset from a6, then returns.
- `clear_strided_long_words`: Zeroes ten long words spaced at a fixed stride of 272 bytes, starting from the address held in the A5 register.
- `forward_call_repack_result`: Calls another routine and repacks its 32-bit result, keeping the low word and moving the top byte into a second returned word.
- `merge_odd_even_bit_words`: Combines the two consecutive longwords at a pointer by keeping the alternate bits of each and shifting the first left by one, so the two sets interleave, and returns the result.
- `forward_result_to_routine`: Calls one routine and passes its result on to a second routine, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `xor_checksum_counted_longwords`: XORs a block of longwords whose count is derived from a size register shifted right by two, then returns the result masked to alternate bits.
- `fill_block_with_alternating_pattern`: Fills a block of 256 longwords, counting down from its end, with a constant alternating-bit pattern.
- `set_structure_bounds_fields`: Computes start and end values from one routine's result and an argument, then stores them with a constant word and a shifted value into fixed fields of a structure.
- `program_blitter_block_passes`: Programs the blitter for a block in several passes, writing pointer, modulo and size values for each pass from computed word counts, and returns after the final pass.
- `wait_while_global_bit_set`: Loops while bit six of a global variable is set, doing nothing else, then returns.
- `reset_blitter_masks_and_modulo`: Waits for the blitter, then sets its control and mask registers to fixed values and clears the modulo registers.
- `frag_move_lsr_bsr`: Short code fragment that copies d0 to the stack, then shifts d0 right by 1, then calls bitwise_mix_store_word_2, then copies the top of the stack to d0, then calls another routine.
- `set_high_bit_from_flag`: Reads a byte through a pointer and checks the low bit of the byte before it.
- `bitwise_mix_store_word_2`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `setup_disk_sync_dma`: Writes the disk sync pattern, the disk pointer and a length value, then enables the disk DMA and interrupt bits to start a transfer.
- `wait_for_disk_status_bit`: Waits on a status byte until a flag bit is set, then writes two disk control values and returns zero.
- `frag_move_tst_rts`: Short code fragment that copies 2 to a fixed memory address, then copies 16384 to a fixed memory address, then tests d0, then returns.
- `frag_move_tst_bpl_moveq`: Short code fragment that copies 1024 to a fixed memory address, then tests the memory at an offset from a6, then branches if the last result was not negative, then loads 255 into d1.
- `update_ciab_port_bit`: Writes a value to the CIAB port B register with one bit set or cleared, using a bit index read from the stack.
- `set_ciab_port_all_bits`: Writes all ones to the CIAB port B register, then calls two other routines and returns.
- `step_stored_value_toward_target`: Reads a stored 16-bit value from a table indexed from the stack, moves it toward a new value by calling a routine once per step of the difference, then stores the new value and returns zero.
- `poll_port_bit_with_timeout`: Polls a bit of the CIAA port register for up to 85 attempts, calling a routine between polls.
- `update_ciab_control_bits`: Sets or clears bits in the CIAB port register according to the sign of an argument, then calls a routine to finish.
- `frag_bsr_move_rts`: Short code fragment that calls another routine, then copies d0 to a fixed memory address, then returns.
- `compute_ciab_bit_mask`: Calculates a bit mask for a bit index read from the stack, combines it with the current port value, clears one bit when a stored flag is set, and returns the result with the input word.
- `poll_cia_status_until_ready`: Repeatedly calls a routine and waits while a CIA status bit stays set, looping until the routine returns one, then returns zero.
- `frag_btst_bne_subq_beq`: Short code fragment that tests bit 0 of a fixed memory address, then branches if the last result was not zero, then subtracts 1 from d0, then branches if the last result was zero.
- `frag_move_rts`: Short code fragment that copies 8 to a fixed memory address, then copies 204 to a fixed memory address, then copies 2 to a fixed memory address, then returns.
- `setup_clipped_blit_modulos`: Looks up a record from a table index, then runs a clip test on two coordinate values.
- `setup_clipped_blit_modulos_2`: Repeats the clip test and blitter-register setup of a neighbouring routine, decoded from an unreached gap.

### logged disk load from Chase H.Q. II - Special Criminal Investigation (1990)(Ocean)[cr SKR - Valhalla][t +3 SKR](Disk 1 of 2).adf

- `build_bit_reversal_table`: Fills a 256-byte table so that each entry holds its index with the bits in reverse order.
- `copy_and_process_data`: Copies 16 words from a source to a destination starting at an offset of 7960, then processes a sequence of bytes by expanding signed bytes into words and copying them to a new location based on a count derived from the byte value.
- `initialize_and_decrement_pointer_loop_2`: Clears a 16-bit register, sets a pointer to an offset of 7960 from a memory location, increments a counter, and enters a loop that decrements.
- `copy_and_loop_bytes`: Copies a byte from the address in a0 to the address in a1, increments d3 by 1, and if d3 is 40, branches; otherwise, it loops using d1 as a counter and eventually branches to another routine.
- `initialize_and_decrement_pointer_loop`: Clears a 16-bit register, sets a pointer to an offset of 7960 from a memory location, increments a counter, and enters a loop that decrements the pointer by 3196 and decrements a loop counter until it reaches zero, otherwise it returns.
- `sort_byte_pairs_in_list`: Walks a list of byte pairs ended by an all-ones byte and swaps adjacent entries that are out of order.
- `allocate_or_compact_blocks`: Manages a table of 219 ten-byte block descriptors, allocating a block from free space and moving other blocks with 32-byte copy loops when space is short.
- `move_block_and_adjust_table`: Moves a block of memory in 32-byte steps, then reduces the descriptor pointers that lie above the moved block by its length.
- `set_high_bit_in_records`: If the table entry for the index is non-zero, it sets the top bit of the first byte of each eight-byte record in its list, stopping at an all-ones marker byte.
- `transform_bitplane_rows`: Builds a 16-entry lookup table, then rearranges bits across four 16-bit words per row using that table, a bit-plane transform.
- `visit_type_four_records`: Walks 220 ten-byte records from the highest index downward and updates two globals from index-based lookups for each one.
- `process_selected_record_entries`: For a non-empty record selected by a global index, it either runs a loop of helper calls over the record's entries, or for index two copies a block whose length is the product of two record fields times ten.
- `copy_product_sized_block`: Copies a block of bytes from the global pointer to the address in A0, with the length equal to the product of the words in D6 and D7 multiplied by ten.
- `split_rows_into_bitplanes`: Copies words from source rows into four separate planes, storing the first plane inverted and the other three unchanged.
- `transform_indexed_bytes`: Reads a byte from a table indexed by d2, rotates and swaps it, then repeats the same three-step transform on a byte indexed by d3 before returning.
- `swap_and_fill_tables`: Swaps two 32-bit pointers stored in global variables, calls another routine, calls a routine to fill two tables, and sets a global variable to 1.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `set_copper_list_and_dma`: Writes copper list start addresses to the first and second copper location registers and triggers a copper jump.
- `advance_state_flag_machine`: Checks a state flag byte: when bit five is set it advances a counter, calls helpers and resets counters; when bit six is set it clears the state and sets bit six; otherwise it checks bit four.
- `translate_keycode_into_buffer`: Translates a keyboard code through a lookup table indexed by the inverted code, appending the character to a buffer when the code's low bit is clear.
- `fill_two_tables_stepped_values`: Writes a 32-bit value, split into high and low words, into four pairs of slots in two tables, adding 8000 to the value on each pass.
- `update_color_tables`: Loops 16 times to read words from a table, performs arithmetic on each word, and stores the results into both the hardware color registers and a global table.
- `clamp_shifted_value`: Reads a 32-bit value from a global variable, shifts it right by 2, adds 32, and stores the result in a global variable after clamping it to a maximum of 63.
- `clear_global_repeatedly`: Stores zero into a global variable seventeen times in a straight-line sequence, with no loop or calls.
- `init_globals_return_address`: Clears several global variables and stores address constants into others, then returns a fixed 32-bit address.
- `set_handler_address_table`: Clears two globals and stores four address constants into a table of handler slots, then returns.
- `set_handler_address_table_2`: The same handler-address setup as a neighbouring routine, decoded from an unreached gap.
- `reset_table_values`: Sets a byte at a global address to zero and updates four global longwords with specific constant values.
- `initialize_table_values_alt`: Sets a byte at a global address to zero, updates four global longwords with different constant values, and then performs a second sequence of setting a byte to zero and updating three global longwords and one more longword with different constants.
- `store_handler_address_after_call`: Calls a helper routine, then stores a fixed address constant in a handler slot.
- `frag_move_rts_2`: Short code fragment that copies 470 to a fixed memory address, then copies 1 to a fixed memory address, then returns.
- `update_audio_channel_state`: Updates four audio channels each pass, writing their period, volume and length registers from per-channel tables and writing the DMA control register.
- `set_global_value_fifteen`: Writes 15 to a global variable and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_store_d0`: Adds one to the value passed in d0, clears its least significant bit, and stores the resulting 32-bit word into a global variable at offset 21540, then loads zero into d0 and 1001 into d1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fall_through_entry_2`: Entry point part-way through straight-line code: runs three instructions (sf.b, ori.b, ori.b) and then falls through into the code that follows, with no return of its own.
- `branch_if_equal`: If the zero flag is set, it branches to another location; otherwise it returns.
- `store_byte_if_not_zero`: Tests a byte in register d0, and if it is not zero, stores it in a memory location before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_zero_d0`: Subtracts 1 from register d0 and performs four logical OR operations with 0 on register d0.
- `setup_stack_and_zero_d0`: Loads constants into registers d1, d6, and d7, pushes the value of d1 onto the stack, and performs a logical OR with 0 on register d0.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditional_zero_d0`: If the zero flag is not set, it performs a series of logical OR operations on register d0 with 0 and 10, otherwise it branches to an earlier instruction.
- `branch_if_not_equal`: Performs a conditional branch if the zero flag is not set; otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditional_return_d0`: If the zero flag is not set, it returns; otherwise, it performs a logical OR with 0 on register d0 and returns.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs three instructions (add.w, ori.b, ori.b) and then falls through into the code that follows, with no return of its own.
- `process_data_blocks`: Reads a word from a pointer in a0, performs several conditional checks and register modifications, and uses a loop to move data between a1 and a2.
- `copy_byte_and_long`: Increments a0 by 2, loads a word from the new a0 into d2 and d1, loads a long from the new a0 into a2, writes the byte from d2 to the address in a2, and copies the long from a2 to the address in a1.
