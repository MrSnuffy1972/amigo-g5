# Game 2211 (it came from the desert (1989)(mirrorsoft)[cr power connection][h esi]): code recovery summary

## Functions (part 2 of 2)

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)
(continued)

- `set_bit_in_calculated_address_8`: calculates a pointer using a base address and an offset, then performs a bitwise OR to set specific bits in the byte at that location.
- `update_byte_via_complex_index_3`: Performs a bitwise OR on a byte at a memory location calculated using a complex index involving bit-shifted and XORed registers.
- `modify_target_byte_2`: calculates a memory address using a base pointer and a scaled offset, then performs a bitwise OR operation on the byte at that location with a constant value.
- `modify_bit_in_calculated_address`: calculates a memory address using a base pointer and a value derived from a register, then performs a bitwise OR operation to set a specific bit in the byte at that location.
- `modify_byte_via_calculated_pointer_6`: calculates a memory address using a base pointer and a value derived from certain registers, then performs a bitwise OR operation on the byte at that location.
- `update_byte_via_calculated_pointer_6`: calculates a memory address using a base pointer and a value derived from a register, then performs a bitwise OR operation to set specific bits in the target byte.
- `modify_byte_via_calculated_pointer_7`: calculates a memory address using a base pointer and a value derived from certain registers, then performs a bitwise OR operation on the byte at that location with a constant.
- `modify_byte_via_offset_2`: calculates a pointer using a base address and a scaled index, then performs a bitwise OR on the byte at that location with a constant value.
- `update_bit_in_memory_6`: calculates a memory address using a pointer and a scaled offset, then performs a bitwise OR to set a specific bit in the byte at that location.
- `set_bit_using_complex_index`: Calculates a pointer using a bitwise-manipulated value and an offset, then sets a bit in the byte at that location.
- `set_bit_using_complex_index_2`: Calculates a pointer using a bitwise-manipulated value and an offset, then sets a bit in the byte at that unable location.
- `set_bit_using_simple_index`: Calculates a pointer using a simple index and an offset, then sets a bit in the byte at that location.
- `update_bit_in_calculated_address_5`: calculates a memory address using a base pointer and a scaled offset, then performs a bitwise OR to set a specific bit in the byte at that location.
- `modify_byte_with_mask`: calculates an address based on a register and an offset, then applies a bitwise OR mask to the byte at that location.
- `update_bit_in_calculated_address_6`: calculates a memory address using a pointer and a value derived from a register, then performs a bitwise OR to set a specific bit in the byte at that location.
- `set_bit_mask_to_memory_4`: sets certain bits in a byte at a memory location calculated from a base address and an index-scaled offset.
- `set_bit_in_memory_array_2`: calculates an offset into a memory array using a register and a multiplier, then applies a bitwise OR to a byte at that location.
- `set_bit_flags`: calculates a pointer using a base address and an index, then performs a bitwise OR to set specific bits in a byte at that location.
- `update_bit_in_calculated_address_7`: calculates a memory address based on a register and a bit-shifted value, then performs a bitwise OR to set a specific bit in that memory location.
- `set_bit_in_memory_array_3`: calculates an offset into a byte array using a register and an index, then performs a bitwise OR to set specific bits in that byte.
- `modify_byte_via_indexed_pointer_2`: calculates an address using a base pointer and an index, then performs a bitwise OR on the byte at that location with a constant value.
- `modify_byte_via_complex_index`: Performs a bitwise OR on a byte at a memory location calculated using a complex combination of registers.
- `update_byte_via_calculated_pointer_7`: calculates a memory address using a base pointer and a scaled offset, then performs a bitwise OR operation to set specific bits in the target byte.
- `modify_target_byte_3`: calculates a memory address using a base pointer and a scaled offset, then performs a bitwise OR operation to set specific bits in the byte at that location.
- `set_bit_in_calculated_address_9`: calculates an address based on a register and an offset, then performs a bitwise OR to set specific bits in the byte at that location.
- `update_byte_via_calculated_pointer_8`: calculates a memory address using a register and a transformed value, then performs a bitwise OR operation to set specific bits in the byte at that location.
- `update_bit_in_calculated_address_8`: calculates a memory address using a base pointer and a value derived from certain registers, then performs a bitwise OR to set a specific bit in the byte at that location.
- `update_bit_in_memory_7`: calculates a memory address using a pointer and an offset, then performs a bitwise OR operation to set a specific bit in the byte at that location.
- `modify_byte_via_pointer`: calculates an address using a base pointer and an offset, then performs a bitwise OR on the byte at that location with a constant value.
- `set_byte_bit_mask`: Calculates an address using a base pointer and an index, then applies a bitwise OR with a constant to the byte at that location.
- `modify_byte_at_index`: Performs a bitwise OR operation on a byte located at a calculated offset from a pointer.
- `apply_bit_to_byte`: Updates a byte at a memory location derived from a register-based index and a base address.
- `set_bit_using_complex_index_3`: Calculates a target address using a combination of a base pointer and a value derived from bit-shifted and XORed registers, then modifies a byte.
- `update_byte_with_index_logic`: Uses a complex index involving bit-shifting and XORing to locate a byte and applies a bitwise OR.
- `apply_mask_to_calculated_address`: Calculates a pointer using a register-based offset and sets a bit in the byte at that address.
- `modify_byte_via_reconstructed_index`: Determines a memory address using a reconstructed index from multiple registers and applies a bitwise OR to a byte.
- `set_bit_at_reconstructed_offset`: Calculates a destination address using a complex register-based index and performs a bitwise OR on the target byte.
- `set_byte_with_offset_index`: Calculates a pointer using an index-based offset and applies a bitwise OR to the byte at that address.
- `apply_bit_mask_to_pointer_offset`: Uses a complex index-based calculation involving bit-shifting and XORing to target a byte for a bitwise OR operation.
- `set_bit_in_memory_2`: calculates a memory address based on a register and an offset, then performs a bitwise OR to set specific bits in a byte at that location.
- `modify_byte_via_offset_3`: Calculates an address using a base pointer and an offset, then performs a bitwise OR on the byte at thatlocation.
- `set_bit_in_memory_array_4`: calculates an offset into a byte array using a register and a multiplier, then performs a bitwise OR to set specific bits in that byte.
- `set_bit_mask_to_memory_5`: calculates a memory address based on a register and an offset, then performs a bitwise OR operation to set specific bits in that byte.
- `set_bit_in_calculated_address_10`: calculates a memory address using a base pointer and an index, then performs a bitwise OR to set specific bits in the byte at that location.
- `modify_target_byte_4`: calculates an address based on a register and an offset, then performs a bitwise OR operation on the byte at that location with a constant value.
- `set_bit_in_calculated_byte`: calculates a pointer using a base address, a register, and a scaled offset to perform a bitwise OR operation on a single byte.
- `modify_target_byte_5`: calculates an address based on a register and an offset, then performs a bitwise OR operation on the byte at that location using a constant mask.
- `modify_byte_complex_offset`: Calculates a target address using a complex combination of register values and an offset, then performs a bitwise OR on the byte at that location.
- `set_bit_in_memory_3`: sets or modifies specific bits within a byte at a calculated memory location.
- `modify_calculated_address_v1`: Calculates an address using a pointer and a value derived from a register, then applies a bitwise OR to the byte at that location.
- `modify_scaled_address_v1`: Calculates an address using a pointer and a scaled register value, then applies a bitwise OR to the byte at that location.
- `modify_byte_with_mask_2`: calculates a pointer using a base address and a scaled index, then performs a bitwise OR on the byte at that location with a constant mask.
- `update_bit_in_calculated_address_9`: calculates a memory address using a base pointer and a value derived from certain registers, then performs a bitwise OR operation to set a specific bit at that location.
- `update_bit_in_memory_8`: calculates a memory address based on a register-held offset and a value derived from other registers, then performs a bitwise OR to set a specific bit in that memory location.
- `modify_calculated_address_v4`: Calculates an address using a pointer and a value derived from a register, then applies a bitwise OR to the byte at a calculated location.
- `update_bit_in_calculated_address_10`: calculates a memory address using a base pointer and a value derived from a register, then performs a bitwise OR to set a specific bit in the byte at that location.
- `set_bit_mask_to_memory_6`: calculates a memory address using a base pointer, a register, and an offset, then applies a bitwise OR mask to the byte at that location.
- `modify_scaled_address_v4`: Calculates anaddress using a pointer and a scaled register value, then applies a bitwise OR to the byte at that location.
- `set_bits_in_table_entry`: sets certain bits in a byte at a calculated index within a table or array.
- `set_bit_in_table`: sets a bit in a byte at a memory location calculated from a pointer and a register.
- `set_bit_with_complex_index`: sets a bit in a byte at a memory location calculated using a complex index and a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop`: the routine executes an infinite loop that never exits.
- `empty_routine_3`: this routine contains no executable instructions.
- `empty_routine_4`: the routine contains no executable instructions and performs no operations.
- `loop_indefinitely`: enters an infinite loop without performing any other operations.
