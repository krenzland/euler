import Helper (primesTo, digits)
import Data.List (permutations)

main :: IO ()
main = print $ primes

euler35 = undefined 

primes = map permutations $ map digits $ primesTo 20

listToNumber :: (Integral a) => [a] -> a -> a
listToNumber [] aku = 0
listToNumber (x:xs) aku = 10^aku * x + (listToNumber xs (aku + 1))