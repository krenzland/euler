module PathSum
(findPath,
Triangle
) where

type Row = [Int]
type Triangle = [[Int]]

findPath :: Triangle -> Int
findPath list =  head . head $ until (\x -> length x == 1) reduceFirst list
  where
    reduceFirst (x:xs:xxs) = sumPair (x, xs) : xxs 
    sumPair (big, small) = zipWith (+) (reduceRow big) small

reduceRow :: Row -> Row
reduceRow xs  
  | length xs < 2  = []
  | otherwise   = (maximum . take 2) xs : (reduceRow . (drop 1)) xs