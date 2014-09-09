import qualified Data.Vector.Unboxed as V
import Data.Char (digitToInt)

main :: IO ()
main = print euler40

euler40 :: Int
euler40 = product $ fmap d  [1,10,100,1000,10000,100000,1000000]
  where
    d n = digitToInt $ digits V.! (n-1) :: Int --vectors are zero based 

digits :: V.Vector Char
digits =  V.fromList dList --vector for O(1) lookup
  where 
    dList = take 1000000 $ concat $ fmap show [1..]