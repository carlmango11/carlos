	.file	"main.dc1f31da4ed3d656-cgu.0"
	.section	.text._start,"ax",@progbits
	.globl	_start
	.type	_start,@function
_start:
	movl	$753665, %eax
	xorl	%ecx, %ecx
.LBB0_1:
	cmpq	$5, %rcx
	je	.LBB0_3
	movw	$3907, -1(%rax,%rcx,2)
	incq	%rcx
	jmp	.LBB0_1
.LBB0_3:
	jmp	.LBB0_3
.Lfunc_end0:
	.size	_start, .Lfunc_end0-_start

	.section	.text._RNvCsj4CZ6flxxfE_7___rustc17rust_begin_unwind,"ax",@progbits
	.hidden	_RNvCsj4CZ6flxxfE_7___rustc17rust_begin_unwind
	.globl	_RNvCsj4CZ6flxxfE_7___rustc17rust_begin_unwind
	.type	_RNvCsj4CZ6flxxfE_7___rustc17rust_begin_unwind,@function
_RNvCsj4CZ6flxxfE_7___rustc17rust_begin_unwind:
.LBB1_1:
	jmp	.LBB1_1
.Lfunc_end1:
	.size	_RNvCsj4CZ6flxxfE_7___rustc17rust_begin_unwind, .Lfunc_end1-_RNvCsj4CZ6flxxfE_7___rustc17rust_begin_unwind

	.ident	"rustc version 1.90.0 (1159e78c4 2025-09-14)"
	.section	".note.GNU-stack","",@progbits
