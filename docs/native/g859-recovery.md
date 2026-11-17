# Game 859 (air support (1992)(psygnosis)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00859). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1809552 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 31 raw-word lines |
| Functions named and described | pass, 130 of 130 named (0 author symbols), 130 of 130 described (35 by rule, 82 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 848 of 848 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 29184 bytes, 31 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4207 of 4289 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 32296 |
| code | 16098 |
| data | 1485683 |
| padding | 19 |
| unused | 275456 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf:bootblock | 1 | 16 | 1 |
| Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf:bootblock | 1 | 16 | 1 |
| logged disk load from Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf boot-block load | 1 | 94 | 3 |
| logged disk load from Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf boot-block load | 1 | 90 | 7 |
| logged disk load from Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf boot-block load | 1 | 88 | 4 |
| logged disk load from Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 480 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1275 | 63 |
| Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf bytes 6144-8192 (code never loaded by a reference run) | 1 | 0 | 0 |
| Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf bytes 12288-39424 (code never loaded by a reference run) | 1 | 2230 | 31 |

## Cases handled

- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1697 non-empty blocks: matched against track-loaded segments
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1281 non-empty blocks: matched against track-loaded segments
- 330 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: the boot block reads 22528 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: the boot block reads 4608 bytes from disk offset 867328 to an address (found statically); recovered as a unit
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: the boot block reads 22528 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: the boot block reads 1536 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 330 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 518 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: bytes 6144-8192 look like code no run loaded: recovered position-independently (load address unknown)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: bytes 12288-39424 look like code no run loaded: recovered position-independently (load address unknown)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: 2 logged loads cover 27136 bytes of the image
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 2-1123 (bytes 1024-575488) outside the filesystem hold data, not code (opcode density 0.004, entropy 7.18)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1146-1157 (bytes 586752-592896) hold ASCII text
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1163-1167 (bytes 595456-598016) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.50)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1173-1173 (bytes 600576-601088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1177-1184 (bytes 602624-606720) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.73)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1192-1200 (bytes 610304-614912) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.13)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1208-1215 (bytes 618496-622592) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.61)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1224-1231 (bytes 626688-630784) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.78)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1236-1693 (bytes 632832-867328) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.81)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1703-1715 (bytes 871936-878592) hold ASCII text
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: 4 logged loads cover 53248 bytes of the image
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 5-11 (bytes 2560-6144) hold ASCII text
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 80-332 (bytes 40960-170496) outside the filesystem hold data, not code (opcode density 0.052, entropy 6.01)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 338-338 (bytes 173056-173568) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.15)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 341-341 (bytes 174592-175104) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 345-345 (bytes 176640-177152) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 375-508 (bytes 192000-260608) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.35)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 511-517 (bytes 261632-265216) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.37)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 559-596 (bytes 286208-305664) outside the filesystem hold data, not code (opcode density 0.047, entropy 6.08)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 610-611 (bytes 312320-313344) outside the filesystem hold data, not code (opcode density 0.031, entropy 3.13)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 613-633 (bytes 313856-324608) outside the filesystem hold data, not code (opcode density 0.024, entropy 5.36)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 688-725 (bytes 352256-371712) outside the filesystem hold data, not code (opcode density 0.050, entropy 6.08)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 739-740 (bytes 378368-379392) outside the filesystem hold data, not code (opcode density 0.031, entropy 3.15)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 742-762 (bytes 379904-390656) outside the filesystem hold data, not code (opcode density 0.024, entropy 5.36)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 774-814 (bytes 396288-417280) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.47)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 936-999 (bytes 479232-512000) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.78)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 1080-1154 (bytes 552960-591360) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.35)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 1200-1274 (bytes 614400-652800) outside the filesystem hold data, not code (opcode density 0.008, entropy 4.19)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 1320-1715 (bytes 675840-878592) hold ASCII text
- 1 never-loaded region(s), 2048 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_palette_and_loop` (runtime segment an address): Configures DMA and interrupts, calls two routines, and if successful, populates several memory locations with values from a table and configures bitplane and copper registers; then it performs a loop calling another routine multiple times based on a bit test. Kind: disk hardware/sync check. Pass path: the loop continues (u6__sub_06DC returns and the loop repeats).
- `calculate_bitstream_parameters_and` (runtime segment an address): Calculates a value using multiplication and subtraction, calls decompress_bitstream_data, and performs a bitwise toggle on bit 3 of a hardware register via a temporary load. Kind: disk hardware/parameter check. Pass path: the branch where d4 == 0 (the 'bne' is not taken).
- `decompress_bitstream_data` (runtime segment an address): Calculates a target value from a register, updates a byte in a structure, and configures custom hardware registers. Kind: bitstream decompression with custom sync/checksum validation. Pass path: The loop continues through the bitstream processing (u6__l0_08E8) and eventually reaches the return statement CONCAT44(in_D0,in_D1) after the data is successfully reconstructed and copied..
- `adjust_byte_value` (runtime segment an address): Adjusts a byte value in a structure until it matches a derived target, sets or clears bits in a global variable based on the lowest bit of an input value. Kind: disk hardware/timing check. Pass path: the branch where uVar2 == *(byte *)(unaff_A3 + 5).
- `configure_interrupt_settings` (Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf bytes 12288-39424 (code never loaded by a reference run)): Iterates through a loop to call three subroutines, then sets two 16-bit values at offsets 250 and 254 of a structure in a6 before returning. Kind: sabotage path. Pass path: u8__sub_09D8() is called.
- `configure_hardware_and_interrupts` (Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf bytes 12288-39424 (code never loaded by a reference run)): Saves registers to the stack, performs several memory and hardware register writes, calls multiple routines, and eventually jumps to another routine. Kind: disk hardware/track check. Pass path: sub_08FA.

## Functions


### Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf:bootblock

- `perform_dma_transfer_and_jump`: Sets up and executes two DMA transfers using the exec.DoIO routine, first transferring data from a memory location to a hardware address and then transferring data from a different memory location to another hardware address, before jumping to a new location.

### Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf:bootblock

- `configure_dma_and_load_code`: Configures two DMA transfers using a structure in a1 to move data from a source to a destination, calls a routine at a specific address, updates a global variable, and then performs a second DMA transfer before jumping to a new location.

### logged disk load from Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf boot-block load

- `write_decrementing_global_until_wrap`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf boot-block load

- `unpack_bitstream_data_2`: Reads a variable number of bits from a bit buffer to reconstruct bytes, which are then stored in a destination memory area using a pointer.
- `frag_moveq_bra`: Short code fragment that loads 8 into d1, then loads 8 into d4, then jumps to another point in the code.
- `branch_on_bit_count`: Loads 2 into d1, calls another routine, and then branches to different locations based on whether the value in d2 is less than 2, equal to 2, or equal to 3.
- `unpack_bitstream_data`: Reads a variable number of bits from a bit buffer to reconstruct bytes, which are then stored in a destination memory area using a pointer.
- `write_decrementing_global_until_wrap_2`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf boot-block load

- `decompress_and_copy_data`: Reads a byte from a source, performs bitwise shifts, and enters a loop to call another routine and copy bytes to a destination buffer.
- `test_d5_then_return`: Tests whether the D5 register shifted right by one is nonzero, then returns in either case without doing anything else.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_and_loop_data`: Performs a bitwise right shift on d5 using a loop controlled by d0, while simultaneously performing a rotate-left through the carry bit into d1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_dma_and_dispatch`: Sets the DMACON register, clears three bytes in a global variable, and calls another routine; if the returned value in d4 is not zero, it performs a second call to another routine with specific values in d0 and d1 before jumping to an address stored at an offset of a global variable.
- `initialize_hardware_and_process`: Disables interrupts, configures DMA and custom registers, calls two routines, and if successful, populates several memory locations with values from a table and configures bitplane and copper registers before entering an infinite loop.
- `frag_move_cmpi_bne_lea_rts`: Short code fragment that copies 7 to a fixed memory address, then compares a fixed memory address with 7, then branches if the last result was not zero, then loads the address of a fixed memory address into a6, then copies 32767 to the memory at an offset from a6 (3 times in a row), then copies 0 to a fixed memory address, then returns.
- `update_palette_and_loop`: Configures DMA and interrupts, calls two routines, and if successful, populates several memory locations with values from a table and configures bitplane and copper registers; then it performs a loop calling another routine multiple times based on a bit test.
- `frag_clr_tst_beq_rts`: Short code fragment that clears a fixed memory address, then tests a fixed memory address, then branches if the last result was zero, then returns.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `frag_movem_bsr_st_move_rte`: Short code fragment that saves registers on the stack, then calls another routine (2 times in a row), then sets a fixed memory address to all ones, then restores saved registers from the stack, then copies 32 to a fixed memory address, then returns from the exception.
- `increment_and_lookup_index`: Calculates the difference between two 16-bit values in global variables, increments the second value if the difference is positive, and then performs a table lookup using a nibble from the updated value to store a new 16-bit word into a third global variable.
- `update_color_palette`: Reads a value from a global variable to select one of four bit-shifting and masking modes, then copies sixteen words from a table to the hardware color registers.
- `wait_for_input_and_decompress_bitstream`: Configures CIA timers and interrupts to wait for a hardware signal, then performs bitstream decompression by bit-manipulating data from a source table and writing the resulting bytes into a destination buffer via a loop.
- `write_port_sequence_clearing_bits`: Writes six port values in turn to the same byte register at a fixed offset, each with a different single bit cleared, and ends with all bits set.
- `frag_btst_beq_moveq_bsr_bra`: Short code fragment that tests bit 4 of the memory at an offset from a4, then branches if the last result was zero, then loads 1 into d0, then calls another routine, then jumps to another point in the code.
- `frag_clr_rts`: Short code fragment that clears a fixed memory address, then returns.
- `frag_move_ori_bra`: Short code fragment that copies a fixed memory address to d1, then merges 128 into d1, then jumps to another point in the code.
- `toggle_cia_timer_output`: Loops a specified number of times, alternating between writing inverted bitwise values to a table at offset 256 of a pointer in a6 and configuring CIAA timer registers to wait for an interrupt bit.
- `calculate_bitstream_parameters_and`: Calculates a value using multiplication and subtraction, calls decompress_bitstream_data, and performs a bitwise toggle on bit 3 of a hardware register via a temporary load.
- `decompress_bitstream_data`: Calculates a target value from a register, updates a byte in a structure, and configures custom hardware registers.
- `adjust_byte_value`: Adjusts a byte value in a structure until it matches a derived target, sets or clears bits in a global variable based on the lowest bit of an input value.
- `frag_bset_bra`: Short code fragment that sets bit 1 of the memory at a1, then jumps to another point in the code.
- `configure_hardware_registers`: Clears specific bits in a byte, sets another bit, writes constant values to multiple memory offsets, and waits for a bit to be set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_via_table`: Loops while a 16-bit value at offset 1692 of a structure in a5 is non-zero and certain bits in a table and the POTGOR register are set; if conditions are met, it stores a value from offset 1606 of the structure, clears offset 1444, and performs an indirect jump using a value from a table.
- `frag_move_bmi_jsr_bra`: Short code fragment that copies the memory at a4 (advancing a4) to d0, then branches if the last result was negative, then copies a location in the code to d0, then calls another routine, then jumps to another point in the code.
- `frag_tst_bne_clr_bra`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was not zero, then clears the memory at an offset from a5, then jumps to another point in the code.
- `generate_dynamic_vertex_table_2`: Decrements a 16-bit value at offset 1672 of a structure in a5 and, if certain hardware and structure conditions are met, calls another routine and updates several offsets in the structure.
- `swap_structure_pointers`: Swaps three 32-bit values between offsets 5ac, 5b0, and 5b8 of a structure in a5, then returns.
- `frag_move_movea_jmp`: Short code fragment that copies 9984 to the status register, then copies a fixed memory address to a7, then copies a fixed memory address to a0, then jumps to the address in a0.
- `clear_and_test_word`: Clears a 16-bit word at an offset of 1412 from a5 and returns if the resulting value is zero, otherwise it returns.
- `generate_dynamic_vertex_table`: Constructs a series of 32-bit coordinate pairs into a buffer by performing arithmetic on values from a structure in a5, including bitwise operations and scaling.
- `scale_rgb_values`: Clears a 16-bit value at offset 578 of a structure in a5, then enters a loop that calls another routine to scale RGB components based on a count derived from a value at offset 652.
- `frag_move_bsr_rts`: Short code fragment that copies 2 to the memory at an offset from a5, then calls another routine, then returns.
- `frag_move_bmi_bra_rts`: Short code fragment that copies the memory at a4 (advancing a4) to d0, then branches if the last result was negative, then copies d0 to the memory at an offset from a5, then copies 4 to the memory at an offset from a5, then jumps to another point in the code, then copies 2 to the memory at an offset from a5, then returns.
- `frag_move_rts`: Short code fragment that copies the memory at a4 (advancing a4) to the memory at an offset from a5, then returns.
- `frag_move_rts_2`: Short code fragment that copies the memory at a4 (advancing a4) to the memory at an offset from a5, then copies 6 to the memory at an offset from a5, then returns.
- `clear_and_fill_structure`: Clears a 16-bit word at one offset in a5, copies three consecutive 16-bit values from a pointer in a4 to three other offsets in a5, and clears another 16-bit word at a different offset in a5, then returns.
- `frag_move_rts_3`: Short code fragment that copies the memory at a4 (advancing a4) to the memory at an offset from a5 (2 times in a row), then returns.
- `frag_move_rts_4`: Short code fragment that copies the memory at a4 (advancing a4) to the memory at an offset from a5 (2 times in a row), then copies the memory at a4 (advancing a4) to d0, then copies d0 to the memory at an offset from a5 (2 times in a row), then copies the memory at a4 (advancing a4) to the memory at an offset from a5 (3 times in a row), then returns.
- `execute_jump_table`: Loads a base address into a2 from a table, uses a value from a4 to index into that table, and then performs an indirect jump using a value from a table indexed by a value loaded from a4.
- `store_word_at_indexed_offset`: Looks up an offset from a table indexed by the first word addressed by the pointer, then stores the second word at that offset in the slot record.
- `frag_lea_movea_move_bra`: Short code fragment that loads the address of a location in the code into a0, then copies a0 to a1, then copies the memory at a4 (advancing a4) to d0, then loads the address of an indexed entry from a0 into a0, then copies the memory at a4 (advancing a4) to d0, then loads the address of an indexed entry from a1 into a1, then jumps to another point in the code.
- `frag_tst_bne_rts`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was not zero, then returns.
- `compare_offset_value`: Calculates a value by subtracting a word from a table entry from another table entry, then compares this result against a 16-bit value in a structure in a5 and returns if the comparison conditions are met.
- `frag_move_rts_5`: Short code fragment that copies 20 to the memory at an offset from a5, then returns.
- `frag_move_lea_eori_rts`: Short code fragment that copies the memory at a4 (advancing a4) to d0, then loads the address of a location in the code into a0, then copies an indexed entry from a0 to the memory at an offset from a5, then exclusive-ors 4 into d0, then copies an indexed entry from a0 to the memory at an offset from a5, then returns.
- `frag_move_rts_6`: Short code fragment that copies the memory at a4 (advancing a4) to the memory at an offset from a5 (2 times in a row), then returns.
- `frag_move_rts_7`: Short code fragment that copies the memory at a4 (advancing a4) to d0, then copies d0 to the memory at an offset from a5, then returns.
- `rotate_structure_pointers`: Rotates three 32-bit values at offsets 5ac, 5b0, and 5b4 of a structure in a5, clears a 16-bit value at offset 5c0, and returns.
- `scale_rgb_components`: Processes three nibbles from a word at a0 by multiplying each by a value in d0, scaling the result, clamping it to a maximum of 15, and storing the combined 16-bit result at a pointer in a1.
- `initialize_structure_pointers`: Clears two 16-bit values in a structure, sets a pointer at offset 1030 to the value of a pointer in a0 plus 6, copies a 16-bit value from offset 2 of a0 to offset 1032 of the structure, and calls three different routines if the first word of a0 is non-negative.
- `update_structure_pointers_2`: Copies a 32-bit pointer from offset 6 to offset 1742 of a structure in a5, writes a 16-bit value from offset 4 to offset 1698, and calls another routine if the value at offset 0 is not negative.
- `synchronize_structure_state`: Waits until a 32-bit value at offset 1720 of the structure in a5 is zero, then clears two 16-bit values at offsets 1662 and 1656, performs a series of 16-bit and 32-bit word/long moves to copy data between various offsets, swaps two 32-bit values at offsets 1460 and 1468, and calls two other routines before returning.
- `copy_data_with_scaling`: Copies data from a source at a0 to a destination at a1 using a scaling factor calculated from a value in a structure in a5, performing block copies for specific bit patterns and using a loop for the remainder.
- `process_data_stream`: Increments a 16-bit value at offset 1656 of a structure in a5 and, depending on values at offsets 1662, 1676, and 1654, either calls another routine to decompress data or clears a value at offset 1678.
- `decompress_data_stream`: Reads a stream of bytes from a0 and writes to a1, handling various control codes for literal copies, offset jumps via a table in a5, and repeated byte sequences.
- `process_and_store_data`: Reads two words from a0, updates several values in a structure in a5 based on these words, copies 16 words to a destination at a2, and then writes a series of 32-bit zeros to a calculated memory location based on a value from a0.
- `decompress_bitstream_data_2`: Calculates bitstream parameters and stores them in a structure, copies sixteen longwords from a source to a destination, and then performs a variable-length bitstream decompression into a buffer.
- `expand_run_length_encoded_data`: Reads run-length encoded data from a source buffer in a0 and expands it into a destination buffer in a1 using a bitmask-based decoding scheme.
- `frag_movem_lea_bra`: Short code fragment that saves registers on the stack, then loads the address of the memory at an offset from a5 into a0, then jumps to another point in the code.
- `frag_movem_lea_bra_2`: Short code fragment that saves registers on the stack, then loads the address of the memory at an offset from a5 into a0, then jumps to another point in the code.
- `frag_movem_lea_bra_3`: Short code fragment that saves registers on the stack, then loads the address of the memory at an offset from a5 into a0, then jumps to another point in the code.
- `update_structure_pointers`: Sets several fields in a structure at offset 208 of a5 using d1, d4, and values from a table in a2, then updates a pointer in that structure to point to a global variable plus an offset from a2, and finally sets a bit in a value at offset 128 of a5.
- `frag_move_rts_8`: Short code fragment that copies the memory at a4 (advancing a4) to the memory at an offset from a5, then returns.
- `setup_object_management`: Writes a constant to offset 1602 of a structure in a5, calls another routine, populates several offsets of a5 and a6 with pointers and constants, and registers three different routines into a table before returning.
- `calculate_status_flags`: Performs bitwise manipulations on a 16-bit value from offset 12 of a6 to determine two 16-bit values, which are then stored at offsets 1436 and 1438 of a5.
- `adjust_coordinate_values`: Updates two 16-bit values at offsets 1440 and 1442 of a5 by calculating differences between a byte from offset 10 of a6 and bytes from offsets 1448 and 1450 of a5, while clamping the results to specific ranges.
- `execute_conditional_jump`: Saves registers, checks bit 14 of a 16-bit value from offset 28 of a6, and if bit 6 and bit 4 of a masked version of that value are both zero, it increments a 16-bit value at offset 1444 of a structure in a5 and performs an indirect jump using a value from a table.
- `update_bit_and_counter`: Calculates a bit position using a 16-bit value from offset 1740 of a structure in a5, stores a bitmask in offset 1744 of the structure, and if the 16-bit value at offset 1738 is negative, it writes specific bytes to offset 256 of a0 and increments offset 1754 of the structure.
- `update_entity_timers_and_flags`: Checks status bits in a structure pointed to by a0 and modifies various 16-bit counters and state bytes within a structure pointed to by a5.
- `frag_move_ori_bra_2`: Short code fragment that copies the memory at an offset from a5 to d1, then merges 128 into d1, then jumps to another point in the code.
- `update_entity_state_and_timers`: Modifies a byte at an offset from a0 using bitwise operations, calls another routine, and updates several fields in a structure pointed to by a6 including timers and coordinate values.
- `frag_movea_move_rts`: Short code fragment that copies a location in the code to a0, then copies 127 to the memory at an offset from a0, then copies 136 to the memory at an offset from a0, then copies 0 to the memory at an offset from a0, then returns.
- `process_input_state`: Checks bits in a value from offset 28 of a6 and performs bitwise operations on a byte from a memory location to update a byte at offset 3584 and a value at offset 1446 of a5.
- `frag_moveq_movea_move_bsr_dbra_rts`: Short code fragment that loads 0 into d1, then copies a location in the code to a0, then copies the memory at an offset from a0 to d1, then calls another routine, then loops back while counting down d1, then returns.
- `increment_conditional_value`: Checks a 32-bit value at offset 1602 of a5; if bit 23 is not set and bit 1 is not set, it stores the value doubled; otherwise, if bit 1 is set, it increments the value and stores it back at offset 1602.
- `manage_object_list`: Selects a pointer based on bits in offset 28 of a6, then either decrements a counter at offset 8 of the pointed-to structure and updates a linked list, or clears several fields in the structure.
- `process_and_call_subroutines`: If a 16-bit value at offset 128 of a structure in a5 is non-zero, it sets two 16-bit values in a6 using that value and its bit-shifted version, clears the value at offset 128, and calls another routine four times with different offsets from a5.
- `update_accumulator_and_counters`: Updates a 16-bit value at offset 10 of a structure in a0 by adding values from offsets 10 and 12, then updates a 16-bit value at offset 6 of a structure in a1 using values from offsets 14 and 16 of a0.
- `clear_structure_fields_2`: Calls another routine four times, each time passing a structure offset from a5 and a destination pointer in a1, to write specific 16-bit and 8-bit values into the destination.
- `clear_structure_fields`: Clears two 16-bit words and two bytes at specific offsets within a structure pointed to by a0.
- `search_and_transform_data_2`: Saves registers to the stack, calls another routine to search for data, and if the result in d0 is zero, copies two pointers from the result into global variables before returning.
- `search_and_transform_data`: Loops through a list of words until a specific value is found, then performs bitwise transformations on data from the list and stores the results into a destination buffer at an offset from a0.
- `frag_ori_lsl_addq_move_rts`: Short code fragment that merges 96 into d1, then shifts d0 left by 3, then adds 2 to d0, then copies d0 to the memory at an offset from a5, then copies d0 to the memory at an offset from a6, then returns.
- `frag_move_rts_9`: Short code fragment that copies the memory at an offset from a5 to the memory at an offset from a6, then returns.
- `frag_move_lsr_tst_ori_btst_beq_rts`: Short code fragment that copies 127 to the memory at an offset from a0, then copies d0 to the memory at an offset from a0, then shifts d0 right by 8, then copies d0 to the memory at an offset from a0, then tests the memory at an offset from a0, then merges 25 into the memory at an offset from a0, then tests bit 0 of the memory at an offset from a0, then branches if the last result was zero, then returns.

### Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf bytes 12288-39424 (code never loaded by a reference run)

- `process_input_and_update_camera_2`: Branches to another routine, performs several memory and hardware register writes, calls another routine, and contains several loops that manipulate data and hardware registers.
- `copy_color_palette`: Copies 32 words from a source table to the hardware color registers starting at the address in a0.
- `configure_interrupt_settings`: Iterates through a loop to call three subroutines, then sets two 16-bit values at offsets 250 and 254 of a structure in a6 before returning.
- `configure_interrupt_registers`: Sets various bits in the interrupt mask and enable registers by performing bitwise operations on d0 and d1 and writing the results to hardware registers.
- `clear_table_entry`: Calculates an offset into a table using the lower two bits of d0, shifts the result, and clears the 16-bit word at that location.
- `configure_hardware_and_interrupts`: Saves registers to the stack, performs several memory and hardware register writes, calls multiple routines, and eventually jumps to another routine.
- `update_hardware_and_structures`: Checks a global variable, writes values to hardware registers and various memory locations, calls several routines, and performs arithmetic on values from a global variable before returning.
- `check_and_call_routines`: Checks a longword at a specific address and, if non-zero, calls two different routines before returning.
- `run_nested_interpolation`: Sets a global variable to negative one and executes a nested loop that calls another routine one hundred and twenty-one times.
- `interpolate_two_points`: Calculates a base address using d6 and d7, then calls another routine twice with different pairs of pointers derived from that address.
- `calculate_interpolated_coordinate`: Checks bit flags in two structures to determine an offset for a global pointer, then performs a complex fixed-point interpolation calculation using values from the structures before calling another routine.
- `extract_and_store_bits`: Extracts bitwise components from two 16-bit values read from hardware registers and stores them into several consecutive word locations in a global table, then enters a nested loop that calls two different routines based on a global flag.
- `check_collision_and_copy`: Calculates a base address using d6 and d7, calls another routine twice with specific pointer offsets, and then copies a 16-bit word from a global variable to another global variable.
- `validate_and_check_collisions`: Performs multiple range and bitwise checks on values derived from d6 and d7 and global tables, then calls another routine twice with specific pointer offsets if the checks pass.
- `evaluate_entity_collision_states`: Checks if three entities have positive values at offset 2304 and exceed a threshold of 2000 at offset 5184, then updates a global variable and a secondary global variable based on shared bit flags in their status words.
- `update_structure_elements`: Iterates 144 times through a structure in a6, performing arithmetic on values at various offsets and writing results to offsets 2880, 3456, and 5184, or setting bit 15 of the value at offset 2304.
- `calculate_scaled_products`: Fetches two 16-bit values from tables at a1 and a2 using an index in d6, multiplies them by d0 and d1 respectively, and returns the results in d4 and d5 as 32-bit values.
- `process_input_and_update_state`: Clears four words at specific offsets, performs bitwise operations on the value from a hardware register to update several global words, and uses various conditional branches to modify values in a structure and hardware registers.
- `combine_bit_fields`: Combines several bit-manipulated values derived from a table and a global variable into a single 32-bit result returned in d0.
- `update_data_tables`: Copies three longwords from specific memory locations to a global variable, performs arithmetic on values from hardware registers, and uses a loop to call another routine and update a table of values.
- `scale_and_swap_values`: Multiplies d0 and d1 by zero, adds them to values derived from d0 and d1, and performs swaps on the resulting 32-bit registers before returning.
- `load_pointers_and_call`: Loads two addresses into a1 and a2 and calls another routine.
- `process_scaled_data`: Clears a memory block using a loop, updates a global value, and enters a nested loop that performs calculations using values from a table and hardware registers to update a structure at offset 2304 of a1.
- `execute_nested_loops`: Calculates a value based on hardware registers and enters a nested loop that calls another routine eleven times.
- `copy_data_block`: Calculates an index using d5 and d6, fetches a pointer from a table at 26e9c, and copies eight 32-bit words from that pointer to a destination at a1.
- `calculate_and_store_interpolated_values_2`: Writes the word 24000 to an offset of 76 of a structure, loads the address of a table into a5, and calls another routine before returning.
- `calculate_and_store_interpolated_values`: Performs complex arithmetic on values from two tables, stores the results in a buffer, calls a routine via a longword pointer, and then executes a loop that calls two different routines.
- `execute_scaled_calls`: Enters a loop that calls another routine eight times, incrementing a pointer by ten each iteration.
- `process_scaling_parameters`: Updates several global variables based on a value from a structure, then performs two separate calls to another routine after modifying a global pointer.
- `process_conditional_scaling`: Performs division-based scaling on two sets of values from a structure in a0 and, depending on bit tests of values at offsets 6 and 16, calls another routine with a pointer from a global variable.
- `clear_three_words`: Writes three 16-bit zero values to the memory locations pointed to by a1.
