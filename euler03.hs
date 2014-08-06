import Helper (largestPrimeFactor)

main :: IO ()
main = print euler3

euler3 :: (Integral a) => a
euler3 = largestPrimeFactor 600851475143 
			
