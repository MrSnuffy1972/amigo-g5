# Game 332 (head over heels (1990)(ocean)[cr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00332). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 10 of 10 named (0 author symbols), 10 of 10 described (1 by rule, 9 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 142 of 142 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 186 of 186 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 538 |
| data | 159306 |
| padding | 924 |
| unused | 740352 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Head Over Heels (1990)(Ocean)[cr].adf:HOH.EXE | 3 | 168 | 9 |
| Head Over Heels (1990)(Ocean)[cr].adf:bootblock | 1 | 18 | 1 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- head over heels (1990)(ocean)[cr]/Head Over Heels (1990)(Ocean)[cr].adf:HOH.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Head Over Heels (1990)(Ocean)[cr].adf:HOH.EXE

- `jump_to_reset_vector`: Saves all registers, calls the routine at address zero, restores the registers, and jumps to address zero.
- `save_registers_call`: Saves all registers onto the stack, calls another routine, restores the registers, and returns.
- `call_trap_routine`: Saves the current longword at global offset 128 into d7, loads the address of a routine into a0, stores that address at global offset 128, and executes trap zero.
- `reset_vector_table`: Stores the value -1 at the longword pointed to by the global vector table at offset 46, then loads 8192 into d0 and returns to the operating system.
- `install_fast_routine`: Sets up a supervisor stack, saves the registers, disables audio and DMA, copies nineteen longwords from a table into fast memory, and jumps to the copied routine.
- `copy_and_transform_block`: Copies a block of longwords from one memory region to another, then branches into a series of cases that call the half-or-input routine and the palette-supervisor routine to build a transformed block before jumping back.
- `return_half_or_input`: Clears a decrementing flag in d7 and, when it reaches zero, loads 16 into that flag while shifting a saved word in d6 right by one bit and returning the result, otherwise it returns the input word in d0 unchanged.
- `lookup_and_enter_supervisor`: Computes an index from d1, masks a value from d6 against a byte in a lookup table, then disables interrupts, writes five bytes to the color palette and calls the supervisor-mode routine.
- `enter_supervisor_mode`: Clears the longword at global offset 128, pushes two words onto the stack, and enters supervisor mode via trap zero.

### Head Over Heels (1990)(Ocean)[cr].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
