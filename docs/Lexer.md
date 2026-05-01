# Lexer

The lexer is responsible for converting source code into a sequence of tokens.

It is implemented as a deterministic state machine and produces a `Vec<Token>` that is consumed by the parser.

---

## Overview

The lexer processes input as a byte stream and emits structured tokens representing identifiers, literals, and syntax elements.

Each token contains:

- `kind`: token category (`TokenKind`)
- `value`: lexeme as a string
- `line`: line number for error reporting

---

## Token Representation

### Token
```
struct Token {
kind: TokenKind,
value: String,
line: usize,
}
```

### TokenKind

Supported token categories include:

- Identifiers
- Keywords:
  - `load`
  - `filter`
  - `export`
  - `import`
- Literals:
  - Integer
  - Float
  - String
- Symbols:
  - Parentheses `(` `)`
  - Braces `{` `}`
  - Brackets `[` `]`
  - Comma `,`
  - Semicolon `;`
  - Dot `.`
  - Range operator `..`
  - Assignment `=`
- End-of-file marker (`EOF`)

---

## Lexer States

The lexer operates using an explicit state machine:

- `Default`  
  Handles whitespace, symbols, and transitions to other states

- `Identifier`  
  Parses identifiers and keywords

- `String`  
  Parses string literals

- `Number`  
  Parses numeric values (integer and float)

This design ensures controlled transitions and avoids ambiguous token emission.

---

## State Behavior

### Default State

- Skips whitespace (` `, `\t`, `\r`)
- Tracks newlines for accurate line numbers
- Emits single-character tokens via symbol mapping
- Transitions to:
  - `Identifier` when encountering alphabetic characters or `_`
  - `Number` when encountering digits
  - `String` when encountering `"`

---

### Identifier State

- Consumes alphanumeric characters and `_`
- Classifies the result as:
  - Keyword (if reserved)
  - Identifier (otherwise)
- Returns to `Default` state without consuming the terminating character

---

### String State

- Accumulates characters until a closing `"`
- Supports multiline strings (tracks line numbers)
- Emits a `String` token on completion
- Produces an error if the string is not terminated

---

### Number State

- Parses integer and floating-point values
- Detects floats based on presence of `.`
- Handles negative numbers (`-`) only when followed by digits
- Differentiates between:
  - Float literals
  - Range operator (`..`)

Invalid numeric formats produce an error.

---

## Helper Functions

### char_to_token()

Maps single-character symbols to their corresponding token types.

### identify_token()

Determines whether a parsed identifier is a keyword or a user-defined identifier.

---

## Error Handling

Errors are represented using `LexError`:

- `InvalidCharacter`
- `UnterminatedString`
- `InvalidNumber`

Each error includes:

- Line number
- Contextual message

---

## End of Input

After processing all input, the lexer emits an explicit `EOF` token.

---

## Current Capabilities

- Deterministic tokenization via state machine
- Support for identifiers, keywords, literals, and symbols
- Line-aware error reporting
- Basic numeric and string parsing
- Range operator (`..`) handling

---

## Limitations

- No support for comments
- No escape sequences in strings
- Limited operator set
- No unicode handling beyond basic ASCII

---

## Summary

The lexer provides:

- A structured token stream for parsing
- Deterministic behavior via explicit state transitions
- Basic error handling for malformed input

It serves as the first stage of the Editron DSL pipeline.