import Helper (digits)

main :: IO ()
main = print $ euler20

euler20 :: (Integral a) => a
euler20 = sum $ digits $ fac 100

fac :: (Integral a) => a -> a
fac 0 = 1
fac n = n * fac (n - 1)