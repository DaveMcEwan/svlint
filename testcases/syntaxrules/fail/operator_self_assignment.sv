module M;
  always @* z += y; // Addition `z = z + y`
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @* z -= y; // Subtraction `z = z - y`
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @* z *= y; // Multiplication `z = z * y`
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @* z /= y; // Division `z = z / y`
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @* z %= y; // Modulo `z = z % y`
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @* z &= y; // Bitwise AND `z = z & y`
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @* z |= y; // Bitwise OR `z = z | y`
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @* z ^= y; // Bitwise XOR `z = z ^ y`
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @* z <<= y; // Logical left shift `z = z << y`
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @* z >>= y; // Logical right shift `z = z >> y`
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @* z <<<= y; // Arithmetic left shift `z = z <<< y`
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @* z >>>= y; // Arithmetic right shift `z = z >>> y`
endmodule