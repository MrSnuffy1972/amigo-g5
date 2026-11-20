# Game 3612 (fears (1995)(manyk)(aga)[cr hlm][pre-release]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03612). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906064 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 55 of 55 named (0 author symbols), 55 of 55 described (38 by rule, 13 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 414 of 414 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1073 of 1073 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 20534 |
| code | 3734 |
| data | 776322 |
| padding | 2 |
| unused | 105472 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf:bootblock | 1 | 9 | 1 |
| logged disk load from Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf boot-block load | 1 | 194 | 37 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 870 | 17 |

## Cases handled

- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: custom-format (non-DOS) disk, 1552 non-empty blocks: matched against track-loaded segments
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: the boot block reads 20480 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 414 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: 1 logged loads cover 20480 bytes of the image
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: blocks 42-138 (bytes 21504-71168) hold 1 packed streams (CrM2: Crunch-Mania LZH-mode) found by scanning, 82% of the bytes
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: blocks 151-222 (bytes 77312-114176) hold 1 packed streams (CrM2: Crunch-Mania LZH-mode) found by scanning, 98% of the bytes
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: blocks 245-321 (bytes 125440-164864) hold 1 packed streams (CrM2: Crunch-Mania LZH-mode) found by scanning, 98% of the bytes
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: blocks 338-444 (bytes 173056-227840) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.14)
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: blocks 1016-1071 (bytes 520192-548864) hold 1 packed streams (CrM2: Crunch-Mania LZH-mode) found by scanning, 97% of the bytes
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: blocks 1086-1096 (bytes 556032-561664) outside the filesystem hold data, not code (opcode density 0.012, entropy 7.14)
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: blocks 1110-1119 (bytes 568320-573440) hold 1 packed streams (CrM2: Crunch-Mania LZH-mode) found by scanning, 94% of the bytes
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: blocks 1133-1188 (bytes 580096-608768) hold 1 packed streams (CrM2: Crunch-Mania LZH-mode) found by scanning, 97% of the bytes
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: blocks 1203-1216 (bytes 615936-623104) hold 1 packed streams (CrM2: Crunch-Mania LZH-mode) found by scanning, 89% of the bytes
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: blocks 1227-1235 (bytes 628224-632832) hold 1 packed streams (CrM2: Crunch-Mania LZH-mode) found by scanning, 93% of the bytes
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: blocks 1250-1260 (bytes 640000-645632) hold 1 packed streams (CrM2: Crunch-Mania LZH-mode) found by scanning, 90% of the bytes
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: blocks 1273-1327 (bytes 651776-679936) hold 1 packed streams (CrM2: Crunch-Mania LZH-mode) found by scanning, 96% of the bytes
- fears (1995)(manyk)(aga)[cr hlm][pre-release]/Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf: blocks 1344-1759 (bytes 688128-901120) hold ASCII text
- 1 packed track region(s), 266752 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf:bootblock

- `setup_io_request`: Initializes fields of an I/O request structure with specific values and performs a device I/O operation using the DoIO library function.

### logged disk load from Fears (1995)(Manyk)(AGA)[cr HLM][pre-release].adf boot-block load

