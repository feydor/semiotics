# markov
NLP on Thirst for Annihilation
```
USAGE:
    markov [FLAGS] [QUERY]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <QUERY>    the text
```

## performance

Using 'srx/txt/*' and generating 1000 words:
- first attempt - 9.419 s
- replacing string comparison loop with std::copy_if - 4.597 s