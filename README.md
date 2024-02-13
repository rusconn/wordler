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
> wordler /usr/share/dict/words
Word: AUDIO # also lowercase is ok
Hint: 02000 # 0: not exists, 1: wrong position, 2: correct position
Remaining: Too many, didn't print: 315
Recommend: YRESL

Word: YRESL
Hint: 11010
Remaining: [RUSHY,RUSKY,RUSTY,SURFY,SURGY]
Recommend: TFHKG

Word: TFHKG
Hint: 10000
Found: RUSTY
```
