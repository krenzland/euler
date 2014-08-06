module Helper
( fib
, primes
, largestPrimeFactor
, digits	
) where 


fib :: (Integral a) => a -> a
fib 1 = 1
fib 2 = 1
fib n = fib  (n-1) + fib (n-2)

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

