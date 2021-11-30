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
- first attempt - 9.419s
- replacing string comparison loop with std::copy_if - 4.597s
- replace 'useless' character-stripping for loop with std::remove_if - 4.401s