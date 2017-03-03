import Helper(primesTo, digits)
import Data.List (sort)
import Control.Applicative((<$>))

main :: IO ()
main = print euler41

euler41 :: Integer
euler41 = head $ filter isPandigital $ reverse primes

primes = primesTo 7654321 --maximum pandigital

isPandigital :: Integer -> Bool
isPandigital n = [1..ndig] == dig
	where
		ndig = length dig
		dig = sort $ fromIntegral <$> digits n