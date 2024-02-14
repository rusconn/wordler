# wordler

A [Wordle](https://www.nytimes.com/games/wordle/index.html) helper.

## Usage

### Install

```shell
cargo install --path .
```

### Example

Find the word **RUSTY**

```shell
> wordler
Word: STERN # also lowercase is ok
Hint: 11010 # 0: not exists, 1: wrong position, 2: correct position
Remaining: Too many, didn't print: 135
Recommend: [AUDIO,MIAOU,AULOI,OUIJA,OUSIA]

Word: AUDIO
Hint: 02000
Remaining: [BURST,CURST,HURST,HURTS,RUSTS,RUSTY,RUTHS,TURFS,TURKS,TURMS,TURPS,WURST,YURTS]
Recommend: [HACKY,CHOKY,MOCHY,PHYMA,HUMPY]

Word: HACKY
Hint: 00002
Found: RUSTY
```
