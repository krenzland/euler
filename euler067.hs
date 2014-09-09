import PathSum

main :: IO ()
main = euler67

euler67 :: IO ()
euler67 = readFile trianglePath >>= print . findPath . reverse . parseTriangle 
  where
    trianglePath = "p067_triangle.txt"

parseTriangle :: String -> Triangle
parseTriangle x = fmap ((rowToInt) . words) $ lines x
  where
    rowToInt = fmap read


