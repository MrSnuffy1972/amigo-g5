# Game 3630 (final assault (1988)(infogrames)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03630). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902656 top-level bytes in 48 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 64 raw-word lines |
| Functions named and described | pass, 42 of 42 named (0 author symbols), 42 of 42 described (24 by rule, 17 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 231 of 231 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1213 of 1213 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 646028 |
| code | 3706 |
| data | 83943 |
| padding | 9747 |
| unused | 159232 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| run | 2 | 435 | 12 |
| endcli | 2 | 117 | 2 |
| Final Assault (1988)(Infogrames).adf:biv | 2 | 59 | 11 |
| Final Assault (1988)(Infogrames).adf:load | 3 | 163 | 2 |
| Final Assault (1988)(Infogrames).adf:bootblock | 1 | 38 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 352 | 11 |

## Cases handled

- final assault (1988)(infogrames)/Final Assault (1988)(Infogrames).adf: file          (header 297 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 223 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 branch displacement, 1 jump target
- final assault (1988)(infogrames)/Final Assault (1988)(Infogrames).adf: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.70)
- final assault (1988)(infogrames)/Final Assault (1988)(Infogrames).adf:biv: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_disk_io` (runtime segment an address): Initializes disk I/O structures by allocating memory, setting up message ports, and configuring device requests. Kind: disk hardware check. Pass path: the routine continues normally after successful hardware setup and signal allocation, with no branching based on invalid disk state.

## Functions


### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `initialize_object_data`: Writes multiple values from registers and tables into a structure pointed to by a1, including bit manipulation on a byte value, and performs conditional assignments based on a global variable's content.
- `load_tile_data`: Reads a byte from a table using a calculated index and stores it in a structure.
- `update_object_tile`: Updates object tile data by copying bytes from one memory location to another based on calculated offsets, then stores a constant value at a specific memory address.
- `update_object_data`: Updates object data fields including position, velocity, and state flags.
- `frag_moveq_add_bra`: Short code fragment that loads 1 into d1, then adds the memory at an offset from a1 to d1, then jumps to another point in the code.
- `frag_tst_bne_bra`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_move_and_tst_beq_ori_moveq_cmp_blt`: Short code fragment that copies 128 to d1, then masks d1 with the memory at an offset from a1, then tests d1, then branches if the last result was zero, then merges an address into the memory at an offset from a1, then loads 133 into d1, then compares d1 with the memory at an offset from a1, then branches if the comparison was less, then jumps to another point in the code.
- `initialize_and_process_3`: Initializes multiple global variables with zero or specific values, processes data pointers, and performs conditional operations based on status flags.
- `initialize_structure`: Initializes multiple fields in a data structure with zero or specific values, processes pointers, and performs conditional operations based on a status flag.
- `initialize_and_process`: Initializes multiple global variables with zero or specific values, processes data pointers, and performs conditional operations based on structure offsets before jumping to an unresolved address.

### endcli

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, writes a header with that value, fills a buffer with sequential words, and sets up a stack frame for a subsequent call.
- `frag_move_bra_2`: Short code fragment that copies d1 to d0, then jumps to another point in the code.

### Final Assault (1988)(Infogrames).adf:biv

- `frag_jsr`: Short code fragment that calls another routine.
- `frag_move_bne`: Short code fragment that copies the memory at an offset from a6 to d7, then branches if the last result was not zero (4 times in a row).
- `frag_bne`: Short code fragment that branches if the last result was not zero (6 times in a row).
- `frag_bne_2`: Short code fragment that branches if the last result was not zero (6 times in a row).
- `frag_bne_3`: Short code fragment that branches if the last result was not zero (6 times in a row).
- `frag_bne_4`: Short code fragment that branches if the last result was not zero (6 times in a row).
- `frag_bne_5`: Short code fragment that branches if the last result was not zero (6 times in a row).
- `frag_bvc`: Short code fragment that branches if the overflow flag is clear.
- `frag_bchg_beq_moveq`: Short code fragment that flips bit d5 of an indexed entry from a4, then branches if the last result was zero, then loads 48 into d5, then loads 88 into d3.
- `frag_bvs_bmi`: Short code fragment that branches if the overflow flag is set, then branches if the last result was negative.
- `xor_and_invert_loop`: Processes a memory region by XORing values with a constant and then inverting words in a loop, while updating a global variable and manipulating a status byte bit.

### Final Assault (1988)(Infogrames).adf:load

- `process_input_string`: Processes a null-terminated string by parsing characters and handling special cases like quotes and negative numbers, updating a global data structure with parsed values.
- `null_terminated_string_handler`: Walks through a null-terminated string by incrementing a pointer until it reaches the end marker, then makes two library calls before returning.

### Final Assault (1988)(Infogrames).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `validate_and_load`: Checks for a specific data pattern at a stack offset and branches if it does not match.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_and_process`: Loads data from a global variable at offset 12 into a register, processes it through a table lookup, and stores the result back to another global variable at offset 8.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_and_compare`: Loads a value from a global variable at offset 12 and compares it with another global variable at offset 8.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_disk_io`: Initializes disk I/O structures by allocating memory, setting up message ports, and configuring device requests.
- `frag_move_bra_3`: Short code fragment that copies 32770 to d4, then jumps to another point in the code.
- `frag_move_bra_4`: Short code fragment that copies 32771 to d4, then jumps to another point in the code, then copies 32773 to d4, then jumps to another point in the code.
- `frag_moveq_move_bra`: Short code fragment that loads 0 into d2, then loads 0 into d1, then copies 32778 to d4, then jumps to another point in the code.
- `frag_move_bra_5`: Short code fragment that copies 13 to d4, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 1 into d3, then jumps to another point in the code.
- `frag_moveq_move_bra_2`: Short code fragment that loads 0 into d3, then copies 32777 to d4, then jumps to another point in the code.
- `initialize_and_process_2`: Initializes data in a structure by copying values from other locations, performs arithmetic operations, sets bits in a register, and calls a library function before returning.
- `frag_move_moveq_bset_movea_jmp`: Short code fragment that copies 16384 to the memory at an offset from a0, then copies 2 to the memory at an offset from a0, then copies the memory at an offset from a1 to d1, then loads 0 into d0, then sets bit d1 of d0, then copies the memory at an offset from a1 to a1, then jumps to another point in the code.
- `process_bitfield_and_clear_memory`: Processes a bitfield value by setting a bit in a register, then clears memory locations in a loop of 324 iterations before returning.
- `frag_move_not_dbra_lea_bra`: Short code fragment that copies 14 to d0, then inverts the memory at a0 (advancing a0), then loops back while counting down d0, then loads the address of a location in the code into a0, then copies 18025 to the memory at a0, then jumps to another point in the code.
