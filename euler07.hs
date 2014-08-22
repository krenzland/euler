import Helper (primesTo, approximationNthPrime)

main :: IO ()
main = print $ euler7

euler7 :: Integer
euler7 =  nthPrime 10001
		where
			nthPrime n = (primesTo $ approximationNthPrime n) !! (n - 1)
		



	