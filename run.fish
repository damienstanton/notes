#!/usr/bin/env fish

set commands distsys

function distsys
    cargo b && ./src/bin/maelstrom test -w echo \
        --bin ./target/debug/notebook \
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
        $cmd
    case help
        help
    case "*"
        main
end
