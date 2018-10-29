# Rose lang overview

# Keywords
*Keywords are noted because they have significant meaning in the language. However,
keywords can be reused for other purposes or used across multiple types. Some of these
keywords relate to traits that will be implemented in the std library. Reserved words
which can not be overloaded will be marked RW instead of KW.*

| Lexer         | Keyword       | Description                                       |
|---------------|---------------|---------------------------------------------------|
| KW\_ABSTRACT  | abstract      | Declares an abstract type or routine definition   |
| RW\_AND       | and           | Logical and, infix operator                       |
| KW\_AS        | as            | Casts left type to right type, aliases imports    |
| KW\_ASSERT    | assert        | A reserved macro for declaring assertions         |
| KW\_AWAIT     | await         | Queues function call to the event queue           |
| RW\_BEGIN     | begin         | Declares a basic block                            |
| KW\_BLANK     | _             | A non-binding variable, catch-all match           |
| KW\_BORROW    | borrow        | Allocates a stack-to-stack pointer                |
| KW\_BREAK     | break         | Terminates execution of a block                   |
| KW\_CALL      | call          | Calls a routine                                   |
| KW\_CLASS     | class         | A stateful interface type                         |
| KW\_CLONE     | clone         | Creates a deep copy                               |
| KW\_CLOSE     | close         | Frees an OS resource                              |
| KW\_CONST     | const         | For const bindings (immutable, no-shadowing)      |
| KW\_CONTINUE  | continue      | Jumps within the block                            |
| KW\_COPY      | copy          | Creates a shallow copy                            |
| KW\_DEF       | def           | Defines a routine for the dispatcher              |
| KW\_DEFER     | defer         | Defers execution to end of block                  |
| KW\_DERIVE    | derive        | A reserved keyword for procedural macros          |
| RW\_DO        | do            | Declares an isolated block                        |
| KW\_DROP      | drop          | Deallocates heap memory                           |
| RW\_ELSE      | else          | Alternative branch                                |
| RW\_END       | end           | Marks the end of a block                          |
| KW\_ENUM      | enum          | Enumerated type                                   |
| KW\_EXIT      | exit          | A reserved macro for exiting the program          |
| KW\_EXPECT    | expect        | A reserved macro for declaring expectations       |
| KW\_EXT       | ext           | Declares ext relationship                         |
| RW\_FALSE     | false         | The false Boolean literal                         |
| KW\_FINAL     | final         | Declares a final binding                          |
| RW\_FN        | fn            | Anonymous closure definition                      |
| KW\_FOR       | for           | Loop for iterable collections                     |
| KW\_GET       | get           | Defines a getter routine                          |
| KW\_GETS      | gets          | A reserved macro for getting standard input       |
| KW\_HAS       | has           | Declares has relationship                         |
| RW\_IF        | if            | Primary branch                                    |
| KW\_IM        | im            | The imaginary type postfix unary operator         |
| KW\_IMPL      | impl          | Declares an impl relationship                     |
| KW\_IN        | in            | Binds left var to right collection, membership    |
| RW\_INF       | Inf           | Infinity                                          |
| KW\_IS        | is            | Strict equality check, infix operator             |
| RW\_LET       | let           | Declares a variable binding                       |
| KW\_LOOP      | loop          | Basic loop block, classic for-loop                |
| KW\_MACRO     | macro         | Declares a custom macro                           |
| KW\_MATCH     | match         | Pattern matching, switch expression               |
| KW\_MOD       | mod           | Declares module, imports local module             |
| KW\_MOVE      | move          | Moves binding to new owner                        |
| RW\_MUT       | mut           | Mutable binding                                   |
| RW\_NAN       | NaN           | Not a number, Inf/Inf                             |
| KW\_NEW       | new           | Allocates heap memory and a stack-to-heap pointer |
| RW\_NIL       | nil           | The nil or null type                              |
| RW\_NOT       | not           | Logical not, prefix unary operator                |
| KW\_OPEN      | open          | Claims an OS resource                             |
| RW\_OR        | or            | Logical or, infix operator                        |
| KW\_OVERLOAD  | overload      | Declares a routine overload for the dispatcher    |
| KW\_OVERRIDE  | override      | Declares a routine override for the dispatcher    |
| KW\_PANIC     | panic         | A reserved macro for throwing an err              |
| KW\_PRINT     | print         | A reserved macro for printing to std out          |
| KW\_PRIV      | priv          | Private type or binding                           |
| KW\_PRO       | pro           | Protected type or binding                         |
| KW\_PUB       | pub           | Publicly available type or binding                |
| KW\_PUTS      | puts          | A reserved macro for printing to std out with '\n'|
| KW\_REF       | ref           | Keyword used for matching borrows and boxes       |
| KW\_RETURN    | return        | Terminates block and returns right value          |
| KW\_SAVE      | save          | A reserved macro for saving the current program   |
| KW\_SELFVALUE | self          | Instance of the current context                   |
| KW\_SELFTYPE  | Self          | Type of the current context                       |
| KW\_SET       | set           | Defines a setter routine                          |
| KW\_STATIC    | static        | Declares a static binding (singleton, eternal)    |
| KW\_STRUCT    | struct        | Declares a struct type                            |
| KW\_SUPERVALUE| super         | Parent of the instance of the current context     |
| KW\_SUPERTYPE | Super         | Type of the parent of the current context         |
| KW\_TRAIT     | trait         | A stateless interface type                        |
| RW\_TRUE      | true          | The true Boolean literal                          |
| KW\_TUPLE     | tuple         | Declares a tuple type                             |
| KW\_TYPE      | type          | Declares a type alias                             |
| KW\_TYPEOF    | typeof        | Casts a var to its symbolic type                  |
| KW\_UNTIL     | until         | Declares an event trigger for the dispatcher      |
| KW\_USE       | use           | Imports module from root                          |
| KW\_USES      | uses          | Declares uses relationship                        |
| KW\_VIRTUAL   | virtual       | Declares a virtual routine definition             |
| KW\_WHERE     | where         | Declare guard(s)                                  |
| KW\_WHILE     | while         | While loop                                        |
| KW\_WITH      | with          | Declares bindings for block                       |
| KW\_YIELD     | yield         | Return call for generators                        |

