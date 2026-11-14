# Game 2506 (deliverance (1992)(21st century)[cr scx][t +1 zyr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02506). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2703360 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 35 of 35 named (0 author symbols), 35 of 35 described (1 by rule, 14 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 104 of 104 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 692 of 692 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2501 |
| code | 1558 |
| data | 2110501 |
| unused | 588800 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 1 of 3).adf:bootblock | 1 | 60 | 2 |
| logged disk load from Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 1 of 3).adf boot-block load | 1 | 409 | 28 |
| logged disk load from Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 1 of 3).adf boot-block load | 1 | 135 | 2 |
| logged disk load from Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 1 of 3).adf boot-block load | 1 | 88 | 3 |

## Cases handled

- deliverance (1992)(21st century)[cr scx][t +1 zyr](disk 1 of 3)/Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 1 of 3).adf: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- deliverance (1992)(21st century)[cr scx][t +1 zyr](disk 2 of 3)/Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 2 of 3).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- deliverance (1992)(21st century)[cr scx][t +1 zyr](disk 3 of 3)/Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 3 of 3).adf: custom-format (non-DOS) disk, 608 non-empty blocks: matched against track-loaded segments
- deliverance (1992)(21st century)[cr scx][t +1 zyr](disk 1 of 3)/Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 1 of 3).adf: the boot block reads 4608 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- deliverance (1992)(21st century)[cr scx][t +1 zyr](disk 1 of 3)/Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 1 of 3).adf: the boot block reads 1536 bytes from disk offset 2560 to an address (found statically); recovered as a unit
- deliverance (1992)(21st century)[cr scx][t +1 zyr](disk 1 of 3)/Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 1 of 3).adf: the boot block reads 1024 bytes from disk offset 4096 to an address (found statically); recovered as a unit
- deliverance (1992)(21st century)[cr scx][t +1 zyr](disk 1 of 3)/Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 1 of 3).adf: 3 logged loads cover 4608 bytes of the image
- 3 packed track region(s), 2106880 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 1 of 3).adf:bootblock

- `validate_memory_and_relocate_boot_data`: Traverses a memory list to verify available RAM bounds before calling a function to allocate a buffer.
- `initialize_io_request`: Sets up an I/O request structure with specific values for buffer size, address, and transfer length before calling the OS I/O handler.

### logged disk load from Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 1 of 3).adf boot-block load

- `initialize_engine_data_structures`: Initializes a series of memory blocks by copying data from fixed locations and writing specific instruction sequences into a target memory area.
- `relocate_relative_pointers`: Iterates through a list of offsets to update a table of pointers by adding a base address to each value.
- `concatenate_registers`: Combines two 32-bit registers into a single 64-bit return value after calling another routine twice.
- `decompress_bitstream_data`: Decompresses a custom bitstream-encoded data block into a destination memory buffer.
- `write_dma_control_constants`: Writes a sequence of four constant values to a global variable and modifies the DMA control register.
- `execute_bit_manipulation_sequence`: Performs a sequence of bitwise operations and status updates while waiting for specific bits to change in memory.
- `check_bit_and_branch`: Tests a bit in a data register and branches to a specific address if the bit is clear, otherwise continues execution.
- `test_and_negate`: Tests a word value in a data register and branches to a specific address if the value is positive, otherwise negates the value and continues execution.
- `update_status_and_poll`: Performs several subroutine calls, modifies bytes at specific offsets within a memory block, and waits for a bit to be set in a byte.
- `increment_and_toggle_bits`: Increments a global variable with a wrap-around check and toggles bits in a memory location while waiting for a bit to be set.
- `check_and_set_dma_flags`: Initializes several hardware registers with specific values, copies a long value from one location to another, checks for certain conditions in memory, and returns early if those conditions are met.
- `set_dma_flag`: Writes a specific value to a memory location and returns a predefined negative one, likely initializing a DMA control flag.
- `validate_and_process_data`: Validates data by comparing checksums and performing bit operations on byte pairs from a buffer, storing processed results into another memory location.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `wait_for_bit_and_clear`: Writes values to offsets of a pointer, waits for a specific bit in a memory location to become set, and then clears that bit.
- `initialize_disk_controller`: Sets specific bits in a memory location, calls another routine, clears and sets bits in the same location, calls the routine again, then sets more bits and waits for a status bit to clear.
- `manage_disk_controller_bits`: Increments a byte value in a data register, clears a specific bit in a memory location, sets a bit based on the incremented data register value, calls another routine, clears the same bit again, then waits for a status bit to clear.
- `set_multiple_bits_and_wait`: Sets several bits in a memory location, performs multiple subroutine calls to manipulate bits, and waits for a bit in another location to be set.
- `toggle_bit_and_wait`: Clears a bit in memory, calls another routine to wait for a condition, sets the same bit, then waits again before returning.
- `wait_for_bit_and_return`: Sets a global variable to zero and waits for a specific bit in a memory location to be cleared before returning a value.
- `wait_for_bit_set`: Checks if a specific bit in memory is set and loops until it becomes set, then returns immediately.
- `set_and_clear_bit`: Sets a bit in memory, calls another routine to wait for a condition, clears the same bit, then waits again before returning.
- `set_and_clear_second_bit`: Sets a second bit in memory, calls another routine to wait for a condition, clears that bit, then waits again before returning.
- `poll_status_bit_loop`: Calls another routine and loops while a specific bit in a memory location remains set.
- `save_registers_and_call`: Saves all general-purpose and address registers onto the stack, calls three other routines, restores the registers from the stack, and then returns.
- `initialize_graphics_system`: Initializes graphics hardware by setting up display registers, clearing screen memory, and configuring coprocessor pointers using data from tables.
- `read_joystick_status`: Reads joystick button states from hardware registers and stores either zero or 255 in a global variable based on the button conditions.
- `setup_interrupt_flags`: Writes 49152 to a memory location, modifies a word value from another location by setting bit 15, writes the modified value twice to different locations, copies a long value from one structure to another, and then returns.

### logged disk load from Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 1 of 3).adf boot-block load

- `initialize_and_copy_data`: Initializes various memory locations with specific values, copies data between addresses using loop structures, and performs conditional jumps based on register comparisons.
- `process_data_with_loop`: Processes data through multiple loops and conditional operations, manipulating word and byte values while updating memory locations based on comparison results.

### logged disk load from Deliverance (1992)(21st Century)[cr SCX][t +1 Zyr](Disk 1 of 3).adf boot-block load

- `initialize_graphics_system_copy`: Initializes graphics hardware by setting up display registers, clearing screen memory, and configuring coprocessor pointers using data from tables, identical to routine R7 but with different global variable references.
- `read_joystick_status_copy`: Reads joystick button states from hardware registers and stores either zero or 255 in a global variable based on the button conditions, identical to routine R8 but with different global variable references.
- `setup_interrupt_flags_2`: Writes 49152 to a memory location, modifies a word value from another location by setting bit 15, writes the modified value twice to different locations, copies a long value from one structure to another, and then returns.
