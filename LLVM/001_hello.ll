;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;
;   file: hello.c (doesn't exist)
;
;   int main() {
;       printf("Hello, World!\n");
;       return 0;
;   }
;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

; ModuleID: hello.c

@.str.lit.1 = private unnamed_addr constant [14 x i8] c"Hello, World!\00"
; literals have to be declared as a Global constant unless they're only used in a single op
; this is a Heap allocation
;   more efficient than a Stack allocation, Heap allocation only needs to be done once
;   Stack allocation would be called each time the function was called
; Global variables must start with @
; letters, numbers, and periods are allowed in identifiers
; all Global variables are treated as if they are pointers
;   they are pointers to Heap memory
; Global assignment: @ident = <list of attributes, space separated> <type> <value>
; @.str.lit.1 is the identifier --- stands for the 1st string literal to be defined
; the attributes are: private unnamed_addr constant
; private prevents @.str.lit.1 from being leaked outside of the module
;   all such literals should be private, would be bad to have literals leaked
;   that would break encapsulation and could lead to naming collisions with emitted code
; unnamed_addr indicates that @.str.lit.1 will be tacitly assigned a memory address
;   this will require that @.str.lit.1 be referenced only by its identifier
; constant indicates that the value of @.str.lit.1 cannot be mutated
; the type of @.str.lit.1 is [14 x i8]
; i8 is an 8-bit integer which is also an ASCII character or char
; [14 x i8] is an array of 14 chars (in this case)
; c"Hello, World!\00" is the value of @.str.lit.1
; all strings literals must begin with c --- it's just the convention
; \00 is the null-character which terminates the string
; the length of c"Hello, World!\00" is 14 chars, hence [14 x i8] --- 14 chars
;   the length of 14 includes the \00 null-terminating character

declare i32 @puts(i8* nocapture) nounwind
; @puts is an accessible Global function that is provided by LLVM
; this is its declaration
; declare indicates that @puts is defined outside of this module
; i32 indicates that the return type of @puts is a 32-bit integer
; @puts is the name which is used to call this function
; (i8* nocapture) is the parameter list
;   this parameter list includes a single unnamed parameter
;   parameter doesn't need to be named since we're only declaring the function
;   parameters are matched from left to right
; i8* indicates that the first parameter is a pointer to an array of chars
; nocapture indicates the first parameter will not be copied inside of @puts
;   nocapture is an attribute of the first parameter
;   essentially means that the pointer is safe from duplication
;   doesn't indicate that the pointer is safe from mutation
; nounwind is an attribute of @puts
;   indicates that @puts will not raise an exception
;   exceptions can still be raised, they will just be handled inside of @puts

define i32 @main() {
    call i32 @puts(i8* getelementptr ([14 x i8], [14 x i8]* @.str.lit.1, i64 0, i64 0))
    ret i32 0
}
; @main is a function which is defined within this module
; functions which are defined the module using the define keyword
; i32 indicates the return type of main is a 32-bit integer
; @main is the name of the function --- main is called automatically when this program is executed
;   also called the entry-point of the program
; () is an empty parameter list
;   @main doesn't receive any parameters
;   doesn't mean that @main isn't passed parameters
;       normally all functions should declare their parameters
;       @main is an exception because it is the entry-point to the program
;       @main is used to receive command line arguments
; {} is used to encapsulate the body of the function
; call is a keyword which executes a function
; the call instruction: call <return-type> <function>(<parameter list>)
; the return type of @puts is i32 so we use: call i32 @puts(...)
; @puts takes a single parameter which is a pointer to an array of chars
; the array of chars that we want is the string literal that we defined globally
; Global variables are treated as pointers
; to get a pointer declared outside of the body of the function, we must use getelementptr
; getelementptr essentially gets a pointer to Heap memory
; the getelementptr instruction: getelementptr <type1>, <type1>* <identifier>, <type2> pointer-offset, <type2> field-offset
;   getelementptr or (GEP) is a bit strange
;   <type1> is the literal type (non-pointer type): in this case [14 x i8] for an array of 14 chars
;   <type1>* is a pointer to the literal type: [14 x i8]* for a pointer to an array of chars
;   <identifier> is the identifier that we are getting a pointer to: @.str.lit.1
;   <type2> should be i32 for x86 or i64 for x64: this is the type of the pointer offset
;   pointer-offset is the offset from the head of the array: 0 is the beginning
;   field-offset is the offset of the struct at pointer-offset
;       LLVM treats all elements of all arrays as structs
;       a single value is essentially just a struct with one thing in it: 0 being the head
;       use 0 for non-structs
; the call to @puts requires the argument to be typed
; @puts requires a single argument of type i8* so the getelementptr must be cast to i8*
;   not technically a cast since its [14 x i8]* to i8*
;   [14 x i8]* and i8* are just different representations of the same thing
; the last statement is the return op for main
; the ret instruction: ret <type> <identifier>
; ret is used to return a value from the function
; <type> is the type of the return variable
;   the type must be consistent with the type of the <identifier> and the type of the function
; <identifier> is the identifier for the value that is being returned
;   can also be a literal as is used above
;   by C convention, 0 means that the program executed successfuly
;       this convention is used to allow any number of error codes
;       we return 0 from main to follow this convention

; EOF