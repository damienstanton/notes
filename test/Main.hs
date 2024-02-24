module Main (main) where

import Notes (foo)
import Test.Hspec (describe, hspec, it, shouldBe)

main :: IO ()
main = hspec $ do
  describe "foo func" $ do
    it "says 'OK'" $ do
      foo `shouldBe` "OK"
