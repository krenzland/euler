main :: IO ()
main = print euler5

euler5 :: (Integral a) => a
euler5 = foldl1 lcm [1..20]
