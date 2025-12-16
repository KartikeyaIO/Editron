# Book Reffered: Crafting Interpreters

## Steps involved:
## 1. Scanning (lexing):
it is kind of taking the data as string and divide each words or important symbols as tokens 
## 2. parsing:
at this step we create trees (parse trees or abstract syntax tree)


# First step : Lexer
for lexer I will be defining a struct and then I will follow the following rules:
1. Each word is one token and is basically considered an identifier until proven otherwise
2. Each punctuation is treated as a token -> () =  two tokens for opening and closing
3. Everything inbetween "" is one token to reserve strings
4. Numbers are one token
