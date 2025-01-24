// Program: Multiplication.asm
// Multiplies R0 and R1 and stores the result in R2

	// R2 = 0
	@R2
	M=0
	// i = 0
	@i
	M=0
(LOOP)
	// if (i > R0) goto END
	@R0
	D=M
	@i
	D=D-M
	@END
	D;JEQ
	// R2 = R2 + R1
	@R2
	D=M
	@R1
	D=D+M
	@R2
	M=D
	// i = i + 1
	@i
	M=M+1
	@LOOP
	0;JMP
(END)
	@END
	0;JMP