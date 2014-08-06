import Helper (primes)

main :: IO ()
main = print euler7

euler7 :: (Integral a) => a
euler7 = primes !! (10001-1)
