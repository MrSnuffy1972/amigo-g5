# Game 138 (Aaargh!): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00138). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907120 top-level bytes in 53 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 13 of 13 named (0 author symbols), 13 of 13 described (1 by rule, 12 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 117 of 117 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 65884 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 599 of 599 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 735491 |
| code | 2650 |
| data | 138963 |
| padding | 13632 |
| unused | 16384 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Aaargh!.adf:ex | 3 | 71 | 3 |
| Aaargh!.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 513 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| Aaargh!.adf:gwave.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| Aaargh!.adf:arg.bin (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 114 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
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

### Aaargh!.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `advance_random_state_2`: Advances a pseudo-random number generator by reading a global state variable, incrementing it when it reaches zero, shifting it left by one bit, and XORing it with a constant when negative, then stores the result back.
- `handle_interrupt_request`: Reads the interrupt request register and sets the copper list enable bit, the blitter enable bit, or the DMA enable bit depending on which interrupt flag is set, then calls the random generator, the audio routine, and the joystick routine before decrementing a counter and handling a blitter interrupt.
- `read_joystick_buttons`: Reads the joystick data register and sets bits of a result depending on which button bits are set, setting bit 0 for bit 1, bit 1 for bit 9, bit 2 for bit 1, and bit 3 for bit 9 when the corresponding conditions hold.
- `process_joystick_buttons`: Reads the joystick data register via read_joystick_buttons, then checks the button result bits and sets the corresponding flag byte at offset 292 of the structure in d1, setting the flag at offset 290 when bit 3 is set and the flag at offset 294 when bit 1 is set.
- `configure_audio_channel`: Writes an audio register, sets a timer period and a DMA enable word, then clears a global variable, configuring audio channel three.
- `update_audio_channels`: Checks a flag and then loads sound data from several tables into the audio registers, advancing per-channel pointers and decrementing their counters, while also configuring the noise channel and writing a random value to the third audio channel.
- `fill_table_with_random_bytes`: Writes the value 255 to the blitter OR plane register and then fills a 2047-byte table at offset an address with random bytes using a dbra loop that calls the random generator for each byte.
- `advance_random_state`: Reads a global variable, increments it when zero, shifts it left by one, XORs it with a constant when negative, and stores the result back.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_and_set_interrupt_bit`: Waits until bit 5 of the interrupt request status register is set, then sets bit 5 of the interrupt request register and returns.
