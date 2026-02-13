## Parser (Current State)

The parser converts a linear token stream into a simple instruction-based IR.

At this stage, the parser does **not** build an AST. Instead, it emits IR instructions directly while parsing expressions. This keeps the system small and allows the instruction set to evolve based on engine requirements.

### What It Supports

- `let` statements
- Integer and string literals
- Basic arithmetic: `+ - * /`
- Operator precedence via recursive descent
- Compile-time evaluation of constant expressions

### Compile-Time vs Runtime Values

During parsing, values are classified as:

- **Compile-time constants** (evaluated immediately)
- **Runtime values** stored in registers

Constant-only expressions are folded during parsing.  
Expressions involving runtime values emit IR instructions.

### IR Emission

Instructions are pushed incrementally during parsing:

- String literals emit a `LoadString` instruction
- Arithmetic operations emit instructions only when runtime evaluation is required
- Registers are allocated sequentially

The output of the parser is a linear IR program intended to be executed by the engine.

### Notes

- Error handling is minimal and uses panics
- No control flow or functions yet
- Design is intentionally simple and incomplete

This parser exists to unblock engine development and will likely be revisited once execution semantics stabilize.
