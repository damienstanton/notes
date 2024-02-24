module Main where

import Notes qualified (foo)

main :: IO ()
main = do
  print Notes.foo