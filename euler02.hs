import Helper (fib)

main :: IO ()
main = print euler2

euler2 :: (Integral a) => a
euler2 = sum $ filter even $ takeWhile (\x -> (x<4000000)) $ map fib [1..]