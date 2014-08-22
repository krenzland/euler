import Helper (primesTo, nubQ)
import Data.Maybe(fromMaybe)


main :: IO ()
main = print $ euler12

euler12 :: Integer
euler12 = snd $ head $ dropWhile (\x -> (length $ divisors x) < 500) $ factorList
			where
				factorList = fmap factorTriangularNumber [1..]


divisors :: (Integral a) => ([a], a) -> [a]
divisors (factors, number) = nubQ $ [x*y | x <- factors, y <- factors, (number `rem` (y * x)) == 0] ++ (factors) ++ [1, number]

primeFactors :: (Integral a) => a -> [Integer]
primeFactors n =  [x | x <- (primesUntil), x <= (fromIntegral n), ((fromIntegral n) `rem` x) == 0]

primesUntil :: [Integer]
primesUntil = primesTo 10000 --99999999


triangularNumber :: (Integral a) => a -> a
triangularNumber n = (n^2 + n) `quot` 2

factorTriangularNumber :: (Integral a) => a -> ([Integer], a)
factorTriangularNumber n 
					| even (triangularNumber n) = ((primeFactors n) ++ (primeFactors (n + 1)), triangularNumber' n) -- n+1 and n are coprimes!
					| otherwise = (filter (\x -> x /= 2) $ (primeFactors n) ++ (primeFactors (n + 1)), triangularNumber' n) -- remove the 2!
						where 
							triangularNumber' n = triangularNumber n