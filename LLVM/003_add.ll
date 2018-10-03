;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;
;   file: add.c (doesn't exist)
;
;   int main() {
;       int a = 4;
;       int b = 8;
;       return a + b;
;   }
;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

; ModuleID: add.c

define i32 @main() {
    %a = alloca i32, align 4
    store i32 4, i32* %a, align 4
    %b = alloca i32, align 4
    store i32 8, i32* %b, align 4
    %.tmp.1 = load i32, i32* %a, align 4
    %.tmp.2 = load i32, i32* %b, align 4
    %.ret.val.1 = add i32 %.tmp.1, %.tmp.2
    ret i32 %.ret.val.1
}
; alloca is used to allocate Stack memory, returns a pointer to Stack memory
;   all defined within a function should use Stack memory
; the alloca instruction: alloca <type>, align <1, 2, 4, or 8>
;   <type> is the type of the allocation: in this case it is a 32-bit integer
;   align sets the allocation to be 4*8-bits = 32-bits
;       this deals with register alignment
;       should be set to 4 and should be consistent
;       provides best performance at this point in time
;   alignment deals with registers architecture
;   doesn't have to be the same as the computer architecture
;   different alignments produce different performances, even on x64 machines
; alloca returns a <type>* pointer to Stack memory
; we store this pointer in the identifier %a
; Local variables are always named with %
;   we use %a to represent the a variable from the example C code above
; store is used to store a value in the Stack
; the store instruction: store <type> <identifier>, <type>* <target>, align <1, 2, 4, or 8>
;   <type> is the type of identifier and the type of the target
;       the type of both should match
;   <type>* is a pointer to the literal type
;   in this case we are storing an 32-bit integer at an address on the Stack
;   <identifier> provides the value that we wish to store
;       can also be a literal such as 4
;   <target> is the identifier of the pointer to Stack memory where we are storing
;   align sets the alignment of the store
;       in this case 4 since the allocation used an alignment of 4
; once a value has been stored on the Stack, it can only be retrieved using a load
; the load instruction: load <type>, <type>* <target>, align <1, 2, 4, or 8>
;   <type> is the literal type of the value that we are loading
;   <type>* is the pointer to the literal type
;   <target> is the pointer to Stack memory that we are loading from
;   align sets the alignment of the load
;       we use 4 since we used 4 for the store instruction
; load returns a literal of <type>: in this case i32
; we store the first load at %.tmp.1 --- it's the 1st tmp value we've defined
; we store the second load at %.tmp.2 --- it's the 2nd tmp value we've defined
; do one instruction at a time --- don't try to combine instructions
;   combining instructions in one line doesn't always work intuitively or at all
; we combine the two values with an add
; the add instruction: add <type> <id1>, <id2>
;   arithmetically adds the literal values of <id1> and <id2> together
;   <type> is the type of <id1> and <id2>
;       they must be the same type
;       they can be literals
; add returns the sum of the two values
; the sum is assigned to @.ret.value.1 --- the first return value we've defined

; EOF