#!/bin/bash
set -xe
FLAGS='-Wall -Wextra -g'
cc ../src/main.c -o ana $FLAGS -lm