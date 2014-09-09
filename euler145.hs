import Helper (digits)

main :: IO ()
main = print euler145

euler145 :: Int
euler145 = length $ filter isReversable [1..(10^9)-1]

isReversable :: Int -> Bool
isReversable n = not (n `rem` 10 == 0) &&
				(null $ filter even $ digits sum)
	where
		sum = n + reverseN n
		reverseN x = (numberFromDigits . reverse . digits) x

numberFromDigits :: [Int] -> Int
numberFromDigits list = numberFromDigits' list 0
	where
		numberFromDigits' [] _ = 0
		numberFromDigits' (x:xs) acc = x * (10^acc) + numberFromDigits' xs (acc+1) 

