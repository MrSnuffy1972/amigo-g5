# Game 1395 (anarchy (1991)(psygnosis)[cr angels]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01395). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1803072 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 7 of 7 named (0 author symbols), 7 of 7 described (0 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 8 of 8 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 364 of 364 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 40924 |
| code | 1126 |
| data | 1738492 |
| padding | 2 |
| unused | 22528 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Anarchy (1991)(Psygnosis)[cr Angels](Disk 1 of 2).adf:bootblock | 1 | 7 | 1 |
| logged disk load from Anarchy (1991)(Psygnosis)[cr Angels](Disk 1 of 2).adf boot-block load | 1 | 97 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 260 | 4 |

## Cases handled

- anarchy (1991)(psygnosis)[cr angels](disk 1 of 2)/Anarchy (1991)(Psygnosis)[cr Angels](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- anarchy (1991)(psygnosis)[cr angels](disk 2 of 2)/Anarchy (1991)(Psygnosis)[cr Angels](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1714 non-empty blocks: matched against track-loaded segments
- anarchy (1991)(psygnosis)[cr angels](disk 1 of 2)/Anarchy (1991)(Psygnosis)[cr Angels](Disk 1 of 2).adf: the boot block reads 41984 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- anarchy (1991)(psygnosis)[cr angels](disk 1 of 2)/Anarchy (1991)(Psygnosis)[cr Angels](Disk 1 of 2).adf: 1 logged loads cover 41984 bytes of the image
- anarchy (1991)(psygnosis)[cr angels](disk 2 of 2)/Anarchy (1991)(Psygnosis)[cr Angels](Disk 2 of 2).adf: blocks 2-439 (bytes 1024-225280) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.46)
- anarchy (1991)(psygnosis)[cr angels](disk 2 of 2)/Anarchy (1991)(Psygnosis)[cr Angels](Disk 2 of 2).adf: blocks 462-1002 (bytes 236544-513536) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.53)
- anarchy (1991)(psygnosis)[cr angels](disk 2 of 2)/Anarchy (1991)(Psygnosis)[cr Angels](Disk 2 of 2).adf: blocks 1025-1759 (bytes 524800-901120) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.00)
- 1 packed track region(s), 858112 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Anarchy (1991)(Psygnosis)[cr Angels](Disk 1 of 2).adf:bootblock

- `issue_exec_io_request`: Seven-instruction boot block entry that calls the exec DoIO library function.

### logged disk load from Anarchy (1991)(Psygnosis)[cr Angels](Disk 1 of 2).adf boot-block load

- `store_combined_byte_pair`: Reads a 32-bit value from memory just before the address register, then writes a 16-bit value built from the high byte of one register and the low byte of that value through a second pointer.
- `shift_bits_with_refill`: Shifts a bit buffer right once per iteration for a count taken from a second register, and calls an unnamed routine to refill the buffer whenever it becomes zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprites_each_frame`: A 244-instruction routine reached by execution from outside the static flow, which advances a frame counter and recomputes sprite data pointers and sprite control words from stored coordinates.
- `dispatch_through_pointer_table`: Calls a routine whose address is read from a fixed offset in a pointer table held in memory.
- `call_pointer_if_flagged`: If a flag byte in memory is nonzero, calls a routine through a function pointer read from a fixed offset in a table.
- `stub_two_instruction_routine`: Two-instruction routine called by one unnamed routine, with no decompiled pseudo-code because the decompiler timed out.
