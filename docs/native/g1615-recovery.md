# Game 1615 (rick dangerous (1989)(micro style)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01615). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 35 of 35 named (0 author symbols), 35 of 35 described (5 by rule, 12 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 82 of 82 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 958 of 958 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2364 |
| code | 2836 |
| data | 204011 |
| padding | 1733 |
| unused | 690176 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rick Dangerous (1989)(Micro Style)[cr QTX].adf:rickdangrous | 2 | 165 | 8 |
| Run | 2 | 435 | 12 |
| Type | 2 | 117 | 2 |
| Rick Dangerous (1989)(Micro Style)[cr QTX].adf:VIRUS | 2 | 24 | 4 |
| Rick Dangerous (1989)(Micro Style)[cr QTX].adf:rick | 2 | 207 | 8 |
| Rick Dangerous (1989)(Micro Style)[cr QTX].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- reference run failed (Karl 4 Nov 2026): run play-long-joy slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 131072 of its 164872 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-keys slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 131072 of its 164872 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-mixed slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 131072 of its 164872 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 131072 of its 164872 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 131072 of its 164872 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 131072 of its 164872 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 131072 of its 164872 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x25k slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 131072 of its 164846 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x35k slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 131072 of its 164846 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference crashes at an address (HLE illegal instruction an address at an address: through vector 0, as before (vector 4 is 0 again)): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- rick dangerous (1989)(micro style)[cr qtx]/Rick Dangerous (1989)(Micro Style)[cr QTX].adf:VIRUS: ShellScript Exe (xfd) stream found by ancient scan (2408 -> 103 bytes) but only 1 of 1 sampled windows are in reference RAM: not used
- rick dangerous (1989)(micro style)[cr qtx]/Rick Dangerous (1989)(Micro Style)[cr QTX].adf:rick: PowerPacker 2.3 (xfd) stream found by ancient scan (18912 -> 69672 bytes) but only 0 of 15 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 operand, 1 jump target
- rick dangerous (1989)(micro style)[cr qtx]/Rick Dangerous (1989)(Micro Style)[cr QTX].adf:rickdangrous: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rick Dangerous (1989)(Micro Style)[cr QTX].adf:rickdangrous

- `enter_supervisor_and_set_hardware`: Switches to supervisor state through the system library, then writes to the CIA port, DMA control and interrupt enable registers.
- `merge_constant_via_routine`: Calls a one-instruction other routine that merges a constant into the data register.
- `clear_bit_then_expand`: Clears a bit in a byte using a register offset and ORs a constant into a global long word, then stores a computed value and handler address before calling the backward packed data expander and a hardware trap routine.
- `expand_backward_packed_data`: Reads a stream length from a header, then loops over flag bits.
- `expand_packed_runs_backwards`: Decodes packed runs copied backwards from the end of the source, alternating literal copies with back-references whose lengths come from bit reads and tables.
- `decode_state_driven_packed_copies`: Keeps a state value that selects between a length decode and a distance decode, copying bytes backwards from the source for each back-reference.
- `copy_packed_runs_until_done`: Repeatedly reads length and distance codes from the packed bit stream and copies runs of bytes backwards from the source.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If the argument pointer is non-null, copies sixteen longwords from a fixed offset in a table into that buffer.
- `update_sprite_state`: Updates sprite-related state variables using data from a table indexed by a register, performs memory operations including bit manipulation and structure field assignments, and handles conditional logic based on internal state values.
- `load_sprite_data`: Loads sprite-related data from a table using an index, updates internal state variables, and performs a series of memory operations including copying values and setting flags based on calculated offsets.
- `update_sprite_data_2`: Updates sprite-related data structures by copying values between memory locations based on calculated offsets, then performs a series of conditional operations on sprite properties before returning.
- `update_sprite_data`: Updates sprite-related data structures by copying values between memory locations based on calculated offsets, then performs a series of conditional checks and assignments before returning.
- `load_and_add`: Loads an immediate value into a register and adds a long word from memory to that register, then branches unconditionally to another location.
- `check_condition_and_branch`: Tests a long value at offset 28 of a1, and branches to address 22a if the value is not zero.
- `update_status_flag`: Loads a long value from memory location at offset 34 of a1, performs a bitwise AND with the value 128, tests the result, and branches to address 242 if zero.
- `initialize_object_data_2`: Writes data values to memory locations based on calculated offsets, clears several fields in a structure, and sets up initial state values for an object.
- `initialize_object_data`: Writes data values to memory locations based on calculated offsets, clears several fields in a structure, and sets up initial state values for an object.
- `process_data_and_jump`: Processes multiple data values from memory locations, performs conditional jumps based on test results, and executes several function calls through a jump table before ending with an unconditional jump.

### Type

- `build_handler_table_from_sizes`: Finds the largest value in a linked list, then fills a table sized from it with a counted pattern and stores a handler address in it.
- `copy_table_block`: Copies a fixed 16-word block from a source table into a destination buffer pointed to by an argument, then invokes another routine.

### Rick Dangerous (1989)(Micro Style)[cr QTX].adf:VIRUS

- `subtract_constant_from_byte_table`: Subtracts a fixed value from every byte of a table, then stores the result of a system call and calls another routine.
- `subtract_table_then_open_dos`: Subtracts a fixed value from every byte of a table, then calls the system library routine that opens an old-style library for the dos library name referenced here.
- `execute_dos_command_via_pointer`: Calls a dos library routine through a pointer loaded from a global cell, using the Execute entry named in its facts, and then returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### Rick Dangerous (1989)(Micro Style)[cr QTX].adf:rick

- `load_amiga_hunk_executable`: Walks the hunk records of an AmigaDOS executable, allocating memory for each code and data hunk and copying its contents in.
- `return_empty_stub`: Contains only a return and performs no work.
- `spin_register_until_zero`: Decrements a 32-bit register in a loop until it reaches zero, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (chk.w) and then falls through into the code that follows, with no return of its own.
- `decode_run_length_bytes_backwards`: Decodes a stream of run-length and back-reference codes, writing bytes backwards from the end of a buffer until the write pointer reaches the start.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_with_wrap`: Decrements a byte counter for a given number of iterations and resets it to a space character whenever it reaches zero.
- `record_undecoded_routine`: The decompiler timed out on this three-instruction routine, so its behaviour cannot be recovered from the listed facts.

### Rick Dangerous (1989)(Micro Style)[cr QTX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
