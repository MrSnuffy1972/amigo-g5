# Game 1133 (disposable hero (1993)(gremlin)(disk 1 of 3)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01133). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 18 of 18 named (0 author symbols), 18 of 18 described (3 by rule, 15 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 20 of 20 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 189 of 189 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 518 |
| data | 482779 |
| padding | 543 |
| unused | 417280 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disposable Hero (1993)(Gremlin)(Disk 1 of 3).adf:COSY_WAS_ERE | 1 | 179 | 17 |
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
- `handle_data_shift`: Processes data by shifting bits, calls other routines for copying operations, and handles conditional branching based on comparisons.
- `shift_right_loop`: Shifts a long word right by one bit position and branches to another routine if the result is zero.
- `rotate_right_word`: Shifts a long word right through carry by one bit position and returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (subq.w) and then falls through into the code that follows, with no return of its own.
- `shift_left_loop`: Initializes a register to zero, shifts a long word right by one bit, branches to another routine if the result is zero, and performs a rotate left operation.
- `rotate_right_long`: Shifts a long word right through carry by one bit position and branches back to an earlier instruction.
- `process_data_block`: Calls several other routines in sequence, compares values, and performs byte copying operations based on conditions.
- `copy_bytes_loop`: Calls another routine, copies bytes from one memory location to another in a loop, and compares addresses.
- `copy_word_loop`: Loads data from a table, calculates addresses, and copies words between memory locations in a loop until a condition is met.
- `setup_and_call`: Calculates an address using register values, calls another routine, and then branches to a specific location.
- `jump_to_function`: Loads a function address from a global table, adds an offset to it, and jumps to that address.

### Disposable Hero (1993)(Gremlin)(Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
