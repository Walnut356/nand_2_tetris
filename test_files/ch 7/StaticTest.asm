//init 'stack' pointer
@256
D=A
@SP
M=D
//push constant 111
@111
D=A
@SP
A=M
M=D
@SP
AM=M+1
//push constant 333
@333
D=A
@SP
A=M
M=D
@SP
AM=M+1
//push constant 888
@888
D=A
@SP
A=M
M=D
@SP
AM=M+1
//pop static 8
@8
D=A
@static
A=D+M
D=A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
//pop static 3
@3
D=A
@static
A=D+M
D=A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
//pop static 1
@1
D=A
@static
A=D+M
D=A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
//push static 3
@3
D=A
@static
A=D+M
D=M
@SP
A=M
M=D
@SP
AM=M+1
//push static 1
@1
D=A
@static
A=D+M
D=M
@SP
A=M
M=D
@SP
AM=M+1
//sub
@SP
AM=M-1
D=M
@SP
AM=M-1
M=M-D
@SP
AM=M+1
//push static 8
@8
D=A
@static
A=D+M
D=M
@SP
A=M
M=D
@SP
AM=M+1
//add
@SP
AM=M-1
D=M
@SP
AM=M-1
M=D+M
@SP
AM=M+1
(INFINITE_LOOP)
@INFINITE_LOOP
0;JMP