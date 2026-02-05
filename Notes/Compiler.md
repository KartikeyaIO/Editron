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

# Second Step : Parser
for parser, I will not be building Abstract Syntax Tree like Traditional compilers, instead we will build Intermediate Representation.
## What I am thinking?
1. I will define some structs and Enums:
- The command struct stores the the command and it's arguments.
- The command struct will actually store the commands
- I will define Enums which define command type like addition, substraction, file open, file closing and all that,
- Based on the token stream we recieve from the lexer we will push in a vector<command> in the exact same order corresponding to the execution order and then I we will pass the commands and it's execution order to the engine which will decipher and perform the commands hence, compiling the high level code.
