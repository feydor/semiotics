# prim
primitive method, accumulation, tallying  
(₱4 == four == PD-0) -> inf
```
USAGE:
    prim <QUERY>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <QUERY>    a numonym of length < 999

EXAMPLE:
    prim zero

    +-- Primitive Method (₱) --+
    query: 'zero' has ₱4
    PRIM  ANGL.  PD
    ₱4    four   0
    +--------------------------+
```
## TODO:
- err: crashes at 100 (one hundred)
    - fix: 100 is two numonyms with an implicit third - 'zero'
- err: gives the wrong PD when greater than 100
    - ex: vauungfdklsjkdhfkdjsfjskfjkdshfjksdhfjksdhfjkhsdkjfhsdkjfeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee, PD should be 101-13=88 but is PD=95
