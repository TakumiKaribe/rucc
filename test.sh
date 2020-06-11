#!/bin/sh

assert() {
    expected="$1"
    input="$2"

    cargo run "$input" >tmp.s
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

assert 0 './test/01'
assert 42 './test/02'
assert 21 './test/03'
assert 41 './test/04'
assert 47 './test/05'
assert 15 './test/06'
assert 4 './test/07'
assert 10 './test/08'
assert 10 './test/09'
assert 10 './test/10'

assert 0 './test/11'
assert 1 './test/12'
assert 0 './test/13'

assert 1 './test/14'
assert 0 './test/15'
assert 0 './test/16'
assert 1 './test/17'
assert 1 './test/18'
assert 0 './test/19'

assert 1 './test/20'
assert 0 './test/21'
assert 0 './test/22'
assert 1 './test/23'
assert 1 './test/24'
assert 0 './test/25'

echo OK
