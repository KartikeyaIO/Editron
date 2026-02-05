# Editron
*A Programming Language for Video Editing*

## Project Goals

Editron is a domain-specific programming language designed for video editing and media processing.

### Targets
1. Simple, readable syntax inspired by Python.  
2. Automatic memory management (garbage-collection–style abstraction)  
3. High performance suitable for processing large video files  
4. Robust file handling for different media formats  
5. A unified language that exposes common and advanced editing tools  

---

## Lexer

The lexer is implemented in Rust and is responsible for converting source code into a stream of tokens.  
It is designed as an explicit **state machine**, which ensures correctness and clear token boundaries.

---

## Token Representation

### `Token`
Each token contains:
- `kind`: the token type (`TokenKind`)
- `value`: the lexeme as a string
- `line`: line number for error reporting

### `TokenKind`
Defines all supported token categories, including:
- Identifiers
- Keywords (`if`, `else`, `let`, `fn`, `return`, etc.)
- Operators and punctuation
- String literals
- (Update) Addition of SemiColon, yeah I realised life is not that easy without them.
- End-of-file marker (`EOF`)

---

## Lexer States

The lexer operates using the following states:

- `Default` – normal scanning mode  
- `Identifier` – reading identifiers or keywords  
- `String` – reading string literals  
- `Number` – reading Numeric Values

This approach prevents premature token emission and keeps lexing logic deterministic.

---

## Helper Functions

### `char_to_token()`
Maps single-character symbols (parentheses, braces, operators) to their corresponding `TokenKind`.

### `identify_token()`
Classifies a completed word:
- Returns a keyword token if the word matches a reserved keyword
- Otherwise returns `Identifier`

---

## `lexer()` Function

The `lexer()` function performs the full tokenization process.

### Initialization
- Reads the source file (`.edt`)
- Converts the source into a byte array for indexed access
- Initializes:
  - cursor index
  - line counter
  - current lexer state
  - temporary buffer for words and strings

---

### State Handling

#### Default State
- Skips whitespace and tabs
- Tracks newlines for accurate line numbers
- Transitions to:
  - `Identifier` state when encountering letters or `_`
  - `String` state when encountering `"`
- Emits single-character tokens via `char_to_token()`

#### Identifier State
- Consumes alphanumeric characters and `_`
- Emits either a keyword or identifier token upon termination
- Returns to `Default` state without consuming the terminating character

#### String State
- Accumulates characters until a closing `"`
- Emits a `String` token
- Tracks newlines inside strings
- Returns to `Default` state

#### Number State
- The compiler enters the Number state when a numeric value is encountered
- if checks whether the number contains a `.` or not and classifies the number as Int or Float
- Returns the corresponding token or an Error based if the Number is Invalid 
---

### End of Input
After processing the entire source, an explicit `EOF` token is emitted.

---

## Current Status

The lexer foundation is complete and structurally stable.
Updates:  
- Added Numeric values
- Added Custom Errors

Planned extensions:
- Comments
- Escaped characters in strings
- Multi-character operators

---

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
