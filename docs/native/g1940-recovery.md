# Game 1940 (kick off 2 - return to europe v2.1e (1991)(anco)[cr qtx][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01940). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901616 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (0 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 79 of 79 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 136 of 136 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 602 |
| data | 855958 |
| unused | 45056 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Kick Off 2 - Return to Europe v2.1e (1991)(Anco)[cr QTX][a].adf:bootblock | 1 | 52 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 81 | 1 |

## Cases handled

- kick off 2 - return to europe v2.1e (1991)(anco)[cr qtx][a]/Kick Off 2 - Return to Europe v2.1e (1991)(Anco)[cr QTX][a].adf: custom-format (non-DOS) disk, 1670 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 77 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- kick off 2 - return to europe v2.1e (1991)(anco)[cr qtx][a]/Kick Off 2 - Return to Europe v2.1e (1991)(Anco)[cr QTX][a].adf: blocks 153-168 (bytes 78336-86528) outside the filesystem hold data, not code (opcode density 0.008, entropy 7.14)
- kick off 2 - return to europe v2.1e (1991)(anco)[cr qtx][a]/Kick Off 2 - Return to Europe v2.1e (1991)(Anco)[cr QTX][a].adf: blocks 220-1759 (bytes 112640-901120) outside the filesystem hold data, not code (opcode density 0.012, entropy 7.04)
- 1 packed track region(s), 58368 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Kick Off 2 - Return to Europe v2.1e (1991)(Anco)[cr QTX][a].adf:bootblock

- `boot_alloc_absolute_and_io`: Boot block that calls the exec library to allocate memory at a fixed address and to run an I/O request through DoIO, and writes the colour, DMA and interrupt control registers.
- `forward_call_to_one_routine`: Eleven-instruction boot block routine reached through an address-taken reference from one caller, which makes one call to another routine.
- `write_dma_control_register`: Five-instruction boot block routine reached through an address-taken reference from one caller, which writes the DMA control register.
- `set_structure_fields_via_library`: Six-instruction routine that makes two calls through a library base at offset minus 456 and, between them, stores three constant values into a structure at offsets 36, 40 and 44 bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_word`: Tests a 16-bit word at an offset of minus 6296 from a pointer in a6 and loops until the value is zero, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_state`: Calls a function through a global pointer, updates that pointer with a value derived from bitwise operations on a memory location, and performs multiple lookups in a table based on bits from hardware registers to determine a final value.
