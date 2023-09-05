#!/usr/bin/env fish

set commands ()

function main
    echo "valid commands are [$commands]"
end

set cmd $argv[1]
switch $cmd
    case in $commands
        $cmd
    case "*"
        main
end
