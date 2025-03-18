# wordler

A [Wordle](https://www.nytimes.com/games/wordle/index.html) helper.

## Install

```shell
cargo install --git https://github.com/rusconn/wordler.git
```

## Example

Find the word **RUSTY**

```text
> wordler
Remaining: Too many, didn't print: 14855
Recommend: [AEROS,AROSE,SOARE,REAIS,SERIA]
Guess: SERIA # also lowercase is ok
Hints: 10100 # 0: not exists, 1: wrong position, 2: correct position

Remaining: Too many, didn't print: 145
Recommend: [DOUBT,POUTY,PUTON,OUTBY,PUNTO]
Guess: DOUBT
Hints: 00101

Remaining: [RUNTS,RUSTS,RUSTY,RUTHS]
Recommend: [HYENS,HYSON,HORNY,HONEY,HUNKY]
Guess: HYSON
Hints: 01200

Found: RUSTY
```
