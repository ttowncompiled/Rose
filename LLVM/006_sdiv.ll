;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;
;   file: sdiv.c (doesn't exist)
;
;   int main() {
;       int a = 4;
;       int b = 8;
;       printf("%d\n", b/a);
;       return 0;
;   }
;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

; ModuleID: sdiv.c

@.str.lit.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00"

define i32 @main() {
    %a = alloca i32, align 4
    store i32 4, i32* %a, align 4
    %b = alloca i32, align 4
    store i32 8, i32* %b, align 4
    %.tmp.1 = getelementptr [4 x i8], [4 x i8]* @.str.lit.1, i32 0, i32 0
    %.tmp.2 = load i32, i32* %b, align 4
    %.tmp.3 = load i32, i32* %a, align 4
    %.tmp.4 = sdiv i32 %.tmp.2, %.tmp.3
    call i32 (i8*, ...) @printf(i8* %.tmp.1, i32 %.tmp.4)
    ret i32 0
}
; the sdiv instruction: sdiv <type> <id1>, <id2>
;   <type> is the type of <id1> and <id2>
;       their types must match
;   returns the quotient of <id1> / <id2>
;   used to calculate the quotient of signed integers
;   division by 0 is undefined behavior
;   sdiv returns a signed int rounded towards 0

declare i32 @printf(i8*, ...)

; EOF