# Game 1493 (distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01493). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (856064 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 16 of 16 named (0 author symbols), 16 of 16 described (0 by rule, 16 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 504 of 504 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 735 |
| code | 1652 |
| data | 847533 |
| unused | 6144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf:bootblock | 1 | 40 | 2 |
| logged disk load from Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf boot-block load | 1 | 0 | 0 |
| logged disk load from Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf boot-block load | 1 | 464 | 14 |

## Cases handled

- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: custom-format (non-DOS) disk, 1658 non-empty blocks: matched against track-loaded segments
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: the boot block reads 2048 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: 2 logged loads cover 3072 bytes of the image
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: blocks 4-21 (bytes 2048-11264) hold ASCII text
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: blocks 33-341 (bytes 16896-175104) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.54)
- distant armies - a playing history of chess (1988)(eagle tree software)[cr band][a]/Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band][a].adf: blocks 343-569 (bytes 175616-291840) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.27)
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
