# Declaring variables with let

One way to declare a variable is to declare a binding using the keyword let. Any declaration of a variable using let must be followed by a variable identifier. Variable identifiers may not be reserved words such as let, but can be any other combination of underscores and letters followed by any combination of underscores, letters, and digits optionally terminated by a bang or a question mark.

When a binding is declared, a few things occur.

First, the identifier is hashed and placed into the current scope. Scope is a kind of context and is maintained as a hierarchy of hash tables. The current scope is a hash table that will be dropped when the current scope ends. When a scope is dropped, everything that still remains in scope is dropped and deallocated. The end of a scope is always marked with the end reserved word.

Second, when the identifier hash is put into the current scope, it is put into the current scope with a default scope record which is called a binding. A binding includes a binding type which is either mutable or immutable (defaults to immutable), a ref type which is either a pointer, a box, or a borrow (defaults to pointer), a value type which is either undefined or a symbolic type (defaults to undefined), and a usize mem address which is defaulted to null.

Third, a binding which uses the mut keyword is set to have a mutable binding type.

Fourth, a binding with an explicit type annotation is set to have that value type.

Fifth, if the binding is composed with an assignment, then the result of the assignment expression is allocated on the stack. The mem address is then put into the current scope using the identifier hash of the binding. If the result of the expression is a value, then the ref type of the binding is set to pointer. If the result is a box ref, then the ref type of the binding is set to box. Otherwise, if the result of the expression is a borrow, then the ref type of the binding is set to borrow. Then, the type of the result of the expression is checked against the value type of the binding. If the value type of the binding is undefined, then the value type is overwritten with the type of the result of the expression. Otherwise, the two types are checked for equality. Note, there is no value type for pointer, box, or borrow. These ref types are treated as similar monadic types and as such cannot be composed. Therefore, it is sufficient to have a ref type and a value type on each binding.
