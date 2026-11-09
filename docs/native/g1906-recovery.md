# Game 1906 (international karate + (1988)(system 3)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01906). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901140 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 5 of 5 named (0 author symbols), 5 of 5 described (1 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 42 of 42 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 362496 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 106 of 430 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 18 |
| code | 1726 |
| data | 389308 |
| padding | 1160 |
| unused | 508928 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| International Karate + (1988)(System 3)[a].adf:ikload | 1 | 43 | 2 |
| International Karate + (1988)(System 3)[a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| International Karate + (1988)(System 3)[a].adf:ik+ (raw code file, load address unknown) | 1 | 372 | 1 |

## Cases handled

- 5 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 0 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- international karate + (1988)(system 3)[a]/International Karate + (1988)(System 3)[a].adf:ik+: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 362496 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### International Karate + (1988)(System 3)[a].adf:ikload

- `open_dos_library_entry`: Program entry that opens the dos library through the exec OldOpenLibrary call and makes three further library calls at fixed offsets.
- `stub_seven_instruction_routine`: A seven-instruction routine reached through an address-taken reference from one caller.

### International Karate + (1988)(System 3)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_dos_library_values`: Stores the decimal value 18720 into the longword pointed to by a6, clears the longwords pointed to by a7 and a8, and calls the Amiga DOS library routine.

### International Karate + (1988)(System 3)[a].adf:ik+ (raw code file, load address unknown)

- `enter_raw_code_region`: Start of a 372-instruction raw code region with no symbols, no pseudo-code and no listed calls.
