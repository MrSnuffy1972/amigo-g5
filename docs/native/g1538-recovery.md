# Game 1538 (ghouls 'n ghosts (1989)(u. s. gold)[cr][t]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01538). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 15 of 15 named (0 author symbols), 15 of 15 described (3 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 66 of 66 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 297 of 297 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 1066 |
| data | 757604 |
| padding | 626 |
| unused | 141824 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t].adf:GhoulsNGhosts | 2 | 287 | 14 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t].adf:bootblock | 1 | 10 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t].adf:GhoulsNGhosts

- `handle_workbench_startup_message`: Program entry point that finds the current task and, when its command-line field is zero, waits on its message port and takes the startup message.
- `open_libraries_and_run_steps`: It opens two shared libraries via an exec open call, storing a pointer from one of them, then hands control to a routine that prints text and runs nine unnamed steps in sequence.
- `open_libraries_with_error_cleanup`: Opens two shared libraries through the exec open call and stores a pointer obtained from one of them.
- `close_window_and_libraries`: Cleanup routine that closes the window and then closes the two libraries, each only when its stored handle is nonzero.
- `print_text_then_run_steps`: Calls the intuition text printing function through a stored library base, then runs nine unnamed routines in sequence and makes one final library call.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_two_library_functions`: Calls two library functions one after the other through a base held in a register, at offsets the listing does not name.
- `dispatch_port_message_codes`: Message-loop handler stored in data that waits on a message port and takes messages until one arrives, then branches on a code field of the message.
- `poll_structures_and_dispatch`: It repeatedly calls two routines through offsets in a global structure until the second one returns a nonzero result.
- `dispatch_port_message_codes_2`: Message-loop handler that waits on a message port, takes messages until one arrives, then branches on a code field of the message.
- `print_intuition_text_item`: Calls the intuition text printing function through a stored library base and returns.
- `reply_to_exec_message`: Calls the exec reply-message function through a stored library base and returns.
- `setup_sprite_copper_display`: Pseudo-code could not be produced, so this rests on the listed facts only: it calls the supervisor-state function, writes the sprite pointer, copper list and display control registers, and refers to the text ARTHUR1.SPR. One routine calls it.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
