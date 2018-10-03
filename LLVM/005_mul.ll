;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;
;   file: mul.c (doesn't exist)
;
;   int main() {
;       int a = 4;
;       int b = 8;
;       return a * b;
;   }
;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

; ModuleID: mul.c

define i32 @main() {
    %a = alloca i32, align 4
    store i32 4, i32* %a, align 4
    %b = alloca i32, align 4
    store i32 8, i32* %b, align 4
    %.tmp.1 = load i32, i32* %a, align 4
    %.tmp.2 = load i32, i32* %b, align 4
    %.ret.val.1 = mul i32 %.tmp.1, %.tmp.2
    ret i32 %.ret.val.1
}
; the mul instruction: mul <type> <id1>, <id2>
;   <type> is the type of <id1> and <id2>
;       the types must match
;   returns the value of <id1> * <id2>

; EOF