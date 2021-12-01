# markov
NLP on Thirst for Annihilation
```
markov 0.1.0

USAGE:
    markov <path> [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <path>          
    <word-count>     [default: 100]
    <ngram>          [default: 4]

EXAMPLE:
    markov ./src/txt/land-thirst-for-annihilation 100 10

    ...
    The technical immortal blemish of mankind n’en est pas
    ...
    The labyrinth interior intoxicated by the murmur told has ‘no assignable meaning’ epistemic ideological or moral that drives the artist into repression

```

## performance

Using 'srx/txt/*' and generating 1,000 words:
- first attempt - 9.419s
- replacing string comparison loop with std::copy_if - 4.597s
- replace 'useless' character-stripping for loop with std::remove_if - 4.401s

ngram version, log(n) time, 1,000,000 words, N=4
- first attempt - 5.144s