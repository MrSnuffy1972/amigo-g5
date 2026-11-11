# Game 2206 (it came from the desert (1989)(mirrorsoft)(disk 1 of 3)[h esi]): code recovery summary

## Functions (part 2 of 2)

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)
(continued)

- `modify_memory_via_calculated_offset_10`: Calculates a memory address using a base register and a value derived from two registers, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_via_calculated_pointer`: Calculates a target address using an offset and a value derived from multiple registers, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_via_calculated_pointer_2`: Calculates a target address using an offset and a value derived from two registers, then performs a bitwise OR operation on the byte at that location.
- `modify_table_entry_via_offset_2`: Calculates a memory address using a base pointer and an index scaled by eight, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_byte_via_offset_2`: Calculates a target address using a base pointer, a register, and a multiplier, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_byte_via_offset_3`: Calculates a destination address using a base pointer, a multiplier, and a constant offset, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_via_calculated_offset_11`: Calculates a destination address using a base pointer and a value derived from two registers, then performs a bitwise OR operation on the byte at that location.
- `modify_byte_via_complex_index_4`: Calculates an address using a combination of a base pointer, a bitwise operation on two registers, and a multiplier, then performs a bitwise OR on the byte at that location.
- `modify_byte_via_simple_index_3`: Performs a bitwise OR on a byte at a memory location calculated by adding a scaled integer to a base pointer.
- `modify_byte_via_simple_index_4`: Updates a byte at a calculated address by applying a bitwise OR operation using a value derived from a register and a base pointer.
- `modify_byte_via_simple_index_5`: Applies a bitwise OR to a byte located at an offset determined by a register-scaled value and a base pointer.
- `modify_byte_via_simple_index_6`: Stores a bitwise ORed result into a byte at a memory address calculated from a base pointer and a scaled register value.
- `modify_byte_via_complex_index_5`: Uses a complex index involving a bitwise XOR and concatenation of registers to target a byte for a bitwise OR operation.
- `modify_byte_via_complex_index_6`: Calculates a target address through register manipulation and bitwise operations to perform a bitwise OR on a single byte.
- `modify_byte_via_complex_index_7`: Modifies a byte by performing a bitwise OR at an address derived from a base pointer and a multi-register calculation.
- `modify_byte_via_complex_index_8`: Targets a byte for a bitwise OR operation using an address computed from a base pointer and a combination of register values.
- `modify_byte_via_complex_index_9`: Performs a bitwise OR on a byte at a memory location determined by a complex calculation involving a base pointer and register-based indexing.
- `modify_byte_via_offset_2`: Performs a bitwise OR operation on a byte at a memory location calculated from a base address and a scaled index.
- `modify_byte_via_complex_index_10`: Performs a bitwise OR operation on a byte at a memory location calculated using a base address and a complex index derived from two registers.
- `modify_memory_via_calculated_offset_12`: Calculates a destination address using a base register and a value derived from two other registers, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_via_offset_3`: Calculates an address using a base pointer, a register, and a multiplier, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_byte_via_offset_4`: Calculates a target address using a base pointer, a register value, and a constant offset to perform a bitwise OR operation on a single byte.
- `modify_memory_via_calculated_offset_13`: Calculates a destination address using a base pointer, an index, and a scaling factor.
- `modify_memory_via_calculated_offset_14`: Calculates a target address using a base register and a value derived from several registers, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_via_calculated_pointer_3`: Calculates a destination address using an offset and a value derived from two registers, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_via_calculated_offset_15`: Calculates a destination address using a base pointer, a multiplier, and an offset, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_via_calculated_pointer_4`: Calculates an address using a base register, a multiplier, and an offset, then performs a bitwise OR operation on the byte at that location.
- `modify_byte_via_calculated_pointer`: Calculates an address using a base register and a value derived from other registers, then performs a bitwise OR on the byte at that location.
- `update_byte_with_mask`: Performs a bitwise OR operation on a byte at a pointer calculated from several registers.
- `apply_bitwise_or_to_memory`: Uses a complex calculation involving multiple registers to determine a memory address and applies a bitwise OR to the byte stored there.
- `set_byte_bits_2`: Calculates a target address through register manipulation and sets specific bits in the byte at that address.
- `write_masked_byte`: Determines a memory location using a base pointer and an offset derived from register values, then modifies the byte using a bitwise OR.
- `modify_memory_byte`: Calculates a pointer based on the contents of several registers and performs a bitwise OR on the byte at that address.
- `modify_byte_simple_offset`: Calculates a memory address by adding a scaled integer to a base register and performs a bitwise OR on the byte at that location.
- `perform_bitwise_or_on_byte`: Calculates a destination address from register data and applies a bitwise OR to the byte at that memory location.
- `update_memory_via_pointer`: Computes a memory address using a combination of registers and modifies the byte at that address with a bitwise OR.
- `apply_mask_to_byte`: Calculates a pointer through register-based arithmetic and performs a bitwise OR on the byte at the resulting address.
- `modify_byte_via_index_2`: Performs a bitwise OR operation on a byte at a memory location calculated using a base address and an index.
- `modify_byte_via_complex_index_11`: Performs a bitwise OR operation on a byte at a memory location calculated using a base address and a complex index derived from multiple registers.
- `modify_byte_via_index_3`: Updates a byte by applying a bitwise OR to a memory address determined by an offset and a scaled index.
- `modify_byte_via_complex_index_12`: Applies a bitwise OR to a byte at a destination address calculated from a base register and a multi-register index.
- `modify_byte_via_index_4`: Stores a bitwise ORed value into a byte at a memory location calculated from a base pointer and an index.
- `modify_byte_via_index_5`: Performs a bitwise OR on a byte located at an address calculated using a base address and a scaled index.
- `modify_byte_via_index_6`: Calculates a memory address using a base and an index, then performs a bitwise OR on the byte at that location.
- `modify_byte_via_complex_index_13`: Modifies a byte using a bitwise OR at an address derived from a base register and a complex index calculation.
- `modify_byte_via_index_7`: Uses a base address and a scaled index to locate a byte and performs a bitwise OR on it.
- `modify_byte_via_index_8`: Performs a bitwise OR on a byte at a memory address calculated from a base address and an index value.
- `modify_byte_via_complex_index_14`: Calculates a pointer using a base address, an offset, and a value derived from bitwise operations on two registers, then performs a bitwise OR on the byte at that location.
- `modify_memory_via_calculated_pointer_5`: Calculates a destination address using an offset and a value derived from registers, then performs a bitwise OR operation to set specific bits in that memory location.
- `modify_byte_via_simple_index_7`: Calculates a pointer using a base address and a scaled integer offset, then performs a bitwise OR on the byte at that location.
- `modify_memory_via_calculated_pointer_6`: Calculates a destination address using an offset and a value derived from the contents of two registers, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_via_calculated_offset_16`: Calculates a target address using a base pointer and a value derived from multiple registers, then performs a bitwise OR operation on the byte at that location.
- `modify_table_entry_via_index`: Calculates an address by applying an offset and a scaled index to a base pointer, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_byte_via_index`: Calculates an address using a base pointer, an index, and a constant offset, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_via_calculated_offset_17`: Calculates a memory address using an index and an offset, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_via_calculated_pointer_7`: Calculates an address using a base pointer, an index, and a scale factor, then performs a bitwise OR operation on the byte at that location.
- `modify_memory_byte_via_offset_5`: Calculates a destination address using a base pointer, a register value, and a constant offset, then performs a bitwise OR operation on the byte at that location.
- `modify_byte_via_complex_index_15`: Calculates an address using a combination of a base register, a constant, and a value derived from two other registers, then performs a bitwise OR on the byte at that location.
- `modify_byte_via_complex_index_16`: Performs a bitwise OR on a byte at a memory location determined by a complex calculation involving multiple registers.
- `modify_byte_via_simple_index_8`: Calculates a memory address using a base register and an index multiplied by eight, then applies a bitwise OR to the byte at that address.
- `modify_byte_via_complex_index_17`: Updates a byte at a calculated address by performing a bitwise OR with a specific constant using values from several registers.
- `modify_byte_via_complex_index_18`: Uses a base register and a value derived from bitwise operations on other registers to locate and modify a byte via a bitwise OR.
- `modify_byte_via_simple_index_9`: Applies a bitwise OR to a byte at an address calculated from a base register and an indexed value.
- `modify_byte_via_complex_index_19`: Determines a target address through a calculation involving a base register and concatenated register values, then modifies the byte there.
- `modify_byte_via_simple_index_10`: Performs a bitwise OR operation on a byte located at an address offset from a base register by an indexed value.
- `modify_byte_via_simple_index_11`: Modifies a byte at a memory location calculated by adding an offset to a base register, where the offset is an indexed value.
- `modify_byte_via_simple_index_12`: Calculates a pointer using a base register and an index, then performs a bitwise OR on the byte at that pointer.
- `modify_byte_via_index_9`: Performs a bitwise OR operation on a byte at a memory location calculated from a base address and an index.
- `modify_byte_via_index_10`: Updates a byte by applying a bitwise OR mask to a value at a calculated offset.
- `modify_byte_via_index_11`: Applies a bitwise OR to a byte located at a pointer derived from an index and a base address.
- `modify_byte_via_index_12`: Stores a bitwise ORed result into a byte at a memory address determined by an offset calculation.
- `modify_byte_via_index_13`: Modifies a single byte using a bitwise OR operation at a calculated memory address.
- `modify_byte_via_complex_index_20`: Performs a bitwise OR on a byte at a location determined by a complex calculation involving multiple registers.
- `modify_byte_via_index_14`: Uses a bitwise OR to change a byte at a memory address calculated from a base and an index.
- `modify_byte_via_complex_index_21`: Updates a byte using a bitwise OR at an address derived from a combination of register values.
- `modify_byte_via_index_15`: Calculates a memory address using an index and performs a bitwise OR on the byte found there.
- `modify_byte_via_index_16`: Applies a bitwise OR mask to a byte at a memory location calculated from an index and a base pointer.
- `modify_byte_via_complex_index_22`: Calculates an address using a combination of a base register and a value derived from two other registers, then performs a bitwise OR on the byte at that location.
- `modify_byte_via_complex_index_23`: Performs a bitwise OR on a byte at a memory location calculated from a base register and a value derived from two other registers.
- `modify_byte_via_complex_index_24`: Uses a base register and a value derived from two other registers to calculate an address and applies a bitwise OR to the byte stored there.
- `modify_byte_via_complex_index_25`: Calculates a pointer using a base register and a value derived from two other registers, then updates the byte at that address with a bitwise OR.
- `modify_byte_via_simple_index_13`: Calculates an address using a base register and an offset multiplied by eight, then performs a bitwise OR on the byte at that location.
- `modify_byte_via_simple_index_14`: Performs a bitwise OR on a byte at a memory location calculated from a base register and an offset multiplied by eight.
- `modify_byte_via_simple_index_15`: Uses a base register and an offset multiplied by eight to calculate an address and applies a bitwise OR to the byte stored there.
- `modify_byte_via_simple_index_16`: Calculates a pointer using a base register and an offset multiplied by eight, then updates the byte at that address with a bitwise OR.
- `modify_byte_via_complex_index_26`: Calculates an address using a base register and a value derived from two other registers, then performs a bitwise OR on the byte at that location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop`: This routine performs an infinite loop that prevents further execution.
- `empty_routine_2`: This routine contains no valid instructions.
- `contain_invalid_instruction`: Consists of a single byte of data that does not form a valid instruction.
- `execute_infinite_loop_2`: Enters an infinite loop that prevents further execution.
