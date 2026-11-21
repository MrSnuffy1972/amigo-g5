# Game 4531 (NinjaWarriors v1.2 0543): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04531). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4487842 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 27 of 27 named (0 author symbols), 27 of 27 described (12 by rule, 13 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 768 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 291 of 291 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 15536 |
| code | 1162 |
| data | 4470886 |
| padding | 2 |
| unused | 256 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 291 | 27 |
| Disk.1 bytes 0-768 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- NinjaWarriors_v1.2_0543/NinjaWarriors_v1.2_0543_hdf/NinjaWarriors/Disk.1: raw disk image (991552 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- NinjaWarriors_v1.2_0543/NinjaWarriors_v1.2_0543_hdf/NinjaWarriors/Disk.2: raw disk image (998400 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- NinjaWarriors_v1.2_0543/NinjaWarriors_v1.2_0543.hdf: hard-disk image (AmigaDOS OFS, 4832 blocks): its filesystem parses and all 8 files in it are byte-identical to the extracted tree NinjaWarriors_v1.2_0543_hdf, which is recovered file by file; no block outside the filesystem holds data
- NinjaWarriors_v1.2_0543/NinjaWarriors_v1.2_0543_hdf/NinjaWarriors/Disk.1: bytes 0-768 look like code no run loaded: recovered position-independently (load address unknown)
- NinjaWarriors_v1.2_0543/NinjaWarriors_v1.2_0543_hdf/NinjaWarriors/Disk.1: 1 logged loads cover 768 bytes of the image
- NinjaWarriors_v1.2_0543/NinjaWarriors_v1.2_0543_hdf/NinjaWarriors/Disk.1: chunks 4-3873 (bytes 1024-991552) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.37)
- NinjaWarriors_v1.2_0543/NinjaWarriors_v1.2_0543_hdf/NinjaWarriors/Disk.2: chunks 0-3899 (bytes 0-998400) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.07)
- 1 never-loaded region(s), 768 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decrement_counter`: Initializes a 16-bit counter in a global variable to zero, then decrements it by one if a byte value from a specific memory location is between 2 and 6 inclusive.
- `initialize_memory_and_jmp`: Initializes multiple memory locations with specific word values, sets up jump vectors, and performs conditional jumps based on global data.
- `save_registers`: Saves all general-purpose and address registers to a structure in memory when a global pointer is non-zero, then clears that pointer.
- `frag_movem_addq_bra`: Short code fragment that restores saved registers from the stack, then adds 8 to a7, then jumps to another point in the code.
- `restore_context`: Sets the processor status register to enable interrupts, initializes the stack pointer, clears a global variable, calls a routine, stores its return value, and restores processor state from saved data in memory.
- `frag_jsr_move`: Short code fragment that calls another routine, then copies 8192 to the status register.
- `save_and_execute`: Saves a pointer in a global variable, then executes code from a structure pointed to by that pointer.
- `frag_move_andi_movea_rts`: Short code fragment that copies d0 to the stack, then copies a0 to d0, then masks d0 with an address, then copies d0 to a0, then copies a1 to d0, then masks d0 with an address, then copies d0 to a1, then copies the top of the stack to d0, then returns.
- `frag_bsr_moveq_rts`: Short code fragment that calls another routine, then loads 0 into d0, then loads 0 into d1, then loads 0 into d2, then returns.
- `frag_bsr_moveq_move_rts`: Short code fragment that calls another routine, then loads 0 into d0, then copies the memory at an offset from a2 to d0, then returns.
- `frag_move_andi_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with an address, then copies the memory at an offset from a6 to d4, then masks d4 with an address, then returns.
- `calculate_offset`: Saves all registers on the stack, performs a multiplication and subtraction operation using values from global memory, then calls another routine with a calculated parameter before restoring registers.
- `compute_value`: Saves all registers on the stack, performs a multiplication and conditional subtraction operation using values from global memory, then calls another routine with a calculated parameter before restoring registers.
- `handle_input`: Performs bit manipulation on a register value, compares it with data in memory, and writes specific values to hardware color registers when certain conditions are met.
- `frag_movem_lea_movea_jsr_rts`: Short code fragment that saves registers on the stack, then loads the address of a location in the code into a0, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.
- `load_save_data`: Loads saved game data from a file if a condition is met, using parameters from global memory.
- `load_or_exit`: Saves all registers on the stack, reads a value from global memory, branches to another location if that value is non-zero, otherwise performs a file operation using parameters from global memory before jumping to a fixed address.
- `frag_move_pea_rts`: Short code fragment that copies an address to a fixed memory address, then pushes the address of a location in the code onto the stack, then copies 20153 to a fixed memory address, then copies the top of the stack to a fixed memory address, then returns.
- `frag_move_pea_rts_2`: Short code fragment that copies an address to a fixed memory address, then pushes the address of a location in the code onto the stack, then copies 20153 to a fixed memory address, then copies the top of the stack to a fixed memory address, then returns.
- `update_flags_and_call`: Updates flag bits in two memory locations, calls another routine, and executes a system function before returning.
- `read_potgo`: Saves selected registers on the stack, checks a bit in a hardware register, writes specific values to another hardware register, and conditionally stores a byte value to memory before restoring registers.
- `frag_lea_clr_movem_rts`: Short code fragment that loads the address of a location in the code into a0, then clears the memory at a0, then restores saved registers from the stack, then returns.
- `read_potgo_and_store`: Reads a bit from a hardware register and stores a byte value to memory location pointed by a0.
- `frag_lea_clr_movem_rts_2`: Short code fragment that loads the address of a location in the code into a0, then clears the memory at a0, then restores saved registers from the stack, then returns.
- `frag_movem_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_moveq_pea_movea_jmp`: Short code fragment that loads 0 into d0, then pushes the address of a fixed memory address onto the stack, then copies a location in the code to a2, then jumps to another point in the code.
