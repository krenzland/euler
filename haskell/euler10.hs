import Helper (primesTo)

main :: IO ()
main = print euler10

euler10 :: Integer
euler10 = sum $ primesTo 2000000
