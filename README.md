1. Constants:
   - `DIGITS`: A string constant that represents the valid digits.

2. Error Handling:
   - `Error` struct: Represents an error that can occur during tokenization.
     - `pos_start` and `pos_end`: Store the start and end positions of the error.
     - `error_name` and `details`: Store the error name and additional details.
     - `as_string()`: Returns a formatted string representation of the error.
   - `Position` struct: Represents a position in the source code.
     - `idx`, `ln`, and `col`: Store the index, line number, and column number.
     - `fn_name` and `ftxt`: Store the filename and source code text.
     - `advance()`: Advances the position based on the current character.
     - `copy()`: Creates a copy of the position.

3. Token Types:
   - `TokenType` enum: Represents the types of tokens.
     - `INT`, `FLOAT`, `PLUS`, `MINUS`, `MUL`, `DIV`, `LPAREN`, `RPAREN`.

4. Tokens:
   - `Token` struct: Represents a token.
     - `type_` and `value`: Store the token type and optional value.
     - `new()`: Creates a new token.

5. Lexer:
   - `Lexer` struct: Responsible for tokenizing the input text.
     - `fn_name` and `text`: Store the function name and input text.
     - `pos` and `current_char`: Store the current position and character.
     - `new()`: Creates a new lexer instance.
     - `advance()`: Advances the position and current character.
     - `make_tokens()`: Tokenizes the input text.
     - `make_number()`: Processes a number token.

6. `run()` Function:
   - Entry point of the program.
   - Creates a lexer instance, tokenizes the input text, and returns the tokens and any error.

7. `main()` Function:
   - Executes the main logic of the program.
   - Calls `run()` with a function name and input text.
   - Prints the tokens and any error.

The program follows these steps:
- The `run()` function is called from `main()`, passing the function name and input text.
- An instance of `Lexer` is created with the function name and input text.
- The `make_tokens()` method of `Lexer` is called to tokenize the input text.
- Inside `make_tokens()`, the characters of the input text are processed one by one.
  - Whitespace characters are skipped.
  - If a digit is encountered, the `make_number()` method is called to process the number token.
  - If any other character is encountered, an `Illegal Character` error is created.
- The `make_number()` method processes a number token by collecting consecutive digits and an optional decimal point.
- The tokens and any error are returned from `run()`.
- In `main()`, the tokens and error are printed to the console.

The code essentially tokenizes an input mathematical expression into individual tokens representing numbers and operators. It uses a lexer to process the input text character by character and generates tokens accordingly.
