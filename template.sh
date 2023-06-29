#!/bin/bash

function argument_error {
    echo "$1: expected $2 argument(s), given $3" >&2
    exit 1
}

function check_not_file {
    if [ -f "$1" ]; then
        echo "file already exists: $1" >&2
        exit 1
    fi
}

function check_not_dir {
    if [ -d "$1" ]; then
        echo "directory already exists: $1" >&2
        exit 1
    fi
}

function check_file {
    if [ !  -f "$1" ]; then
        echo "file missing: $1" >&2
        exit 1
    fi
}

function check_dir {
    if [ ! -d "$1" ]; then
        echo "directory missing: $1" >&2
        exit 1
    fi
}

function check_reserved {
    if [ "$1" = "mod" ]; then
        echo "filename $1 is reserved and cannot be used." >&2
        exit 1
    fi
}

function check_reserved_batch {
    for arg in "$@"; do
        check_reserved "$arg"
    done
}

function register {
    if [[ $# -eq 1 ]]; then
        modname=$1
        file=./src/problems/mod.rs
    elif [[ $# -eq 2 ]]; then
        modname=$2
        file=./src/problems/$1/mod.rs
    else
        argument_error "$0" "1 or 2" $#
    fi
    echo "register module $modname to $file"
    echo "mod $modname;" >> "$file"
}

function ensure_register {
    if [[ $# -ne 1 ]]; then
        argument_error "$0" "1" $#
    fi
    dir=./src/problems/$1
    if [ ! -d "$dir" ]; then
        register "$1"
    fi
}

function create_flat {
    if [[ $# -ne 1 ]]; then
        argument_error "$0" "1" $#
    fi
    template=./template.rs
    base=./src/problems/$1
    target=$base.rs

    echo "create problem $1 in $target"
    check_not_dir "$base"
    check_not_file "$target"
    cp "$template" "$target"
    register "$1"
}

function create_nested {
    if [[ $# -ne 2 ]]; then
        argument_error "$0" "2" $#
    fi
    template=./template.rs
    base=./src/problems/$1
    target=$base/$2.rs

    echo "create alternative $2 for problem $1 in $target"
    check_not_file "$base"
    check_not_file "$target"
    ensure_register "$1"
    mkdir -p "$base"
    cp "$template" "$target"
    register "$1" "$2"
}

function create_multi {
    if [[ $# -le 1 ]]; then
        argument_error "$0" "more than 1" $#
    fi
    for sub in "${@:2}"; do
        create_nested "$1" "$sub"
    done
}

function create {
    if [[ $# -eq 1 ]]; then
        create_flat "$1"
    elif [[ $# -ge 2 ]]; then
        create_multi "$@"
    else
        argument_error "$0" "at least 1" $#
    fi
}

function migrate {
    if [[ $# -ne 2 ]]; then
        argument_error "$0" "2" $#
    fi
    base=./src/problems/$1
    flat=$base.rs
    nested=$base/$2.rs

    echo "migrate problem $1 from $flat to alternative $2 in $nested"
    check_file "$flat"
    check_not_dir "$base"
    mkdir -p "$base"
    mv "$flat" "$nested"
    register "$1" "$2"
}

command=$1
shift
check_reserved_batch "$@"
if [ "$command" = "create" ]; then
    create "$@"
elif [ "$command" = "migrate" ]; then
    group=$1
    shift
    migrate "$group" "$1"
    shift
    create "$group" "$@"
else
    echo "unknown command $1" >&2
fi