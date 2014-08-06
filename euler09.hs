main :: IO ()
main = print euler9

euler9 :: (Integral a) => a
euler9 = product $ listify tupel
			where	
				listify (a,b,c) = [a,b,c]
				tupel = head [(a,b,c) | a <- [1..1000], b <-[2..1000], c <- [3..1000], a<b, b<c, a^2 + b^2 == c^2 ,a + b + c == 1000]

