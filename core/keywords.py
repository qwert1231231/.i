from __future__ import annotations

# Edit this file to customize your language words.

LANGUAGE_NAME = "Rag"

# Keyword aliases (parser level)
KW_LET = {"let", "make"}       # "make x = ..."
KW_RETURN = {"return", "give"} # "give 0"

# Statement sugar:
# say "hi"   -> io_println("hi")
SAY_WORDS = {"say", "printl"}

# Function aliases (executor level)
FUNCTION_ALIASES = {
    # your_word: real_45m_function_name
    "println": "io_println",
    "write": "fs_write_text",
    "append": "fs_append_text",
    "read": "fs_read_text",
}

