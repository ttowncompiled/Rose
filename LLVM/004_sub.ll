;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;
;   file: sub.c (doesn't exist)
;
;   int main() {
;       int a = 4;
;       int b = 8;
;       printf("%d\n", b-a);
;       return 0;
;   }
;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

; ModuleID: sub.c

@.str.lit.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00"

define i32 @main() {
    %a = alloca i32, align 4
    store i32 4, i32* %a, align 4
    %b = alloca i32, align 4
    store i32 8, i32* %b, align 4
    %.tmp.1 = load i32, i32* %b, align 4
    %.tmp.2 = load i32, i32* %a, align 4
    %.tmp.3 = getelementptr [4 x i8], [4 x i8]* @.str.lit.1, i32 0, i32 0
    %.tmp.4 = sub i32 %.tmp.1, %.tmp.2
    call i32 (i8*, ...) @printf(i8* %.tmp.3, i32 %.tmp.4)
    ret i32 0
}
; I prefer that the order of instructions models the order of the AST
;   makes example closer to final emitted instructions
;   variables are defined first
;   each definition requires an allocation and a store
;   then the return statement returns the difference
;   we load a tmp variable for each variable in the return expression
;   tmps are loaded in order of operator precedence
;       in this case: b then a
;   return value is computed and stored in a tmp
;   then the tmp expression for the return value is returned
; the sub instruction: sub <type> <id1>, <id2>
;   <type> is the type of <id1> and <id2>
;       the types must match
;   returns the difference of <id1> - <id2>

declare i32 @printf(i8*, ...)

; EOF