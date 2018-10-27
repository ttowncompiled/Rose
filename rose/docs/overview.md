# Rose lang overview

# Keywords
| Lexer         | Keyword       | Description                                       |
|---------------|---------------|---------------------------------------------------|
| KW\_ABSTRACT  | abstract      | Declares an abstract type or routine definition   |
| KW\_AND       | and           | Logical and                                       |
| KW\_AS        | as            | Casts left type to right type, aliases imports    |
| KW\_BEGIN     | begin         | Declares a basic block                            |
| KW\_BORROW    | borrow        | Allocates a stack-to-stack pointer                |
| KW\_BREAK     | break         | Terminates execution of a block                   |
| KW\_CLASS     | class         | A stateful interface type                         |
| KW\_CONST     | const         | For const bindings (immutable, no-shadowing)      |
| KW\_CONTINUE  | continue      | Jumps within the block                            |
| KW\_DEF       | def           | Defines a routine for the dispatcher              |
| KW\_DO        | do            | Declares an isolated block                        |
| KW\_ELSE      | else          | Alternative branch                                |
| KW\_END       | end           | Marks the end of a block                          |
| KW\_ENUM      | enum          | Enumerated type                                   |
| KW\_EXT       | ext           | Declares ext relationship                         |
| KW\_FALSE     | false         | The false Boolean literal                         |
| KW\_FINAL     | final         | Declares a final binding                          |
| KW\_FN        | fn            | Anonymous closure definition                      |
| KW\_FOR       | for           | Loop for iterable collections                     |
| KW\_HAS       | has           | Declares has relationship                         |
| KW\_IF        | if            | Primary branch                                    |
| KW\_IM        | im            | The imaginary type postfix unary operator         |
| KW\_IMPL      | impl          | Declares an impl relationship                     |
| KW\_IN        | in            | Binds left var to right collection, membership    |
| KW\_INF       | Inf           | Infinity                                          |
| KW\_IS        | is            | Strict equality check                             |
| KW\_LET       | let           | Declares a variable binding                       |
| KW\_LOOP      | loop          | Basic loop block, classic for-loop                |
| KW\_MACRO     | macro         | Declares a custom macro                           |
| KW\_MATCH     | match         | Pattern matching, switch expression               |
| KW\_MOD       | mod           | Declares module, imports local module             |
| KW\_MOVE      | move          | Moves binding to new owner                        |
| KW\_MUT       | mut           | Mutable binding                                   |
| KW\_NAN       | NaN           | Not a number, Inf/Inf                             |
| KW\_NEW       | new           | Allocates heap memory and a stack-to-heap pointer |
| KW\_NIL       | nil           | The nil or null type                              |
| KW\_NOT       | not           | Logical not, prefix unary operator                |
| KW\_OR        | or            | Logical or                                        |
| KW\_OVERLOAD  | overload      | Declares a routine overload for the dispatcher    |
| KW\_OVERRIDE  | override      | Declares a routine override for the dispatcher    |
| KW\_PRO       | pro           | Protected type or binding                         |
| KW\_PUB       | pub           | Publicly available type or binding                |
| KW\_REF       | ref           | Keyword used for matching borrows and boxes       |
| KW\_RETURN    | return        | Terminates block and returns right value          |
| KW\_SELFVALUE | self          | Instance of the current context                   |
| KW\_SELFTYPE  | Self          | Type of the current context                       |
| KW\_STATIC    | static        | Declares a static binding (singleton, eternal)    |
| KW\_STRUCT    | struct        | Declares a struct type                            |
| KW\_SUPERVALUE| super         | Parent of the instance of the current context     |
| KW\_SUPERTYPE | Super         | Type of the parent of the current context         |
| KW\_TRAIT     | trait         | A stateless interface type                        |
| KW\_TRUE      | true          | The true Boolean literal                          |
| KW\_TUPLE     | tuple         | Declares a tuple type                             |
| KW\_TYPE      | type          | Declares a type alias                             |
| KW\_TYPEOF    | typeof        | Casts a var to its symbolic type                  |
| KW\_USE       | use           | Imports module from root                          |
| KW\_USES      | uses          | Declares uses relationship                        |
| KW\_VIRTUAL   | virtual       | Declares a virtual routine definition             |
| KW\_WHERE     | where         | Declare guard(s)                                  |
| KW\_WHILE     | while         | While loop                                        |
| KW\_WITH      | with          | Declares bindings for block                       |
| KW\_XOR       | xor           | Logical xor                                       |
| KW\_YIELD     | yield         | Return call for generators                        |

