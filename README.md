# Editron
*A Programming Language for Video Editing*

## Project Goals

Editron is a domain-specific programming language designed for video editing and media processing.

### Targets
1. Simple, readable syntax inspired by Python  
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

# Parser
- The Work on Parser is postphoned and I will be working on the actual editing engine and image, audio and video processing because I have never \
worked with trees so I need to learn tree structures which will take sometime and until then I can research on media processing the issues and techniques I should or can work on.
