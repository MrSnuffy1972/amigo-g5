# Game 1207 (jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01207). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 23 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 21 of 21 named (3 author symbols), 21 of 21 described (2 by rule, 19 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 240 of 240 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 27970 |
| code | 918 |
| data | 574830 |
| padding | 5562 |
| unused | 291840 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:JStrike | 1 | 222 | 20 |
| Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:bootblock | 1 | 18 | 1 |

## Cases handled

- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a2]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:status: high-entropy data file (7.62 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a2]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:copilots: high-entropy data file (7.69 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a2]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:JSC: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a2]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:JSF: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a2]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:intro: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a2]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:menubgd: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a2]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:music: high-entropy data file (7.82 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a2]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:heli_sprite: high-entropy data file (7.66 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a2]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:sfx: high-entropy data file (7.81 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a2]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:menubgd2: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a2]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:brieffilm: high-entropy data file (7.71 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a2]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:slides: high-entropy data file (7.75 bits/byte), classified as packed data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:JStrike

- `BEGINING`: Stores a program address into a vector table and enables interrupts.
- `SUPER`: Enables interrupts and DMA channels, then runs a loop that copies a word from a source to a destination.
- `PROGGY`: Sets up the stack pointer, compares two values against thresholds, and otherwise returns.
- `store_constant_and_jump`: Stores a constant into a global variable and then jumps to another routine.
- `clear_loop_counter`: Clears a loop counter and stores zeros into a block of memory.
- `build_pointer_and_read`: Builds a destination pointer from a base and an offset, then reads a byte from a source.
- `increment_and_loop`: Adds one to a value in d0 and jumps back to the start of the routine, forming an infinite loop that only returns via the branch.
- `load_globals_and_call`: Loads a global address into a0, sets d0 and a2 to constants, then calls the routine at 2a92.
- `call_and_indirect_jump`: Loads a global address into a0, sets d0 and a2 to constants, then calls the routine at 2a92.
- `decode_and_copy_bytes`: Builds a destination pointer from a base and an offset, then reads a byte from a source.
- `copy_bytes_then_branch`: Copies a byte from the source pointed to by a0 to the destination pointed to by a1, advancing both pointers, then decrements d0 and repeats until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `save_registers_and_read`: Saves registers on the stack, sets d4 to 15, and reads a short from a source.
- `adjust_d6_and_copy`: Adds 16 to d6, combines d6 with d3 using a bitwise OR, then copies d1 into d6 and d2 into d7.
- `compare_and_adjust_d6`: Adds 16 to d6, combines d6 with d3 using a bitwise OR, then copies d1 into d6 and d2 into d7.
- `modify_and_store_d6`: Adds 256 to d6, combines d6 with d3 using a bitwise OR, and stores the result at the stack location below a1.
- `call_twice_loop_restore`: Calls the routine at d38 twice, copies a2 into a1 and a3 into a0, then runs a loop that decrements d4 and jumps back to c40 until it reaches zero.
- `process_and_decrement_values`: Saves registers on the stack, sets d4 to 15, and reads a short from a source.
- `call_twice_loop_restore2`: Calls the routine at d38 twice, copies a2 into a1 and a3 into a0, then runs a loop that decrements d4 and jumps back to cdc until it reaches zero.
- `wait_for_interrupt`: Sets a5 to the hardware register base, writes a value to a register at offset 9c, then polls a bit in a register at offset 1e.

### Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
