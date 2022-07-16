#!/bin/bash
set -xe
FLAGS='-Wall -Wextra -Wconversion -pedantic -g'
cc ../src/main.c -o ana $FLAGS -lm