# Operators
| Lexer             | Operator      | Description                                       |
|-------------------|---------------|---------------------------------------------------|
| OP\_ADD           | +             | Infix arithmetic addition, unary +                |
| OP\_SUB           | -             | Infix arithmetic subtraction, unary -             |
| OP\_MUL           | \*            | Infix arithmetic multiplication, unary deref      |
| OP\_DIV           | /             | Infix operator for arithmetic division            |
| OP\_MOD           | %             | Infix operator for arithmetic modulation          |
| OP\_POW           | \*\*          | Infix operator for arithmetic exponentiation      |
| OP\_RDIV          | //            | Infix operator for arithmetic ration division     |
| OP\_CMP           | <=>           | Infix comparator, returns -1, 0, 1                |
| OP\_NOT           | !             | Prefix not, boolean                               |
| OP\_EQ            | ==            | Infix equality check, boolean                     |
| OP\_NEQ           | !=            | Infix not-equality check, boolean                 |
| OP\_GT            | >             | Infix greater-than, boolean                       |
| OP\_GTE           | >=            | Infix greater-than-equal-to, boolean              |
| OP\_LT            | <             | Infix less-than, boolean                          |
| OP\_LTE           | <=            | Infix less-then-equal-to, boolean                 |
| OP\_LSHIFT        | <<            | Bitwise left-logical-shift                        |
| OP\_RSHIFT        | >>            | Bitwise right-logical-shift                       |
| OP\_AND           | &&            | Bitwise AND                                       |
| OP\_OR            | \|\|          | Bitwise OR                                        |
| OP\_XOR           | ^             | Bitwise XOR                                       |
| OP\_LNOT          | ~             | Bitwise NOT                                       |
| OP\_ASSIGN        | =             | Assignment operator                               |
| OP\_INF\_ASSIGN   | :=            | Assign and infer type from right type             |
| OP\_ADD\_ASSIGN   | +=            | Adds left to right and assigns to left            |
| OP\_SUB\_ASSIGN   | -=            | Subs right from left and assigns to left          |
| OP\_MUL\_ASSIGN   | \*=           | Muls right and left and assigns to left           |
| OP\_DIV\_ASSIGN   | /=            | Divides left by right and assigns to left         |
| OP\_MOD\_ASSIGN   | %=            | Mods left by right and assigns to left            |
| OP\_POW\_ASSIGN   | \*\*=         | Computes pow(left, right) and assigns to left     |
| OP\_RDIV\_ASSIGN  | //=           | Rational divides left by right and assigns to left|
| OP\_LSHIFT\_ASSIGN| <<=           | Left-logically-shifts left and assigns to left    |
| OP\_RSHIFT\_ASSIGN| >>=           | Right-logically-shifts left and assigns to left   |
| OP\_AND\_ASSIGN   | &&=           | Logical and of left and right and assigns to left |
| OP\_OR\_ASSIGN    | \|\|=         | Logical or of left and right and assigns to left  |
| OP\_XOR\_ASSIGN   | ^=            | Logical xor of left and right and assigns to left |
| OP\_CURRY         | .             | Left-right function composition, membership       |
| OP\_COMP          | \|            | Right-left function composition                   |
| OP\_MORPH         | ->            | Declares resulting type of routine                |
| OP\_MAP           | =>            | Maps left to right, infer right return type       |
| OP\_DISPATCH      | ::            | Dispatch resolution                               |
| OP\_BCAST\_ADD    | .+            | Matrix addition                                   |
| OP\_BCAST\_SUB    | .-            | Matrix subtraction                                |
| OP\_BCAST\_MUL    | .\*           | Matrix multiplication                             |
| OP\_BCAST\_DIV    | ./            | Matrix division                                   |
| OP\_BORROW        | &             | Borrow by stack-to-stack reference                |
| OP\_LIFETIME      | \`            | Annotates borrow with lifetime                    |
| OP\_RANGE\_INC    | ..            | Infix operator for inclusive range                |
| OP\_SPLAT         | ...           | Infix operator for exclusive range, prefix splat  |

# Delimiters
| Lexer             | Delimiter     | Description                                       |
|-------------------|---------------|---------------------------------------------------|
| DEL\_END          | '\n', '\r', ; | Marks the end of a statement or expression        |
| DEL\_COMMA        | ,             | Comma separator for params, variables, etc.       |
| DEL\_COLON        | :             | Separates binding val from binding type           |
| DEL\_LPAREN       | (             | Open paren for precedence, params, args, tuples   |
| DEL\_RPAREN       | )             | Closing paren for precedence, params, args, tuples|
| DEL\_LBRACKET     | [             | Open bracket for precedence, basic collections    |
| DEL\_RBRACKET     | ]             | Closing bracket for precedence, basic collections |
| DEL\_LBRACE       | {             | Open brace for contexts, scopes, and structs      |
| DEL\_RBRACE       | }             | Closing brace for contexts, scopes, and structs   |

# Literals
| Lexer             | Literal       | Description                                       |
|-------------------|---------------|---------------------------------------------------|
| LIT\_IDENT        | foo           | A variable, routine, or type identifier           |
| LIT\_BLANK        | _             | A non-binding variable, catch-all for match       |
| LIT\_INT          | 5             | A signed, i32 integer literal                     |
| LIT\_FLOAT        | 5.0           | A signed, f64 float literal                       |
| LIT\_INF          | Inf           | A literal for Infinity                            |
| LIT\_NAN          | NaN           | A literal for not-a-number                        |
| LIT\_NIL          | nil           | A literal for the nil type                        |
| LIT\_BOOL         | true          | A Boolean literal                                 |
| LIT\_CHAR         | 'a'           | A character literal (i32 for UTF-8 encoding)      |
| LIT\_STRING       | "foo"         | A stack allocated string literal                  |
| LIT\_SYMBOL       | :foo          | A symbol literal                                  |

# Meta-tokens
| Lexer             | Literal       | Description                                       |
|-------------------|---------------|---------------------------------------------------|
| META\_MACRO       | @puts         | A declarative macro                               |
| META\_PRE\_MACRO  | #[derive(Eq)] | A procedural macro                                |
| META\_COMMENT     | # comment     | A single-line comment, removed by pre-processor   |
| META\_DOC\_STRING | """doc"""     | A multi-line documentation string                 |
| META\_EOF         |               | Token to represent end of file                    |
| META\_ILLEGAL     |               | Token to represent unrecognized character         |

