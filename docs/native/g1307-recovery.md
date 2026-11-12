# Game 1307 (laser squad (1989)(microillusions)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01307). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902368 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 10 of 10 named (0 author symbols), 10 of 10 described (0 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 242 of 242 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 319 of 319 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 1208 |
| data | 4648 |
| unused | 896512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Laser Squad (1989)(MicroIllusions).adf:bootblock | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 262 | 3 |

## Cases handled

- laser squad (1989)(microillusions)/Laser Squad (1989)(MicroIllusions).adf: file Trashcan.info (header 1166 bytes, blocks hold 488): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 228 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 immediate, 1 immediate+address
- laser squad (1989)(microillusions)/Laser Squad (1989)(MicroIllusions).adf: blocks 687-688 (bytes 351744-352768) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.48)
- laser squad (1989)(microillusions)/Laser Squad (1989)(MicroIllusions).adf: blocks 884-884 (bytes 452608-453120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.91)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Laser Squad (1989)(MicroIllusions).adf:bootblock

- `forward_boot_entry_to_routine`: Boot block entry consisting of a single instruction that calls one unnamed routine.
- `invoke_unnamed_routine_once`: Seven-instruction routine whose address is taken by a PC-relative load.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `unnamed_routine`: The routine is empty as no instructions or operations were provided in the pseudo-code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_without_action`: Routine whose shown body is only a return, so it does no work in the pseudo-code.
- `store_fixed_constant_and_return`: Stores a fixed 32-bit constant into a global variable and returns.
- `call_routine_then_set_flag`: Calls one unnamed routine, then sets a global flag to one if it is currently zero, and returns.
- `write_constant_to_memory`: writes a constant 32-bit value to a global memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_audio_parameters`: Updates entity position or coordinate values and modifies audio hardware registers.
- `update_entity_kinematics`: Updates an entity's position or coordinate values based on a command nibble, applying movement increments, decrements, or boundary clamping.
- `update_sprite_attribute_table`: This routine reads a nibble from a data source to index into a table, then updates a structure with new coordinate and attribute values.