- `frag_movea_move_sub_cmpa_bcs_subq_bne_b`: Short code fragment that copies a location in the code to a1, then copies a location in the code to d0, then subtracts a1 from d0, then compares a0 with a1, then branches if the carry flag is set, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then subtracts 4 from d0, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_adda_move_subq_bne_bra`: Short code fragment that adds d0 to a0, then adds d0 to a1, then copies the memory just below a0 (moving a0 down) to the memory just below a1 (moving a1 down), then subtracts 4 from d0, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_subq_move_lea_moveq_jmp`: Short code fragment that subtracts 1 from d6, then copies d6 to d3, then loads the address of a location in the code into a6, then loads 8 into d4, then jumps to the address in a4.
- `frag_move_dbra_lea_bra`: Short code fragment that copies d6 to the memory just below a2 (moving a2 down), then loops back while counting down d3, then loads the address of a location in the code into a6, then jumps to another point in the code.
- `frag_tst_beq_bra`: Short code fragment that tests d6, then branches if the last result was zero, then jumps to another point in the code.
- `frag_moveq_lea_jmp`: Short code fragment that loads 3 into d4, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_addi_bra`: Short code fragment that adds 6 to d6, then jumps to another point in the code.
- `frag_moveq_lea_jmp_2`: Short code fragment that loads 10 into d4, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_moveq_bra`: Short code fragment that loads 16 into d4, then jumps to another point in the code.
- `load_and_process_data`: Loads data from multiple memory locations using address pointers, processes bits from a data stream, and stores results into global variables.
- `frag_cmp_bcs_beq_lea_jmp`: Short code fragment that compares d6 with 6, then branches if the carry flag is set, then branches if the last result was zero, then loads the address of a location in the code into a6, then jumps to the address in a5.
- `frag_lea_jmp`: Short code fragment that loads the address of a location in the code into a6, then jumps to the address in a5.
- `frag_cmp_bcs_beq_lea_moveq_jmp`: Short code fragment that compares d6 with 2, then branches if the carry flag is set, then branches if the last result was zero, then loads the address of a location in the code into a6, then loads 1 into d4, then jumps to the address in a4.
- `frag_moveq_jmp`: Short code fragment that loads 2 into d4, then jumps to the address in a4.
- `frag_moveq_bra_2`: Short code fragment that loads 3 into d3, then jumps to another point in the code.
- `frag_moveq_tst_beq_bra`: Short code fragment that loads 1 into d3, then tests d6, then branches if the last result was zero, then jumps to another point in the code.
- `frag_tst_beq_lea_bra`: Short code fragment that tests d6, then branches if the last result was zero, then loads the address of a location in the code into a6, then jumps to another point in the code.
- `frag_tst_beq_bra_2`: Short code fragment that tests d6, then branches if the last result was zero, then jumps to another point in the code.
- `frag_cmp_bcs_lea_jmp`: Short code fragment that compares d6 with 3, then branches if the carry flag is set, then loads the address of a location in the code into a6, then jumps to the address in a5.
- `frag_cmp_bcs_lea_jmp_2`: Short code fragment that compares d6 with 3, then branches if the carry flag is set, then loads the address of a location in the code into a6, then jumps to the address in a5.
- `frag_cmp_bcs_moveq_lea_jmp`: Short code fragment that compares d6 with 3, then branches if the carry flag is set, then loads 8 into d4, then loads 10 into d3, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_add_lea_bra`: Short code fragment that adds d6 to d3, then loads the address of a location in the code into a6, then jumps to another point in the code.
- `frag_tst_beq_lea_bra_2`: Short code fragment that tests d6, then branches if the last result was zero, then loads the address of a location in the code into a6, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 8 into d4, then loads 2 into d2, then jumps to another point in the code.
- `frag_moveq_move_bra`: Short code fragment that loads 9 into d4, then copies 258 to d2, then jumps to another point in the code.
- `frag_tst_beq_moveq_move_lea_jmp`: Short code fragment that tests d6, then branches if the last result was zero, then loads 10 into d4, then copies 770 to d2, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_add_movea_adda_move_dbra_bra`: Short code fragment that adds d2 to d6, then copies a2 to a6, then adds d6 to a6, then copies the memory just below a6 (moving a6 down) to the memory just below a2 (moving a2 down), then loops back while counting down d3, then jumps to another point in the code.
- `frag_moveq_bra_4`: Short code fragment that loads 7 into d3, then jumps to another point in the code.
- `frag_moveq_bra_5`: Short code fragment that loads 4 into d3, then jumps to another point in the code.
- `frag_moveq_cmp_bcs_beq_bra`: Short code fragment that loads 2 into d3, then compares d6 with 1, then branches if the carry flag is set, then branches if the last result was zero, then jumps to another point in the code.
- `frag_moveq_bra_6`: Short code fragment that loads 0 into d6, then jumps to another point in the code.
- `frag_lsl_beq_roxl_dbra_jmp`: Short code fragment that shifts d5 left by 1, then branches if the last result was zero, then rotates d6 left through the extend flag by 1, then loops back while counting down d4, then jumps to the address in a6.
- `frag_cmpa_beq_move_roxl_bra`: Short code fragment that compares a0 with a1, then branches if the last result was zero, then copies the memory just below a0 (moving a0 down) to d5, then copies d3 to the memory at an offset from a3, then rotates d5 left through the extend flag by 1, then jumps to another point in the code.
- `copy_and_compare_bytes`: Copies bytes from one memory area to another while comparing addresses and performing conditional operations based on byte values.
- `frag_move_movea_jmp`: Short code fragment that copies d0 to the memory just below a2 (moving a2 down), then copies a location in the code to a2, then copies 33280 to the memory at an offset from a4, then copies 49152 to the memory at an offset from a4, then jumps to the address in a2.
- `frag_move_subq_bne_bra`: Short code fragment that copies the memory just below a0 (moving a0 down) to d2, then copies the memory just below a0 (moving a0 down) to d1, then copies d1 to the memory just below a2 (moving a2 down), then subtracts 1 from d2, then branches if the last result was not zero, then copies d1 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_moveq_move_bra_2`: Short code fragment that loads 0 into d1, then copies the memory just below a0 (moving a0 down) to d2, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_ori_bset_rts_btst_beq`: Short code fragment that merges 6 into d3, then sets bit d1 of an indexed entry from a6, then returns, then tests bit 5 of a fixed memory address, then branches if the last result was zero, then returns.
- `frag_tst_bne_move_subi_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then copies 1 to a fixed memory address (2 times in a row), then copies 700 to a fixed memory address, then subtracts 1 from a fixed memory address, then returns.
- `copy_pattern_data`: Copies pattern data from a source table to a destination buffer, handling different data offsets and incrementing counters.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_movem_bsr_move_rte`: Short code fragment that saves registers on the stack, then calls another routine, then copies 32 to a fixed memory address, then restores saved registers from the stack, then returns from the exception.
- `initialize_audio_and_input_state`: Sets up audio volume registers and initializes a table of pointers and values.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `increment_and_check_flag`: Increments a global variable and checks if it exceeds a threshold value, clearing the variable and branching based on the comparison result.
- `update_audio_channels`: Configures audio period values for multiple channels based on global state and table lookups, updating hardware registers AUD0LCH through AUD3LCH.
- `update_entity_state`: Processes entity data from a buffer, updates position and state variables based on input values, and configures audio and DMA settings.
- `process_input_value`: Processes input data based on its lower nibble, performing bit manipulation and conditional operations on global variables and memory locations.
- `lookup_and_adjust_value`: Searches a table using an input value to find a matching entry, adjusts the result based on flags, and stores the adjusted value in a global variable.
- `adjust_and_lookup_value`: Adjusts a value based on input parameters and performs arithmetic operations to determine a new value.
- `calculate_and_store_value`: Calculates a value using bit manipulation and arithmetic operations, then stores the result in memory.
- `process_input_byte`: Processes a byte from input data based on its lower nibble, performing bit manipulation and conditional operations on global variables and memory locations.
- `store_low_nibble`: Extracts the lower nibble from a byte value and stores it in a global variable.
- `advance_sample_pointer_on_wrap`: Adds a table value to a counter when the high nibble of a field is non-zero.
