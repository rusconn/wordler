# wordler

A [Wordle](https://www.nytimes.com/games/wordle/index.html) helper.

## Usage

### Install

```shell
cargo install --path .
```

### Example

Find the word **WATCH**

```shell
> wordler /usr/share/dict/words
Word: AUDIO # also lowercase is ok
Hint: 10000 # 0: not exists, 1: wrong position, 2: correct position
Remaining: Too many, didn't print: 1691
Recommend: [ERNST,STERN,RESTY,STREY,TYLER]

Word: STERN
Hint: 01000
Remaining: Too many, didn't print: 79
Recommend: [LYMPH,CHYLE,GLYPH,HYLIC,LOCHY]

Word: GLYPH
Hint: 00002
Remaining: [BATCH,CATCH,HATCH,MATCH,WATCH]
Recommend: [CLAMB,CLIMB,CLOMB,COMBY,COOMB]

Word: CLIMB
Hint: 10000
Remaining: [HATCH,WATCH]
Recommend: [ABLOW,ABWAB,ADAWE,ADAWN,ADOWN]

Word: ADOWN
Hint: 10010
Found: WATCH
```
