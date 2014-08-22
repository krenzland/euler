import           Control.Applicative ((<$>))
 
main :: IO ()
main = print $ euler12
 
euler12 :: Integer
euler12 = head $ dropWhile (\x -> length (divisors x) < 500) $ triangularNumber <$> [1..] 
 
divisors :: Integer -> [Integer]
divisors n = [x | x <- [1..n], n `rem` x == 0]
 
triangularNumber :: Integer -> Integer
triangularNumber n = (n^2 + n) `quot` 2