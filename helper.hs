module Helper
( fib
, primes
, largestPrimeFactor
, digits	
) where 

-- http://hackage.haskell.org/package/matrix-0.3.3.0/docs/Data-Matrix.html
import qualified Data.Matrix as M

fib :: (Num a) => Int -> a
fib n = M.getElem 1 2 $ powerMatrix n
	where
		powerMatrix n = foldl1 M.multStd2 $ replicate n matrix
		matrix = M.fromList 2 2 [1,1,1,0]

primes :: (Integral a) => [a]
primes = sieve [2..]
			where
				sieve (p:xs) = p : sieve [x | x <- xs, (x `rem` p) /= 0]

largestPrimeFactor :: (Integral a) => a -> a
largestPrimeFactor n = maximum $ map (gcd n) (primesUntil $ ceiling $ sqrt $ fromIntegral n)
			where
				primesUntil n = takeWhile(\x -> (x<= n)) primes				

digits :: Integral x => x -> [x]
digits 0 = []
digits x = x `rem` 10 : digits (x `div` 10)

