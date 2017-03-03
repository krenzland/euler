main :: IO ()
main = print euler6

euler6 :: (Integral a) => a
euler6 = squareOfSums - sumOfSquares
			where
				sumOfSquares = sum [x * x | x <- [1..100]]
				squareOfSums = sum [1..100] ^ 2
