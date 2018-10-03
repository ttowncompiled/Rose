;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;
;   file: fmul.c (doesn't exist)
;
;   int main() {
;       double a = 4.8;
;       double b = 8.2;
;       printf("%f\n", a*b);
;       return 0;
;   }
;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

; ModuleID: fmul.c

@.str.lit.1 = private unnamed_addr constant [4 x i8] c"%f\0A\00"

define i32 @main() {
    %a = alloca double, align 8
    store double 4.800000e+00, double* %a, align 8
    %b = alloca double, align 8
    store double 8.200000e+00, double* %b, align 8
    %.tmp.1 = getelementptr [4 x i8], [4 x i8]* @.str.lit.1, i32 0, i32 0
    %.tmp.2 = load double, double* %a, align 8
    %.tmp.3 = load double, double* %b, align 8
    %.tmp.4 = fmul double %.tmp.2, %.tmp.3
    call i32 (i8*, ...) @printf(i8* %.tmp.1, double %.tmp.4)
    ret i32 0
}
; the fmul instruction: fmul <type> <id1>, <id2>
;   <type> is the type of <id1> and <id2>
;       their types must match
;   returns the product of <id1> * <id2>
;   used to calculate the product of floats and doubles

declare i32 @printf(i8*, ...)

; EOF