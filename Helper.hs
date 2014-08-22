module Helper
(fib
 ,primes
, primeFactors
, digits
, primesTo
, approximationNthPrime
, nubQ
) where

-- http://hackage.haskell.org/package/matrix-0.3.3.0/docs/Data-Matrix.html
import           Data.Array.Unboxed
import qualified Data.Matrix        as M
import           Data.Maybe         (fromJust)
import qualified Data.Set           as S
import Control.Applicative ((<$>))


fib :: (Num a) => Int -> a
fib n = M.getElem 1 2 $ powerMatrix n
	where
		powerMatrix n = foldl1 M.multStd2 $ replicate n matrix
		matrix = M.fromList 2 2 [1,1,1,0]

primes :: (Integral a) => [a]
primes = sieve [2..]
			where
				sieve (p:xs) = p : sieve [x | x <- xs, (x `rem` p) /= 0]

primeFactors :: (Integral a) => a -> [Integer]
primeFactors n =  [x | x <- (primesUntil $ fromIntegral n), ((fromIntegral n) `rem` x) == 0]
			where
				primesUntil n = takeWhile(\x -> (x<= n)) $ primesTo n

digits :: Integral x => x -> [x]
digits 0 = []
digits x = x `rem` 10 : digits (x `div` 10)

primesTo :: Integer -> [Integer]
primesTo m = sieve 3 (array (3,m) [(i,odd i) | i<-[3..m]]
                        :: UArray Integer Bool)
			  where
				   sieve p a
				     | p*p > m   = 2 : [i | (i,True) <- assocs a]
				     | a!p       = sieve (p+2) $ a//[(i,False) | i <- [p*p, p*p+2*p..m]]
				     | otherwise = sieve (p+2) a

approximationNthPrime :: (Integral a1, Num a) => a1 -> a
approximationNthPrime n
			|  n <= 6
			= 13
			|  otherwise =
			fromIntegral $ ceiling $ (fromIntegral n) * (log $ fromIntegral n)  + (fromIntegral n) * (log $ log $ fromIntegral n)

nubQ :: (Ord a) => [a] -> [a]
nubQ = S.toList . S.fromList

--isLeapYear year = year `dividesBy` 4 && (not (year `dividesBy` 100) || year `dividesBy` 400)
--	where
--		a `dividesBy` b = a `rem` b == 0

--daysInMonth February year = if isLeapYear year
--							then 29
--							else 28
--daysInMonth month year = fromJust $ month `lookup` daysInMonth'
--	where
--			daysInMonth' = zip [January .. December] [31,28,31,30,31,30,31,31,30,31,30,31]
