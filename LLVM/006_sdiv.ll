;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;
;   file: sdiv.c (doesn't exist)
;
;   int main() {
;       int a = 4;
;       int b = 8;
;       return b / a;
;   }
;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

; ModuleID: sdiv.c

define i32 @main() {
    %a = alloca i32, align 4
    store i32 4, i32* %a, align 4
    %b = alloca i32, align 4
    store i32 8, i32* %b, align 4
    %.tmp.1 = load i32, i32* %b, align 4
    %.tmp.2 = load i32, i32* %a, align 4
    %.ret.val.1 = sdiv i32 %.tmp.1, %.tmp.2
    ret i32 %.ret.val.1
}
; the sdiv instruction: sdiv <type> <id1>, <id2>
;   <type> is the type of <id1> and <id2>
;       their types must match
;   sdiv returns <id1> / <id2>
;   division by 0 is undefined behavior
;   sdiv returns a signed int rounded towards 0

; EOF