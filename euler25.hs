import Helper (fib)

main :: IO ()
main = print euler25

euler25 = (length $ takeWhile (\x -> (x <= 10^(1000-1))) (map fib [1..])) + 1

