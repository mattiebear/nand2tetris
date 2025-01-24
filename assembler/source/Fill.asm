// Program: Fill.asm
// Displays a black screen if any key is pressed
// Black is set to 1, white to 0

(LOOP)
    // n = 0
    @n
    M=0
    @KBD
    D=M
    // if (keydown) goto DOWN
    @DOWN
    D;JGT
    // else goto UP
(UP)
    // if n == 8192, goto LOOP
    @8192
    D=A
    @n
    D=D-M
    @LOOP
    D;JEQ

    // Screen register + n
    @n
    D=M
    @SCREEN
    A=D+A
    // Set value of register to off
    M=0
    // n = n + 1
    @n
    M=M+1
    @UP
    0;JMP
(DOWN)
    // if n == 8192, goto LOOP
    @8192
    D=A
    @n
    D=D-M
    @LOOP
    D;JEQ

    // Screen register + n
    @n
    D=M
    @SCREEN
    A=D+A
    // Set value of register to off
    M=-1
    // n = n + 1
    @n
    M=M+1
    @DOWN
    0;JMP