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
    only nausea and fear are there more
    ...
    The labyrinth interior intoxicated by the murmur told has ‘no assignable meaning’
    epistemic ideological or moral that drives the artist into repression

```

## performance

Using 'srx/txt/*' and generating 1,000 words:
- first attempt - 9.419s
- replacing string comparison loop with std::copy_if - 4.597s

ngram version, 1,000,000 words, N=4
uses iterators instead of raw std::string (relevant to copies)
- first attempt - 5.144s
- replace use of true random value with psuedo-random (std::mt19937) - 1.066s
