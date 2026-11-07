# Game 1366 (vroom (1991)(lankhor)[cr tkk][t +1 tkk]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01366). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902656 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 19 raw-word lines |
| Functions named and described | pass, 23 of 23 named (0 author symbols), 23 of 23 described (2 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 99 of 99 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 275 of 275 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11006 |
| code | 1018 |
| data | 615688 |
| unused | 274944 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Vroom (1991)(Lankhor)[cr TKK][t +1 TKK].adf:bootblock | 1 | 15 | 1 |
| logged disk load from Vroom (1991)(Lankhor)[cr TKK][t +1 TKK].adf boot-block load | 1 | 27 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 233 | 19 |

## Cases handled

- vroom (1991)(lankhor)[cr tkk][t +1 tkk]/Vroom (1991)(Lankhor)[cr TKK][t +1 TKK].adf: custom-format (non-DOS) disk, 1221 non-empty blocks: matched against track-loaded segments
- vroom (1991)(lankhor)[cr tkk][t +1 tkk]/Vroom (1991)(Lankhor)[cr TKK][t +1 TKK].adf: the boot block reads 11776 bytes from disk offset 0 to an address (found statically); recovered as a unit
- runtime segment an address: 78 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- vroom (1991)(lankhor)[cr tkk][t +1 tkk]/Vroom (1991)(Lankhor)[cr TKK][t +1 TKK].adf: 1 logged loads cover 11776 bytes of the image
- vroom (1991)(lankhor)[cr tkk][t +1 tkk]/Vroom (1991)(Lankhor)[cr TKK][t +1 TKK].adf: blocks 1600-1702 (bytes 819200-871936) outside the filesystem hold data, not code (opcode density 0.014, entropy 7.12)
- 3 packed track region(s), 561664 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Vroom (1991)(Lankhor)[cr TKK][t +1 TKK].adf:bootblock

- `load_block_via_io_loop`: Boot block entry that repeatedly issues exec DoIO requests for a fixed-length block at a start-labelled address until a status byte clears.

### logged disk load from Vroom (1991)(Lankhor)[cr TKK][t +1 TKK].adf boot-block load

- `step_until_values_match`: Loops while two byte values differ, calling an unnamed routine and stepping one of the values up or down each time, then waits until a status bit in a memory byte clears.
- `set_low_status_bit_and_spin`: Clears bit one and sets bit zero of a byte at an address register, then loops endlessly.
- `set_status_bits_and_spin`: Sets bit one and bit zero of a byte at an address register, then loops endlessly.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_return_only`: Two-instruction routine in an unreached gap that returns immediately with no work.
- `dispatch_interrupt_bits_to_handler`: Tests set bits in an interrupt status global and, for each bit that is set, stores a value and calls a handler through a pointer read from a table.
- `route_interrupt_flags_to_handlers`: Returns unchanged unless a bit of an interrupt status global is set, then tests further bits and stores a fixed value for each one found before calling handlers.
- `dispatch_request_bit_to_handler`: Tests interrupt request bits and, when one is set, stores a fixed value and calls a handler through a pointer in an address register table.
- `dispatch_upper_request_bits_to_handlers`: Tests the upper interrupt request bits and calls a handler from a pointer table for a set bit, storing a fixed value for it.
- `dispatch_top_request_bit_handler`: Stores a fixed value and then calls a handler through a pointer table entry when the tested interrupt request bit is set.
- `stub_return_only_2`: Two-instruction routine whose body is a bare return, called by two unnamed routines.
- `stub_return_only_3`: Fifteen-instruction routine whose decompiled body is a bare return, so no behaviour is recovered from the code.
- `stub_cia_register_return`: Three-instruction unreached routine whose decompiled body is a bare return, so no behaviour is recovered.
- `dispatch_pending_state_handler`: Checks a pending state global and, when it is set, calls a handler selected from a table indexed by that state, then clears the pending state.
- `dispatch_pending_state_handler_2`: Checks a pending state global and, when it is set, calls a handler selected from a table indexed by that state.
- `flush_pending_state_call`: If a pending state global is set, calls one unnamed routine and then clears the pending state to minus one.
- `write_stepped_longword_table`: Writes a sequence of longword entries to a buffer, each holding a running value that increases by a fixed step and split into two 16-bit words.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `set_display_fetch_registers`: Writes the bitplane control register and the display data fetch start and stop registers with fixed values.
- `set_fixed_bits_in_byte`: Returns its input byte with a fixed set of bits forced on.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `poll_input_ports_with_timing`: Samples the fire-button and disk status port registers of both CIA chips and the joystick and mouse ports, with the right and middle button register, and uses a counter to time a short window before updating state.
- `stub_bad_instruction_data`: Routine whose decompiled output ends in an undecodable instruction, so no routine behaviour is recovered.
