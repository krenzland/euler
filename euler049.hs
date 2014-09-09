import Helper (primesTo, digits, nubQ)
import Data.List (permutations, sort)

main :: IO ()
main = print euler49

euler49 :: [String]
euler49 = fmap concatList perPrimes

concatList :: Show a => [a] -> String
concatList xs = foldl1 (++) $ fmap show xs

perPrimes :: [[Integer]]
perPrimes = [[a,b,c] | a <- primes, b <- primes, c <- primes, a < b, b < c, correctGap [a,b,c], arePerms [a,b,c]]
	where
		correctGap [a,b,c] = b - a == c - b		
		primes = [ x | x <- primesTo 9999, x >= 1000, x <= 9999]

arePerms :: [Integer] -> Bool
arePerms xs@[a,b,c] = (length . nubQ $ filter isPerms xs) == 3
	where
		isPerms n = digits n `elem` (permutations . digits) a 