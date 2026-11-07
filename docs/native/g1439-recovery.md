# Game 1439 (anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01439). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 45 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 9 of 9 named (0 author symbols), 9 of 9 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 100 of 100 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 317 of 317 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 50711 |
| code | 1082 |
| data | 773661 |
| padding | 10642 |
| unused | 65024 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Off | 2 | 52 | 2 |
| Type | 2 | 156 | 3 |
| Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:Anarchy | 1 | 99 | 3 |
| Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_23: high-entropy data file (7.81 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_24: high-entropy data file (7.74 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_25: high-entropy data file (7.76 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_05: high-entropy data file (7.88 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_06: high-entropy data file (7.81 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_07: high-entropy data file (7.87 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_08: high-entropy data file (7.91 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_10: high-entropy data file (7.82 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_12: high-entropy data file (7.85 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_0A: high-entropy data file (7.89 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_14: high-entropy data file (7.87 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_0C: high-entropy data file (7.84 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_16: high-entropy data file (7.76 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_0E: high-entropy data file (7.84 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][h toad][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:DEFJAM_18: high-entropy data file (7.84 bits/byte), classified as packed data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Off

- `open_libraries_and_run_task`: Program entry of fifty instructions that opens libraries by name, adds an interrupt server, waits until two screen dimension fields reach fixed values, and then removes its task through Exec routines.
- `parse_command_line_and_dispatch`: Command-line handling routine that scans an argument string for a space and dispatches on the character after it, calling library routines to open and use a library along the way.

### Type

- `load_file_into_allocated_memory`: Program entry of 139 instructions that opens a file through DOS, seeks and reads it into memory allocated through Exec, and closes library handles.
- `call_indirect_through_pointer`: Calls a routine through a pointer held relative to a register and then returns.
- `call_indirect_from_table`: Calls a routine taken from a global pointer table at a fixed offset and then returns; the facts also list a DOS write library call.

### Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:Anarchy

- `set_background_color_at_start`: Program entry point that writes the background color register and calls four unnamed routines.
- `shift_long_and_store_word`: Reads the longword four bytes before the address register, writes a word built from its low byte and the high byte of a data register through a pointer, and returns the value shifted right one bit with the top bit set.
- `shift_bits_through_refill_loop`: Shifts a bit register right one place per step for a count taken from its argument, calling a refill routine whenever the register empties.

### Anarchy (1991)(Psygnosis)[cr Defjam - CCS][h TOAD][t +7 Defjam - CCS].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
