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

@.str.lit.1 = private unnamed_addr constant [15 x i8] c"Hello, World!\0A\00"

declare i32 @printf(i8*, ...)
; @printf is an acessible Global function that is provided by LLVM
;   it is a var args function which takes:
;       a single unnamed parameter which is of type: i8* --- an array of chars
;       followed by any number (even 0) of args of any type
;   @printf returns an i32
; NOTE THE LINE FEED \0A CHARACTER
;   unlike @puts, @printf doesn't include a newline character by default

define i32 @main() {
    %.tmp.1 = getelementptr [15 x i8], [15 x i8]* @.str.lit.1, i32 0, i32 0
    call i32 (i8*, ...) @printf(i8* %.tmp.1)
    ret i32 0
}
; calling a var args function is a bit tricky
;   the type of the call to a var args function is different
;   the type is the return type of the function
;       i32 since @printf returns an i32
;   followed by the full parameter declaration of the function
;       (i8*, ...) since this is the full parameter declaration of @printf

; EOF