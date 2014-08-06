import Helper (digits)

main :: IO ()
main = print $ euler16

euler16 = sum  $ digits $ 2^1000 
