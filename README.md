# Rag (Python terminal language) scaffold

This project scaffolds a **Python-powered terminal environment** for a C/C++-style beginner-friendly language.

## Folders

- `core/`: interpreter engine (parser, executor, installer)
- `libs/45m/`: built-in `45m` core library (auto-available)
- `packages/`: installed `.rag` packages
- `projects/`: user projects (`.rag` files)

## Run

```powershell
cd "C:\Users\Anass\OneDrive\PC\Desktop\.i"
python main.py
```

## Custom words (keywords)

Edit `core/keywords.py` to change your language name and add your own words.

Included by default:

- `say "text";` (same as `io_println("text");`)
- `make x = fs_read_text("a.txt");` (same as `let x = ...`)
- `give 0;` (same as `return 0;`)

