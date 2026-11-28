# Game 1493 (distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01493). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (860608 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 47 of 47 named (0 author symbols), 47 of 47 described (11 by rule, 26 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 102 of 102 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 964 of 964 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1685 |
| code | 5006 |
| data | 847771 |
| padding | 2 |
| unused | 6144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf:bootblock | 1 | 40 | 2 |
| logged disk load from Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf boot-block load | 1 | 129 | 6 |
| logged disk load from Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf boot-block load | 1 | 469 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 375 | 23 |

## Cases handled

- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: custom-format (non-DOS) disk, 1658 non-empty blocks: matched against track-loaded segments
- 102 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: the boot block reads 2048 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- runtime segment an address: 102 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: 2 logged loads cover 3072 bytes of the image
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: 8 track-loaded blocks found in recovered runtime segments and classified through them
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: blocks 4-21 (bytes 2048-11264) hold ASCII text
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: blocks 33-341 (bytes 16896-175104) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.54)
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: blocks 343-536 (bytes 175616-274944) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.25)
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: blocks 539-549 (bytes 275968-281600) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.41)
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: blocks 556-569 (bytes 284672-291840) outside the filesystem hold data, not code (opcode density 0.044, entropy 5.42)
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: blocks 571-640 (bytes 292352-328192) outside the filesystem hold data, not code (opcode density 0.048, entropy 6.31)
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: blocks 644-1671 (bytes 329728-856064) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.76)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_hardware_registers` (Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf boot-block load +an address to an address (2048 bytes)): Clears a register, writes two bytes to a port, and sets several hardware registers including DSKLEN and DMACON before returning. Kind: disk hardware/sync check. Pass path: the instruction following the bsr.w an address call (st.b an address.l).
- `set_hardware_parameters` (Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf boot-block load +an address to an address (2048 bytes)): Clears a register, writes four bytes to a port, and sets a hardware register before returning. Kind: disk hardware/geometry check. Pass path: bra.w an address.
- `update_hardware_and_data` (Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf boot-block load +an address to an address (2048 bytes)): Writes values to several hardware registers including custom chip and CIAA registers, then enters a loop that performs bitwise operations and stores results into a memory location pointed to by a calculated address. Kind: disk hardware/parameter validation. Pass path: bpl.b an address.
- `write_port_and_call` (Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf boot-block load +an address to an address (2048 bytes)): Writes two bytes to a port and calls another routine before returning. Kind: disk hardware/track check. Pass path: bra.b an address.
- `write_port_and_call_alt` (Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf boot-block load +an address to an address (2048 bytes)): Writes two bytes to a port and calls another routine before returning. Kind: disk/track check. Pass path: bra.b an address.

## Functions


### Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf:bootblock

- `allocate_memory_and_execute_io`: Calls a routine to allocate memory, calls a routine at a fixed address, performs memory allocation via a library call, checks a value, and if it meets certain criteria, writes a value to a global variable and calls a routine via a trap.
- `initialize_hardware_and_jump`: Disables interrupts, sets several hardware registers including BPLCON0 and DMACON, updates the user stack pointer, and jumps to a fixed address.

### logged disk load from Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf boot-block load

- `initialize_chess_copper_list`: Initializes a copper list for chess display by setting up color registers and wait instructions, then branches to a loop that configures additional copper commands based on a counter value.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `append_copper_color_wait_chess`: Writes d0, a copper wait longword, and d1 through a2, advances the pointer, adds 256 to d0, and returns for the chess loader.
- `append_color_register_pair_chess`: Calls the chess wait-entry writer, appends two colour-register words with d3, lowers d3 by 273, raises d0 by 512, and returns.
- `call_builder_adjust_d2_a2_chess`: Calls the adjacent chess builder, adds 384 to d2, subtracts d4 from a2, and falls through into the next code.
- `invoke_two_os_operations_chess`: Passes a3, d2 and d3 to one library call, passes a2 and d4 to another, advances a2 by d4, and returns.

### logged disk load from Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf boot-block load

- `initialize_graphics_hardware`: Sets up multiple Amiga custom chipset registers including bitplane control, display window parameters, copper list pointers, and sprite configurations.
- `compare_and_call_subroutine`: Calls two different routines, then enters a loop to compare seven bytes from two memory locations, otherwise it returns with a negative value in d0.
- `process_data_structure`: Loads a pointer from a global variable and enters a loop to iterate through a data structure, calling another routine or a second routine depending on specific conditions and bit tests.
- `configure_hardware_registers`: Clears a register, writes two bytes to a port, and sets several hardware registers including DSKLEN and DMACON before returning.
- `set_hardware_parameters`: Clears a register, writes four bytes to a port, and sets a hardware register before returning.
- `copy_memory_block`: Calls another routine and, if a value in d0 is not positive, enters a loop to copy bytes from a source address to a destination address using multiplication to calculate offsets.
- `update_hardware_and_data`: Writes values to several hardware registers including custom chip and CIAA registers, then enters a loop that performs bitwise operations and stores results into a memory location pointed to by a calculated address.
- `process_byte_loop`: Loads a word from a memory location, checks bit 0 of d2, and enters a loop that stores a byte at a specific address before returning.
- `process_counter_and_input`: Tests a word at a pointer, calls various routines based on bit tests in a hardware register, and contains two loops that decrement a value in d1 using the dbra instruction.
- `check_hardware_status`: Checks bit 4 of the CIAA_PRA register and, if not set, calls one of four different routines before returning; otherwise it calls another routine and returns.
- `test_button_bit`: Tests bit 2 of a hardware register and branches to another routine if the bit is not set.
- `write_port_and_call`: Writes two bytes to a port and calls another routine before returning.
- `test_button_bit_branch`: Tests bit 2 of a hardware register and branches to another routine if the bit is not set.
- `write_port_and_call_alt`: Writes two bytes to a port and calls another routine before returning.
- `check_interrupt_status`: Writes a value to a register, checks a bit in another register, and either returns or writes the same value to the first register again.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_tst_bne_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_btst_bne_rte`: Short code fragment that tests bit 4 of a fixed memory address, then branches if the last result was not zero, then tests bit 5 of a fixed memory address, then branches if the last result was not zero, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then returns from the exception.
- `wait_for_interrupt_flag`: Polls the interrupt request register until one bit is set, then writes the same bit back to acknowledge it.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address (2 times in a row), then copies 4178 to a fixed memory address, then copies 252 to a fixed memory address, then copies 13 to a fixed memory address, then copies 1 to a fixed memory address, then copies 130 to a fixed memory address, then copies 32776 to a fixed memory address, then returns.
- `frag_clr_move_rts_2`: Short code fragment that clears a fixed memory address, then copies 50332 to d0, then copies d0 to a fixed memory address (2 times in a row), then copies 12809 to a fixed memory address, then copies 9 to a fixed memory address, then copies 2 to a fixed memory address, then clears a fixed memory address (2 times in a row), then returns.
- `process_input_interrupts`: Reads input interrupt flags from CIA hardware registers and updates a global counter based on interrupt conditions.
- `frag_addq_bne_move_rts`: Short code fragment that adds 1 to the memory at a0, then branches if the last result was not zero, then copies 156 to the memory at a0, then adds 1 to the memory just below a0 (moving a0 down), then branches if the last result was not zero, then copies 196 to the memory at a0, then adds 1 to the memory just below a0 (moving a0 down), then returns.
- `frag_clr_move_bra`: Short code fragment that clears a fixed memory address, then copies 65280 to a fixed memory address, then jumps to another point in the code.
- `calculate_input_modifier`: Tests specific memory locations for zero or non-zero values and branches accordingly.
- `frag_tst_bne_move_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then copies 0 to the condition codes, then returns.
- `frag_moveq_mulu_move_subq_lsr`: Short code fragment that loads 7 into d2, then multiplies d2 by a location in the code (unsigned), then copies d2 to d3, then subtracts 1 from d2, then shifts d3 right by 1.
- `frag_move_rts`: Short code fragment that copies d2 to a fixed memory address, then copies d3 to a fixed memory address, then returns.
- `frag_moveq_move_clr_not_lea_lsr_scc_rts`: Short code fragment that loads 0 into d0, then copies a fixed memory address to d0, then clears a fixed memory address, then copies 64 to a fixed memory address, then inverts d0, then copies d0 to a fixed memory address, then loads the address of a location in the code into a0, then shifts d0 right by 1, then sets an indexed entry from a0 if the carry flag is clear, then returns.
- `read_joystick_input`: Processes joystick and mouse input from hardware registers including joystick movement data, mouse port data, and fire button status.
- `setup_video_registers`: Initializes various graphics hardware registers including bitplane control, display window settings, copper list pointers, and sprite configurations.
- `execute_copper_list`: Contains a trap instruction followed by several hardware register writes including setting the copper list pointer, modifying a flag bit, and branching based on that flag's state.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `copy_graphics_data`: Copies graphics data from program memory to a specific memory location in the custom chipset area.
- `frag_st_tst_bne_clr_move_rts`: Short code fragment that sets a fixed memory address to all ones, then tests a fixed memory address, then branches if the last result was not zero, then clears a fixed memory address, then copies 416 to a fixed memory address, then returns.
- `frag_st_move_rts`: Short code fragment that sets a fixed memory address to all ones, then copies d0 to a fixed memory address, then returns.
- `frag_tst_bne_rts_2`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then returns.
- `wait_for_interrupt`: Contains a trap instruction followed by several hardware register operations including setting processor status, testing a flag value, and branching based on that flag's state.
- `clear_coprocessor`: Clears the high word of the coprocessor 2 location register and writes to the interrupt request register, then returns.
