# Game 4506 (necronom (1991)(linel)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04506). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 5 of 5 named (0 author symbols), 5 of 5 described (0 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 105 of 105 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 190 of 190 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 806 |
| data | 841946 |
| unused | 58368 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Necronom (1991)(Linel).adf:bootblock | 1 | 190 | 5 |

## Cases handled

- necronom (1991)(linel)/Necronom (1991)(Linel).adf: custom-format (non-DOS) disk, 1644 non-empty blocks: matched against track-loaded segments
- 3 packed track region(s), 841728 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_disk_controller` (bootblock): Configures disk controller registers and initializes hardware states including drive status, interrupt settings, and data transfer parameters. Kind: disk track check. Pass path: the routine initializes the disk controller and performs a series of disk operations including setting up DSKLEN, DSKPTH, ADKCON, and DSKSYNC, then waits for an interrupt. If the check passes (i.e., the correct track is read), it continues .

## Functions


### Necronom (1991)(Linel).adf:bootblock

- `copy_and_execute_boot_code`: Copies a block of code from one memory location to another and then executes it, using a loop to transfer data in 32-bit chunks.
- `setup_disk_controller`: Initializes disk controller registers by loading addresses and values into specific memory locations, then performs a subroutine call before jumping to another address.
- `initialize_disk_controller`: Configures disk controller registers and initializes hardware states including drive status, interrupt settings, and data transfer parameters.
- `compute_shifted_division`: Performs a division operation on a value from memory, shifts the result, and returns a combined value with a conditional adjustment based on comparison.
- `frag_move_andi_lsl_or_rts`: Short code fragment that copies the memory at an offset from a0 to d0, then copies the memory at an offset from a0 to d1, then masks d0 with an address, then masks d1 with an address, then shifts d0 left by 1, then merges d1 into d0, then returns.
