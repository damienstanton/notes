#!/usr/bin/env fish

set commands distsys

function distsys
    set exe $argv[2]
    cargo b && ./src/bin/maelstrom test -w $exe \
        --bin ./target/debug/$exe \
        --node-count 1 \
        --time-limit 10
end

function help
    echo "valid commands are [$commands]"
end

function main
end

set cmd $argv[1]
switch $cmd
    case in $commands
        $cmd $argv
    case help
        help
    case "*"
        main
end
