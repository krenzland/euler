import Helper (primes)

main :: IO ()
main = print euler10

euler10 :: (Integral a) => a -- FUCKING SLOW
euler10 = sum $ takeWhile (\x -> x < 2000000) primes
