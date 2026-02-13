# Editron
*A Programming Language for Video Editing*

## Project Goals

Editron is a domain-specific programming language designed for video editing and media processing.

### Targets
1. Simple, readable syntax inspired by Python.  
2. Automatic memory management, refer to [Memory Architecture of Editron](docs/Memory.md)  
3. High performance suitable for processing large video files  
4. Robust file handling for different media formats  
5. A unified language that exposes common and advanced editing tools  

---
# Lexer
- The Lexer's job is to convert the bytes inside the file to a set of Tokens that we can work on. 
- In Editron the Lexer returns a `Vec<Token>` and which is used by the Parser to implement the IR.
- For Detailed Architecture, Look at : [Lexer Architecture](docs/Lexer.md)

# Parser
- The Parser's job is to understand the tokens emitted by the Lexer and give meaning to it, the parser is what gives the language it syntax and semantics
- Unlike traditional compilers that build an AST first, Editron’s parser directly emits an IR optimized for media execution.

- You can take a look at it's current state here: [Parser](docs/Parser.md)


# The Media Processing Engine
The execution core of Editron.  \
Responsible for decoding, frame-level manipulation, audio processing, and encoding.\
- For now Editron uses FFMPEG for Decoding and Encoding of file types and does require you to have FFMPEG installed on your system, for details check out the requirements section.
- Editron introduces some DataTypes which are completely unique to Editron, They are Listed below:
1. Frame: The Frame Type is used to store and work with a single image. Check : [The Frame Type](docs/Frame.md)
2. Clip: The Clip Type is a wrapper around the `Vec<Frame>`  and is limited dynamically according to the resolution. [The Clip Type](docs/Clip.md)
3. Track: The Track Type is built specifically to process the audio files. [The Track Type](docs/Track.md)
4. Video: Video type is used to work with Video files as the name suggests. [The Video Type](docs/Video.md)
- Clip is intentionally memory-bounded (~100MB default).  
- For full-length processing, users should iterate over the Video type.

## Current Status


- Lexer: Stable for basic syntax.
- Parser: Emits IR for simple constructs.
- Frame: Basic RGB24 support implemented.
- Image IO:
  - `load_image_rgb()`
  - `export_frame_to_png()`
- PixelFormat currently fixed to RGB24.

## Future Updates may Include:
- Implementation for other types
- Prelude, to allow Editron to access the features and Types I have built.
- A shift to `ffmpeg-next` from CLI.

## Requirements

Editron v0.1 has been tested with:

- **FFmpeg 8.0.1 (full_build, gyan.dev)**
- Windows x64 (MSYS2 GCC build)

Older versions may work, but are not officially tested.
