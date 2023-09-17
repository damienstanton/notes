#!/usr/bin/env fish

function echo-test
    cargo b && ./src/bin/maelstrom test -w echo \
        --bin ./target/debug/echo \
        --node-count 1 \
        --time-limit 10
end

function unique-ids
    cargo b && ./src/bin/maelstrom test -w unique-ids \
        --bin ./target/debug/unique-ids \
        --node-count 3 \
        --time-limit 30 \
        --availability total \
        --nemesis partition
end

function help
    echo "valid commands are [$commands]"
end

function main
    help
end

set cmd $argv[1]
switch $cmd
    case book
        mdbook serve --open
    case distsys
        set sub $argv[2]
        switch $sub
            case unique-ids
                unique-ids
            case echo
                echo-test
            case "*"
                echo "unrecognized maelstrom test: $sub"
            case help
                help
            case "*"
                main
        end
end
