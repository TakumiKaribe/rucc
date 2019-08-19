#!/bin/sh

try() {
    expected="$2"
    input="$3"

    ARG=true DEBUG="$1" cargo run "$input" > tmp.s
    gcc -o tmp tmp.s
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$expected expected, but got $actual"
        exit 1
    fi
}

try $1 21 '5+20-4;'
try $1 41 ' 12 + 34 - 5; '
try $1 47 '5+6*7;'
try $1 15 '5*(9-6);'
try $1 4 '(3+5)/2;'

echo OK
