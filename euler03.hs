import Helper (primeFactors)

main :: IO ()
main = print euler3

euler3 :: Integer
euler3 = last $ primeFactors 600851475143 
			
