# Game 1338 (pipe mania (1989)(empire)[cr oracle][t +3 setrox][b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01338). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907040 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 91 of 91 named (31 author symbols), 91 of 91 described (6 by rule, 76 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1114 of 1114 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 205196 bytes, 3 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2320 of 2322 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 8172 |
| data | 297578 |
| padding | 2530 |
| unused | 598528 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf:Oracle | 2 | 83 | 3 |
| type | 2 | 117 | 2 |
| clr | 3 | 19 | 1 |
| Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf:shit | 2 | 39 | 3 |
| Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf:Load PipeMania | 2 | 718 | 33 |
| Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf:Setrox | 1 | 155 | 3 |
| Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 218 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 377 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 52 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 499 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf:PipeMania (raw code file, load address unknown) | 1 | 33 | 3 |

## Cases handled

- runtime segment an address (play-rs-m1500): 47 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 328 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 52 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 454 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- pipe mania (1989)(empire)[cr oracle][t +3 setrox][b]/Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf:PipeMania: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- pipe mania (1989)(empire)[cr oracle][t +3 setrox][b]/Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.77)
- pipe mania (1989)(empire)[cr oracle][t +3 setrox][b]/Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf:Setrox: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 205196 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf:Oracle

- `initialize_and_dispatch_tasks`: As the program entry point it performs initial setup, then invokes two routines to process data through a short sequence and a slightly longer sequence of operations.
- `run_short_task`: A short five-instruction routine that is invoked by a single unnamed caller, performing a minimal operation on its operands.
- `process_multi_step`: An eleven-instruction routine called by one unnamed routine that processes data through a slightly longer sequence of operations.

### type

- `initialize_and_dispatch_tasks_2`: As the program entry point it performs initial setup, then invokes a short dispatch routine and a slightly longer dispatch routine by taking their addresses.
- `dispatch_task_entry`: A two-instruction routine that is entered by address rather than by a call, serving as a small jump target within the task dispatch sequence.

### clr

- `open_and_close_library`: The program entry point that opens a library through exec.OpenLibrary and later closes it via exec.CloseLibrary, with no parameters required.

### Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf:shit

- `copy_byte_and_read_cia`: Copies a byte from a location addressed by a register to another location addressed by a different register, then reads the CIA parallel-port register and the joystick/mouse port, storing the resulting values back through pointers.
- `register_handler_pointer`: A thirteen-instruction routine whose address is taken via an absolute LEA and held as a handler pointer in data, then dispatched by a single caller.
- `read_cia_io_ports`: A thirteen-instruction routine that reads and writes CIAA_PRA and POTGOR, accessing the joystick and mouse fire button and disk status lines.

### Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf:Load PipeMania

