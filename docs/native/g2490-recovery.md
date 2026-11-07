# Game 2490 (infestation (1990)(psygnosis)(m3)[cr qtx][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02490). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 47 raw-word lines |
| Functions named and described | pass, 19 of 19 named (0 author symbols), 19 of 19 described (1 by rule, 14 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 44 of 44 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 435200 bytes, 7 functions |
| Independent decoder cross-check (Ghidra against capstone) | 824 of 1189 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 4052 |
| data | 683273 |
| padding | 1315 |
| unused | 212480 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Infestation (1990)(Psygnosis)(M3)[cr QTX][a].adf:Psygnosis | 2 | 141 | 9 |
| type | 2 | 117 | 2 |
| Infestation (1990)(Psygnosis)(M3)[cr QTX][a].adf:bootblock | 1 | 10 | 1 |
| Infestation (1990)(Psygnosis)(M3)[cr QTX][a].adf:Infestation (raw code file, load address unknown) | 1 | 921 | 7 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- infestation (1990)(psygnosis)(m3)[cr qtx][a]/Infestation (1990)(Psygnosis)(M3)[cr QTX][a].adf:Infestation: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 435200 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_system_environment` (Psygnosis): Performs library lookups and DOS writes, modifies CIA and DMA control registers, and executes a short routine. Kind: disk hardware/format check. Pass path: The routine likely performs a low-level verification of the disk geometry or hardware signature (CIAB_PRB) and proceeds to the main game loading sequence if the signature matches..
- `initialize_system_state` (Infestation (raw code file, load address unknown)): Populates a series of global variables and a table via a loop, then calls a routine to manage audio hardware and another routine. Kind: checksum/integrity trap. Pass path: the instruction at offset 3992 (bra.w an address) is skipped or the routine completes normally to allow the game to proceed.

## Functions


### Infestation (1990)(Psygnosis)(M3)[cr QTX][a].adf:Psygnosis

- `setup_system_environment`: Performs library lookups and DOS writes, modifies CIA and DMA control registers, and executes a short routine.
- `execute_short_routine`: A four instruction routine called via an address taken from a handler pointer in data.
- `dispatch_bit_reader_handler`: Routine of 43 instructions that is reached only through an address taken into a data pointer, so it acts as a handler entered indirectly.
- `load_registers_and_branch`: Loads two registers with the value eight and then performs a relative branch.
- `branch_conditional_bit_reader`: An eleven instruction routine that calls a bit reader routine and uses conditional branches based on a comparison.
- `call_bit_reader`: A fourteen instruction routine that calls a bit reader routine.
- `write_decrementing_global_until_wrap`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_register_data`: Moves a long word from one register to another and performs a short branch.

### Infestation (1990)(Psygnosis)(M3)[cr QTX][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Infestation (1990)(Psygnosis)(M3)[cr QTX][a].adf:Infestation (raw code file, load address unknown)

- `execute_trap_sequence`: Loads an address into a register, stores it to a memory location, and executes a trap.
- `configure_hardware_and_process_data`: This routine performs hardware register configuration and executes two consecutive subroutines that return concatenated register results.
- `call_sub_routine`: Calls another routine and returns the concatenated results of two registers.
- `initialize_system_state`: Populates a series of global variables and a table via a loop, then calls a routine to manage audio hardware and another routine.
- `configure_hardware_registers`: Writes values to various hardware registers including bitplane controllers, DMA, interrupts, and CIA registers.
- `clear_memory_block`: Fills a block of memory starting at a specific address with zeros using a loop.
- `manage_audio_hardware`: A routine consisting of 126 instructions that interacts with audio channels and DMA registers and calls another routine.
