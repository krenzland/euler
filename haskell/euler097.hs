import Helper (digits)
import Data.List (foldl')

main :: IO ()
main = print euler97

--euler97 :: [Integer]
euler97 = reverse $ take 10 $ digits number
	where
		number = (28433 * (2^7830457)) + 1 :: Integer