- `start`: Initializes the machine by requesting memory, configuring the blitter and display registers, enabling interrupts and DMA, then shows the title screen and waits for a frame.
- `loadaddr`: Small routine of 1 instruction that merges 0 into d0.
- `waitframe`: Waits for the next vertical retrace by polling the vertical position register until it reaches a fixed value, then returns.
- `clss`: A three-instruction routine that clears a block of title screen memory by tail-branching to another routine.
- `titleclsblk`: It clears a block of memory by writing a byte from a register through a pointer, repeating the store for each of 22 consecutive steps, and it is used to wipe the title screen and to shrink or grow the logo.
- `dologo`: The routine is a 95-instruction 68000 subroutine that manipulates a block of data through a pointer, branching on values and writing bytes and words to memory.
- `growlogo`: A thirteen-instruction routine that draws a logo, clears a title block, and waits for a frame, invoked by a single caller.
- `shrinklogo`: A twelve-instruction routine that draws a logo, clears a title block, and waits for a frame, invoked by a single caller.
- `showtitles`: It touches the COLOR00 hardware register, then calls the logo routine, a clear routine and a frame-wait routine to sequence the title display.
- `copytobkgnd`: A three-instruction routine that copies data to the background by tail-branching to another routine.
- `copybkgnd`: A two-instruction routine that copies a small amount of data and is called by a single routine that draws a logo.
- `copyit`: Copies a block of source bytes into a destination buffer by repeating a byte-store through incrementing pointers until a counter reaches zero.
- `cls`: Fills a region of memory with a fixed byte value by storing through a pointer in a loop that runs a fixed number of times.
- `genripple`: The routine loads an address from a table via a PC-relative move into a register, then writes a value through that register's pointer to set up the data.
- `plotripple`: A fourteen-instruction routine that plots a ripple effect by invoking another routine to draw a line.
- `plotshrink`: Loops over a table of 27 entries, calling another routine for each one to draw a shrinking line, then returns.
- `plotline`: Draws a straight line by stepping along its length and writing successive pixels through a pointer, using a large instruction sequence to handle the geometry.
- `doletters`: Invokes a single-letter processing routine repeatedly to render successive characters of a string.
- `doletters2`: Invokes a single-letter processing routine repeatedly to render a second sequence of characters.
- `doletter`: Copies a byte from a table through a pointer and stores it via another pointer, then decrements that second pointer, repeating the copy for a fixed count of 22 instructions.
- `do_moveto`: Updates a movement target by storing new coordinates through a pointer before the calling routine positions the next character.
- `do_vto`: A short author routine that branches into the do_vto12 routine via a tail branch to continue the conversion sequence.
- `do_vto2`: A fourteen-instruction author routine that branches into the do_vto12 routine via a tail branch and invokes one additional unnamed routine.
- `do_vto_intermediate`: A four-instruction routine entered by a branch from do_vto2 that then falls through into the do_vto12 routine.
- `do_vto12`: Draws a filled shape by calling a rectangle routine, reached from several branching callers.
- `do_hto`: Stores a byte from a register into a byte pointed to by a pointer, then increments that pointer by one.
- `do_hto2`: do_hto2 is a fourteen-instruction routine entered via an author symbol that branches to do_hto and then falls through into do_hto_intermediate.
- `do_hto_intermediate`: A four-instruction routine entered by a branch from do_hto2 that then falls through into the do_hto routine.
- `do_hto12`: A fifteen-instruction author routine that branches into the do_hto routine via a tail branch to continue the conversion sequence.
- `do_ret`: Empty routine: returns to the caller immediately without doing anything.
- `rectangle`: Draws a rectangle outline by calling a routine that draws a single thin side, then draws the opposite side, and returns to the caller.
- `thinrect`: A routine named thinrect that consists of exactly 11 instructions and is reached via an author symbol, called by another routine named rectangle.
- `allogo`: It opens the dos.library library, clears and copies screen buffers, then drives a series of logo effects such as letter building, ripple and shrink animations while waiting for the raster frame.

### Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf:Setrox

- `initialize_and_compute_values`: As the program entry point it touches the COLOR16 hardware register, then repeatedly invokes the intermediate-value calculation routines to obtain results for their callers.
- `compute_intermediate_value_2`: Computes an intermediate value by invoking a calculation routine that itself calls three others and returns results to multiple callers.
- `compute_intermediate_value`: Performs a calculation by calling three other routines and returning a result to each of the five callers that invoke it.

### Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_runtime_environment`: Copies a block of code to a new location, clears a large memory buffer, populates a table with specific bit patterns, and executes a series of indirect calls and jumps.
- `execute_indirect_jump`: Performs an indirect jump to an address stored in a jumptable.
- `initialize_memory_blocks`: Copies a block of code to a new location, clears a large memory range, and fills another memory area with specific patterns.
- `initialize_runtime_buffers`: Updates a global state variable and copies a block of code to a new memory location.
- `initialize_memory_blocks_variant`: Copies a block of code to a new location and performs extensive initialization of multiple memory buffers with specific values.
- `increment_and_check_value`: Increments a word in a global variable and branches to another routine if the new value equals four.
- `search_memory_table`: Clears a word in memory and then loops through a table to compare values, incrementing a counter and a pointer during the process.
- `decrement_linked_list_elements`: Traverses a linked list of structures, decrementing specific bit-fields within the elements and updating a global variable based on a counter.
- `decrement_and_traverse_list`: Decrements bits in a value at a pointer and traverses a linked list of structures until a null pointer is encountered.
- `configure_blitter_transfer`: Calculates parameters and repeatedly writes to hardware registers to perform a series of bitplane transfers.
- `decrement_and_test_byte`: Subtracts a constant value from a byte stored at a memory location and checks if the resulting value equals a specific byte.
- `copy_data_tables`: Searches for a specific byte in a table and then copies data from one memory location to another based on the found index.
- `execute_callback_and_set_pointer`: Calls a function via a pointer, then updates a global variable with a value from a memory location and writes a constant to a specific offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trigger_software_interrupt`: Stores the current routine's address into a global variable and executes a software trap.
- `execute_repeated_calls`: Calls another routine five consecutive times.
- `copy_and_call_routine`: Copies a block of code from one location to another and then executes a function through a pointer stored in a global variable.
- `initialize_font_and_graphics_tables`: Clears several memory buffers with a constant value and populates a font data table by copying character glyphs from a source table to a destination buffer.
- `copy_string_font_data`: Iterates through a sequence of bytes to copy blocks of data from a table to a destination buffer.
- `copy_word_pairs_to_buffer`: The routine copies word pairs from D0 into the buffer pointed to by A0.
- `wait_for_non_zero_byte`: Repeatedly calls another routine until a global variable is non-zero, then performs bitwise shifts and concatenations on that variable.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `update_score_display`: Updates a numeric value and performs a string-based redraw of a text element.
- `update_animation_frames`: The routine increments a palette index with wrap-around and copies eight 16-bit palette entries from a table to a fixed destination.
- `copy_data_block`: Checks a byte value and, if it is zero, performs a block copy of data from one memory location to another using a loop.
- `extract_data_from_list`: Compares a byte in a register against two values and, depending on the result, increments a pointer and copies bytes from a source to a destination or a register.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d4, then merges 0 into (a4)+, then merges 0 into d0.
- `initialize_hardware_settings`: Opens a library, modifies a global variable, and manipulates hardware registers including DMA and interrupt enable bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_offset_2`: Performs a short relative branch to a specific location within another routine.
- `branch_to_offset`: Performs a short branch to a specific offset.
- `clamp_and_store_values`: Clamps a value to a maximum of sixty-four and stores it in a global variable, then stores the lower four bits of another register into a different global variable.
- `init_channel_from_stream`: Clears a channel field and loads a period value from the stream pointer, scales it by a shift into a second field, then zeroes the remaining channel fields.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_input_interrupt`: The routine calls initiate_emotion_sprites and then calls a function pointer stored in the data segment.
- `call_routine_and_update_flag`: Calls a routine via a pointer stored in a global variable and then updates that global variable to a constant value.
- `initiate_emotion_sprites`: The routine initializes a counter and optionally sets it from a RAM byte when a RAM word and byte match specific values.
- `trigger_game_over`: The routine calls update_sprite_positions and returns the value in d1.
- `update_sprite_positions`: The routine updates sprite position data by iterating over three sprite entries, writes the computed values to custom chip registers via DMACON, and conditionally changes the sprite offset.
- `process_input_events`: The routine iterates over a set of input slots, reads per-slot bytes from RAM and updates a per-slot control block in RAM. It copies and combines values into the block, writes to CIAA_PRA via the custom chip registers, and sets a dirty flag when a field changes.
- `update_sprite_attribute_data`: Updates sprite position, attribute, and color data by fetching values from a table based on an index.
- `set_player_sprite_offset`: The routine writes a 16-bit value from a table indexed by a byte in D4 to the word at offset six from the pointer in A1.
- `update_animation_counters`: Iterates through a list of structures and updates three independent animation counters in each using tables.
- `update_animation_frames_2`: Updates three independent animation counters stored in the structure pointed to by A2 using tables pointed to by A3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### Pipe Mania (1989)(Empire)[cr Oracle][t +3 Setrox][b].adf:PipeMania (raw code file, load address unknown)

- `enable_interrupts_dma`: Enables interrupts and DMA channels by writing to their control registers, then invokes two stored routine addresses from a table.
- `store_address_in_table`: Its address is taken via a PC-relative LEA and stored in a table, where it is later invoked by two other routines.
- `store_address_table`: An eighteen-instruction routine whose address is taken via a PC-relative LEA and stored in a table, then invoked by a single unnamed caller.
