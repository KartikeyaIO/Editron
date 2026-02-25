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
- When I write `let a = load("image.jpg");` It will be translated to Tokens like : let, identifier a, equal, identifier load, opening ( and closing ) and in between these String that contains the path.
- Read about Infix, prefix, and suffix, operations.
- prefix notations


# Engine
## Pulse Code Modulation:
Pulse Code Modulation (PCM) is a method used to digitally represent analog signals by sampling their amplitude at uniform intervals and converting them into a binary sequence (
s and 
s). It is the standard for digital audio in computers, CDs, and telephony, involving three key steps: sampling, quantization, and coding to transform continuous signals into discrete data

- Process: The analog signal is sampled, then quantized (rounded to the nearest digital value), and finally coded into a binary stream.
Advantages: PCM offers high noise immunity,, improved signal-to-noise ratio, and efficient long-distance communication.
- Applications: Widely used in digital audio (CDs, WAV files), digital telephony, and data conversion.
- Standards: Uses time-division multiplexing, typically with 24 channels (North American/Japanese) or 30 channels (European/Indian).
- Types: Differential PCM (DPCM) and Adaptive Differential PCM (ADPCM) are variants for specialized applications
