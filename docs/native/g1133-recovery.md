# Game 1133 (disposable hero (1993)(gremlin)(disk 1 of 3)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01133). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 7 of 7 named (0 author symbols), 7 of 7 described (2 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 20 of 20 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 89 of 89 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 286 |
| data | 483011 |
| padding | 543 |
| unused | 417280 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disposable Hero (1993)(Gremlin)(Disk 1 of 3).adf:COSY_WAS_ERE | 1 | 79 | 6 |
| Disposable Hero (1993)(Gremlin)(Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- disposable hero (1993)(gremlin)(disk 1 of 3)/Disposable Hero (1993)(Gremlin)(Disk 1 of 3).adf:COSY_WAS_ERE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Disposable Hero (1993)(Gremlin)(Disk 1 of 3).adf:COSY_WAS_ERE

- `search_list_then_setup_vectors`: Walks a linked list of structures by following their next pointers, comparing 16-bit fields until it finds a match or reaches the end, then sets up custom registers, copies a routine into a vector table, and calls it or loops forever based on a computed value.
- `disable_interrupts_set_pointer`: Disables interrupts by writing a mask to the status register, computes a local address, and stores it into a global pointer variable, then falls through into the following routine.
- `init_custom_registers_copy_vector`: Disables interrupts and DMA, clears several custom registers, copies a 21-word routine into a vector table via a loop, saves d7 onto the stack, and then either jumps to the copied routine or enters an infinite loop depending on the sign of d7.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `setup_pointers_copy_words`: Loads the address in a2 by adding a constant offset to d7, then computes a displacement from d0 and copies 16-bit words from one list to another using a loop that decrements d0 by two, returning indirectly through a2 when the counter is exhausted.
- `copy_words_loop_then_jump`: Copies a 16-bit word from the source pointer in a5 to the destination pointer in a1, decrementing the counter in d0 by two each time, and repeats until the counter reaches zero, then jumps indirectly through the address in a2.

### Disposable Hero (1993)(Gremlin)(Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
