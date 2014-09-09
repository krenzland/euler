{-# LANGUAGE OverloadedStrings #-}

import Text.ParserCombinators.Parsec hiding (spaces)
import Data.Text(split)
import System.Environment

main :: IO ()
main = euler22

euler22 = do
	names <- readFile "p022_names.txt"
	print names

parseNames :: Parser String
parseNames = do
			x <- many (noneOf "\"")
			return x