# Game 2696 (angel nieto pole 500 (1990)(opera sport)[cr cool-x]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02696). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905984 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 17 of 17 named (0 author symbols), 17 of 17 described (5 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 428 of 428 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 45056 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 882 of 882 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 33629 |
| code | 3402 |
| data | 837718 |
| padding | 3 |
| unused | 31232 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf:bootblock | 1 | 63 | 2 |
| logged disk load from Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 279 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 540 | 6 |
| Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf bytes 1024-4096 (code never loaded by a reference run) | 1 | 0 | 0 |
| Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf bytes 4608-8704 (code never loaded by a reference run) | 1 | 0 | 0 |
| Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf bytes 107008-125952 (code never loaded by a reference run) | 1 | 0 | 0 |
| Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf bytes 369152-388096 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: custom-format (non-DOS) disk, 1697 non-empty blocks: matched against track-loaded segments
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: the boot block reads 32256 bytes from disk offset 865280 to an address (found statically); recovered as a unit
- runtime segment an address: 123 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 305 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: bytes 1024-4096 look like code no run loaded: recovered position-independently (load address unknown)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: bytes 4608-8704 look like code no run loaded: recovered position-independently (load address unknown)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: bytes 107008-125952 look like code no run loaded: recovered position-independently (load address unknown)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: bytes 369152-388096 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 9 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 9 immediate
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: 5 logged loads cover 77312 bytes of the image
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 18-207 (bytes 9216-106496) outside the filesystem hold data, not code (opcode density 0.021, entropy 5.37)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 247-248 (bytes 126464-127488) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 250-276 (bytes 128000-141824) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.14)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 293-404 (bytes 150016-207360) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.80)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 407-423 (bytes 208384-217088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.78)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 425-425 (bytes 217600-218112) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.00)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 427-439 (bytes 218624-225280) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.30)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 442-451 (bytes 226304-231424) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.78)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 453-453 (bytes 231936-232448) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 455-481 (bytes 232960-246784) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.65)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 483-495 (bytes 247296-253952) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.46)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 497-501 (bytes 254464-257024) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.86)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 503-506 (bytes 257536-259584) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.94)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 508-719 (bytes 260096-368640) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.43)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 759-760 (bytes 388608-389632) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.95)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 762-788 (bytes 390144-403968) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.42)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 805-829 (bytes 412160-424960) outside the filesystem hold data, not code (opcode density 0.006, entropy 4.97)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 831-935 (bytes 425472-479232) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.41)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 937-986 (bytes 479744-505344) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.12)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 989-990 (bytes 506368-507392) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.80)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 992-1001 (bytes 507904-513024) outside the filesystem hold data, not code (opcode density 0.004, entropy 2.15)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 1003-1004 (bytes 513536-514560) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.82)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 1006-1007 (bytes 515072-516096) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.26)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 1009-1689 (bytes 516608-865280) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.16)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 1753-1754 (bytes 897536-898560) outside the filesystem hold data, not code (opcode density 0.018, entropy 3.67)
- angel nieto pole 500 (1990)(opera sport)[cr cool-x]/Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf: blocks 1756-1759 (bytes 899072-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.97)
- 4 never-loaded region(s), 45056 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Angel Nieto Pole 500 (1990)(Opera Sport)[cr Cool-X].adf:bootblock

- `initialize_hardware_2`: Configures Amiga hardware registers including display and DMA settings, performs multiple system library calls to set up the operating environment, and initializes memory areas through a loop that copies data from a source to a destination.
- `initialize_hardware`: Sets up hardware registers with specific values and performs multiple library calls to configure system state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `frag_btst_bne_move_rts`: Short code fragment that tests bit 14 of a fixed memory address, then branches if the last result was not zero, then copies a fixed memory address to a fixed memory address, then copies an address to a fixed memory address, then copies 0 to a fixed memory address (2 times in a row), then copies 10644 to a fixed memory address, then returns.
- `swap_and_update_pointers`: Exchanges two 32-bit values stored in global variables, then writes the original value and its increments to specific offsets in a memory area pointed to by a0.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (or.b) and then falls through into the code that follows, with no return of its own.
- `wait_for_vertical_blank`: Waits for the vertical blank period by polling the vertical position register until a specific value is detected, then reads the joystick status byte and loops while the fire button bit is set.
- `frag_lea_addi_rts`: Short code fragment that loads the address of a fixed memory address into a0, then adds 4 to the memory at a0 (advancing a0) (2 times in a row), then adds 0 to the memory at a0, then returns.
- `lookup_and_store_values`: Reads three word values from a table, applies bitwise AND operations with a global variable, multiplies the results by two, then uses these as offsets to read words from two different tables and stores them into specific memory locations.
- `process_and_transform_data`: Processes arrays of short integers by performing arithmetic operations on their elements, updates values in multiple memory locations based on mathematical transformations, and stores the results in another array with specific calculations involving multiplication and division.
- `update_and_process_data`: Decrements counters in global variables and updates data pointers based on character comparisons, with conditional resets and value assignments depending on specific character matches.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `disable_audio_playback`: Clears the audio playing flag and zeroes four audio channel period registers.
- `initialize_audio_dma`: Initializes audio DMA control structures for four channels, setting up pointers, lengths, periods, and volumes while configuring interrupt and DMA enable registers.
- `update_dma_control_structure`: Updates a control structure by reading data from a table using an index and bit-shift, then writes values to various offsets within the structure, including updating the DMA control register via a global variable.
- `process_command_stream`: Iterates through a command stream, executing operations based on opcode bytes to update DMA control registers, load pointers from a table, modify bitfields, and adjust counters within a structure.
- `validate_and_set_bit`: Validates a short value against a global variable and triggers an exception if the condition fails, then sets a specific bit in two memory locations using bitwise operations with input values.
