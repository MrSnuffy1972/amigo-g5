# Game 2175 (great giana sisters, the (1988)(time warp)[cr rsi][a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02175). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901520 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 7 of 7 named (0 author symbols), 7 of 7 described (0 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 17 of 17 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 79 of 79 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 78158 |
| code | 398 |
| data | 777396 |
| unused | 45568 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf:bootblock | 1 | 12 | 1 |
| logged disk load from Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf boot-block load | 1 | 32 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 35 | 4 |

## Cases handled

- great giana sisters, the (1988)(time warp)[cr rsi][a3]/Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf: custom-format (non-DOS) disk, 1669 non-empty blocks: matched against track-loaded segments
- great giana sisters, the (1988)(time warp)[cr rsi][a3]/Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf: the boot block reads 81920 bytes from disk offset 337920 to an address (found statically); recovered as a unit
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- great giana sisters, the (1988)(time warp)[cr rsi][a3]/Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf: 1 logged loads cover 81920 bytes of the image
- great giana sisters, the (1988)(time warp)[cr rsi][a3]/Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf: blocks 2-654 (bytes 1024-335360) hold 2 packed streams (BK: ByteKiller) found by scanning, 89% of the bytes
- great giana sisters, the (1988)(time warp)[cr rsi][a3]/Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf: blocks 657-659 (bytes 336384-337920) outside the filesystem hold data, not code (opcode density 0.040, entropy 4.33)
- great giana sisters, the (1988)(time warp)[cr rsi][a3]/Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf: blocks 820-880 (bytes 419840-451072) outside the filesystem hold data, not code (opcode density 0.014, entropy 2.14)
- great giana sisters, the (1988)(time warp)[cr rsi][a3]/Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf: blocks 902-1693 (bytes 461824-867328) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.82)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `issue_io_write_cia_port` (bootblock): Boot block entry of twelve instructions that issues an Exec DoIO request while writing the CIA B port register and the DMA control register. Kind: disk hardware/timing check. Pass path: The branch taken when the CIA port write/read sequence completes without triggering a hardware exception or timing mismatch (likely the instruction immediately following the call if it leads to the main loader)..
- `poll_input_and_wait` (runtime segment an address): Checks the status of joystick fire buttons and disk status via CIA registers and executes a countdown wait. Kind: disk hardware/sync check. Pass path: the instruction immediately following the call (the code continues execution normally).

## Functions


### Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf:bootblock

- `issue_io_write_cia_port`: Boot block entry of twelve instructions that issues an Exec DoIO request while writing the CIA B port register and the DMA control register.

### logged disk load from Great Giana Sisters, The (1988)(Time Warp)[cr RSi][a3].adf boot-block load

- `write_interrupt_enable_register`: Nine-instruction routine reached through an absolute address load that writes the interrupt enable register; a set top bit enables the selected interrupts and a clear one disables them.
- `call_fixed_routine_then_store`: Three-instruction routine that makes one call to a fixed-address routine, then stores a constant into a global variable; the listed facts also note a DMA control register touch.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_input_and_wait`: Checks the status of joystick fire buttons and disk status via CIA registers and executes a countdown wait.
- `wait_while_cia_bit_set`: Three-instruction routine that loops while one bit of a global status variable remains set.
- `return_without_action`: Routine whose shown body is only a return, so it does no work in the pseudo-code.
- `busy_wait_word_countdown`: Counts a word from a fixed starting value down until it wraps, a busy-wait loop that other routines call.
