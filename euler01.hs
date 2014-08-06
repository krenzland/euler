main :: IO ()
main = print $ euler1

euler1 :: (Integral a) => a
euler1 = sum numbers
		where 
			numbers = filter divBy5Or3 [1..999]
			divBy5Or3 n = (n `rem` 3 == 0) || (n `rem` 5 == 0)
