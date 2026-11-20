# Game 138 (Aaargh!): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00138). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907120 top-level bytes in 53 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 56 of 56 named (0 author symbols), 56 of 56 described (25 by rule, 13 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 172 of 172 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 65884 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 851 of 851 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 734805 |
| code | 3994 |
| data | 138302 |
| padding | 13635 |
| unused | 16384 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Aaargh!.adf:ex | 3 | 77 | 4 |
| Aaargh!.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 750 | 47 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 4 |
| Aaargh!.adf:gwave.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| Aaargh!.adf:arg.bin (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 169 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Aaargh!/Aaargh!.adf:gwave.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Aaargh!/Aaargh!.adf:arg.bin: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 65884 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Aaargh!.adf:ex

- `init_and_open_file`: Initializes the system via library calls, opens a file, and returns 32 if it fails, otherwise performs cleanup and returns zero.
- `loop_call_store`: Loops 180 times calling another routine and storing its result into a global variable, then returns.
- `wait_bit_set_fade`: Waits in a loop until bit 5 of a global variable is set, then writes 32 to the color fade register and returns that bit.
- `delay_color_update`: Updates a color register with a value from a global variable, performing nested loops to create a delay before returning.

### Aaargh!.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `advance_random_state_2`: Advances a pseudo-random number generator by reading a global state variable, incrementing it when it reaches zero, shifting it left by one bit, and XORing it with a constant when negative, then stores the result back.
- `frag_andi_move_beq_bmi_bsr_bra`: Short code fragment that masks d5 with 127, then copies the memory at a3 (advancing a3) to d0, then branches if the last result was zero, then branches if the last result was negative, then calls another routine, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_cmpi_blt_addi_bra`: Short code fragment that compares d0 with 10, then branches if the comparison was less, then adds 55 to d0, then jumps to another point in the code.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (addi.w) and then falls through into the code that follows, with no return of its own.
- `bitwise_pixel_operation`: Performs bitwise OR and AND operations between pixel data from multiple memory locations and stores the results in corresponding positions of a fourth memory area.
- `frag_lea_move_bra`: Short code fragment that loads the address of a fixed memory address into a3, then copies 15 to d5, then copies 660 to a fixed memory address, then jumps to another point in the code.
- `frag_lea_move_clr_bsr_bra`: Short code fragment that loads the address of a fixed memory address into a3, then copies 24 to d5, then clears a fixed memory address, then calls another routine, then loads the address of a fixed memory address into a3, then copies 3 to d5, then copies 308 to a fixed memory address, then jumps to another point in the code.
- `frag_clr_bra`: Short code fragment that clears a fixed memory address, then jumps to another point in the code.
- `frag_lea_move_clr_bsr_bra_2`: Short code fragment that loads the address of a fixed memory address into a3, then copies 20 to d5, then clears a fixed memory address, then calls another routine, then loads the address of a fixed memory address into a3, then copies 20 to d5, then copies 352 to a fixed memory address, then jumps to another point in the code.
- `string_processing_loop`: Processes character strings by iterating through them until encountering a null terminator or negative value, calling another routine during each iteration.
- `frag_lea_move_clr_bsr_bra_3`: Short code fragment that loads the address of a fixed memory address into a3, then copies 0 to d5, then clears a fixed memory address, then calls another routine, then loads the address of a fixed memory address into a3, then copies 0 to d5, then copies 352 to a fixed memory address, then jumps to another point in the code.
- `frag_lea_move_clr_bsr_bra_4`: Short code fragment that loads the address of a fixed memory address into a3, then copies 0 to d5, then clears a fixed memory address, then calls another routine, then loads the address of a fixed memory address into a3, then copies 0 to d5, then copies 440 to a fixed memory address, then jumps to another point in the code.
- `frag_lea_move_clr_bra`: Short code fragment that loads the address of a fixed memory address into a3, then copies 20 to d5, then clears a fixed memory address, then jumps to another point in the code.
- `handle_interrupt_request`: Reads the interrupt request register and sets the copper list enable bit, the blitter enable bit, or the DMA enable bit depending on which interrupt flag is set, then calls the random generator, the audio routine, and the joystick routine before decrementing a counter and handling a blitter interrupt.
- `read_joystick_buttons`: Reads the joystick data register and sets bits of a result depending on which button bits are set, setting bit 0 for bit 1, bit 1 for bit 9, bit 2 for bit 1, and bit 3 for bit 9 when the corresponding conditions hold.
- `process_joystick_buttons`: Reads the joystick data register via read_joystick_buttons, then checks the button result bits and sets the corresponding flag byte at offset 292 of the structure in d1, setting the flag at offset 290 when bit 3 is set and the flag at offset 294 when bit 1 is set.
- `configure_audio_channel`: Writes an audio register, sets a timer period and a DMA enable word, then clears a global variable, configuring audio channel three.
- `frag_tst_bmi_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was negative, then returns.
- `frag_move_bra`: Short code fragment that copies 1500 to a fixed memory address, then jumps to another point in the code.
- `frag_move_clr_rts`: Short code fragment that copies an address to a fixed memory address, then copies a fixed memory address to a fixed memory address, then copies 1024 to a fixed memory address, then copies 32776 to a fixed memory address, then clears a fixed memory address, then returns.
- `frag_bsr_andi_addi_move_bra`: Short code fragment that calls advance_random_state, then masks d0 with 960, then adds 1024 to d0, then copies d0 to a fixed memory address, then copies 0 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies 5376 to a fixed memory address, then copies 0 to a fixed memory address, then jumps to another point in the code.
- `frag_tst_bmi_rts_2`: Short code fragment that tests a fixed memory address, then branches if the last result was negative, then returns.
- `audio_channel_setup`: Initializes and configures audio channel 1 registers including length, period, and DMA control settings for sound playback.
- `frag_tst_bmi_rts_3`: Short code fragment that tests a fixed memory address, then branches if the last result was negative, then returns.
- `frag_clr_move_bra`: Short code fragment that clears a fixed memory address, then copies 512 to a fixed memory address, then copies an address to a fixed memory address, then jumps to another point in the code.
- `frag_tst_bmi_rts_4`: Short code fragment that tests a fixed memory address, then branches if the last result was negative, then returns.
- `frag_clr_move_bra_2`: Short code fragment that clears a fixed memory address, then copies an address to a fixed memory address, then copies 4096 to a fixed memory address, then jumps to another point in the code.
- `frag_tst_bpl_bmi_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not negative, then tests a fixed memory address, then branches if the last result was negative, then returns.
- `audio_channel_init`: Sets up audio channel 1 with specific frequency and length values, then configures DMA control register to enable audio output.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `sound_effect_generator`: Generates sound effects by setting up audio channel 2 parameters including length, period, and DMA control registers with random pitch adjustments.
- `frag_move_rts_2`: Short code fragment that copies 1 to a fixed memory address, then copies 100 to a fixed memory address, then copies 4 to a fixed memory address, then copies an address to a fixed memory address, then copies a fixed memory address to a fixed memory address, then copies 448 to d0, then copies d0 to a fixed memory address, then copies 32772 to a fixed memory address, then returns.
- `frag_move_bra_3`: Short code fragment that copies 1 to a fixed memory address, then copies 40 to a fixed memory address, then copies 4 to a fixed memory address, then copies an address to a fixed memory address, then copies 384 to d0, then jumps to another point in the code.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `frag_move_movea_bsr_bra`: Short code fragment that copies 1 to a fixed memory address, then copies 45 to a fixed memory address, then copies 4 to a fixed memory address, then copies a fixed memory address to a0, then copies a fixed memory address to a2, then calls another routine, then copies 496 to d0, then jumps to another point in the code.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `frag_move_movea_bsr_bra_2`: Short code fragment that copies 1 to a fixed memory address, then copies 30 to a fixed memory address, then copies 4 to a fixed memory address, then copies a fixed memory address to a0, then copies a fixed memory address to a2, then calls another routine, then copies 320 to d0, then jumps to another point in the code.
- `frag_move_rts`: Short code fragment that copies d0 to a fixed memory address, then copies a fixed memory address to a fixed memory address, then copies 32772 to a fixed memory address, then returns.
- `frag_clr_tst_bpl_move_rts`: Short code fragment that clears a fixed memory address, then tests a fixed memory address, then branches if the last result was not negative, then tests a fixed memory address, then branches if the last result was not negative, then copies an address to a fixed memory address, then copies 10 to a fixed memory address, then copies 2 to a fixed memory address, then returns.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address, then copies 65535 to a fixed memory address, then returns.
- `update_audio_channels`: Checks a flag and then loads sound data from several tables into the audio registers, advancing per-channel pointers and decrementing their counters, while also configuring the noise channel and writing a random value to the third audio channel.
- `fill_table_with_random_bytes`: Writes the value 255 to the blitter OR plane register and then fills a 2047-byte table at offset an address with random bytes using a dbra loop that calls the random generator for each byte.
- `advance_random_state`: Reads a global variable, increments it when zero, shifts it left by one, XORs it with a constant when negative, and stores the result back.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `frag_lea_clr_move_addq_cmpa_blt_rts`: Short code fragment that loads the address of a fixed memory address into a1, then clears d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then adds 1 to d0, then compares a0 with a2, then branches if the comparison was less, then copies d0 to a fixed memory address, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_and_set_interrupt_bit`: Waits until bit 5 of the interrupt request status register is set, then sets bit 5 of the interrupt request register and returns.
- `call_super_state`: Wrapper that calls the exec library function SuperState and returns its result to the caller.
- `call_forbid`: Wrapper that calls the exec library function Forbid and returns its result to the caller.
- `call_permit`: Wrapper that calls the exec library function Permit and returns its result to the caller.
