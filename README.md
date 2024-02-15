# wordler

A [Wordle](https://www.nytimes.com/games/wordle/index.html) helper.

## Install

```shell
cargo install --path .
```

## Example

Find the word **RUSTY**

```text
> wordler
Remaining: Too many, didn't print: 14855
Recommend: [AEROS,AROSE,SOARE,REAIS,SERIA]
Word: SERIA # also lowercase is ok
Hint: 10100 # 0: not exists, 1: wrong position, 2: correct position

Remaining: Too many, didn't print: 145
Recommend: [DOUBT,POUTY,UNPOT,PUTON,PUNTO]
Word: DOUBT
Hint: 00101

Remaining: [RUNTS,RUSTS,RUSTY,RUTHS]
Recommend: [RHYNE,HINNY,HENNY,HYENS,HYENA]
Word: RHYNE
Hint: 20100

Found: RUSTY
```
