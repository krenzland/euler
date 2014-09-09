import qualified Data.Text as T

main :: IO ()
main = euler99

euler99 = readFile file >>= print . getLineNumber . reduce . parse  
  where
    file = "p099_base_exp.txt"
    getLineNumber n = snd . maximum $ zip n [1..]

reduce :: [[Integer]] -> [Integer]
reduce = fmap (\x -> (x !! 0) ^ (x !! 1)) -- or n^(log e)
  
-- GOD, THIS SHIT IS SO FUCKING UGLY! THIS CAN'T BE! OH NO!

parse n = fmap (fmap read) $ fmap (split) $ lines n :: [[Integer]] 

split = (fmap (T.unpack)) . T.split (==',') . T.pack 