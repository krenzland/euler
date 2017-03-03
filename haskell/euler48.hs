main :: IO()
main = print euler48 

euler48 :: String
euler48 = reverse $ take 10 $ reverse $ show $ sum $ map selfpower [1..1000]
		where
			selfpower n = n ^ n 