; Day 1's solution in PP2 assembly.
;

@DATA
	; Our initial puzzle input.
	puzzle_input	DW	199,200,208,210,200,207,240,269,260,263
	; We don't seem to have access to the static size of the puzzle_input array,
	; (as far as I can tell from the old leftover docs). This is kind of a hack,
	; but allows us to figure out where the array ends.
	stopper_value	DW	0
	; The window size is reconfigurable at run tine
	window_size	DW	1

; The default stack space is fine, we use like 1 value + a few function calls.

@CODE
	main:
	; Initially the window is set to size 1
	BRS solve_for_current_window
	; Push the result onto the stack
	PUSH R0

	; Configure a new window
	LOAD R0, 3
	STOR R0, [R6+window_size]
	; Recalculate for the new window size.
	BRS solve_for_current_window
	LOAD R1, R0
	PULL R0

	; Reading values from a pp2 is already confusing
	; enough as it is, lets remove the clobbers
	XOR R2, R2
	XOR R3, R3
	XOR R4, R4
	XOR R5, R5

	; All done
	BRA sinkhole

	solve_for_current_window:
	; R0: Ret val
	; R1: last value
	; R2: scratch
	; R3: Counter value
	; R4: puzzle idx
	; R5: scratch
	LOAD R3, 0
	LOAD R4, puzzle_input

	; Load an initial value into R1
	BRS do_window_add
	LOAD R1, R0
	ADD R4, 1

	; Iterate over all values
	loop:
	; If the last value is less or equal to the next,
	; skip the increment step
	BRS do_window_add
	CMP R1, R0
	BGE skip_inc
	ADD R3, 1
	skip_inc:
	LOAD R1, R0
	ADD R4, 1
	; Check if we are still in puzzle_input land
	BRS check_bounds
	BLT loop
	; Return
	LOAD R0, R3
	RTS

	; Does the window add
	; R4 must point to start of window
	; result is returned in R0
	; clobbers: R2, R5
	do_window_add:
	LOAD R0, 0
	LOAD R2, 0
	LOAD R5, R4
	window_loop:
	ADD R0, [R6+R5]
	ADD R2, 1
	ADD R5, 1
	CMP R2 [R6+window_size]
	BLT window_loop
	RTS

	; Set the negative flag if we're about to go OOB.
	; R4 must contain current idx
	; clobbers: R2
	check_bounds:
	LOAD R2, R4
	ADD R2, [R6+window_size]
	; We also want set negative flag if we are _at_ window size,
	; so sub 1 more before the compare
	SUB R2, 1
	CMP R2, stopper_value
	RTS

	sinkhole:
	BRA sinkhole
@END
