#!/bin/bash
set -xe
FLAGS='-Wall -Wextra -Wconversion -pedantic -g'
cc -I../src ../src/main.c ../src/dict.c ../src/set.c -o ana $FLAGS -lm