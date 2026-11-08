# Game 67 (Aaargh v1.0a 0511): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00067). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (845495 top-level bytes in 54 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 4 of 4 named (0 author symbols), 4 of 4 described (1 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 98004 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 73 of 73 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 744321 |
| code | 270 |
| data | 100887 |
| padding | 17 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Aaargh.Slave | 1 | 2 | 1 |
| ex | 3 | 71 | 3 |
| arg.bin (raw code file, load address unknown) | 1 | 0 | 0 |
| gwave.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| osemu.400 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- Aaargh_v1.0a_0511/Aaargh/data/arg.bin: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Aaargh_v1.0a_0511/Aaargh/data/gwave.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Aaargh_v1.0a_0511/Aaargh/data/osemu.400: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3 never-loaded region(s), 98004 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Aaargh.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### ex

- `init_library_and_fade`: Clears the color fade register and a global variable, opens a library by name, and stores the returned pointer.
- `loop_call_store`: Loops 180 times calling another routine and storing its result into a global variable, then returns.
- `wait_and_set_bit`: Waits until a byte at a global variable has bit 5 set, then writes 32 to the color fade register and returns that bit.
