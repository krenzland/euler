import qualified Data.Text as T

main :: IO ()
main = euler99

euler99 :: IO()
euler99 = readFile file >>= print . getLineNumber . reduce . parse  
  where
    file = "p099_base_exp.txt"
    getLineNumber n = snd . maximum $ zip n [1..]

reduce :: [[Integer]] -> [Float]
reduce = fmap (\x -> (ilog (x !! 0)) * (fromIntegral (x !! 1)))
  where
    ilog n = log $ fromIntegral n 

parse n = fmap (fmap read) $ fmap (split) $ lines n :: [[Integer]] 

split = (fmap (T.unpack)) . T.split (==',') . T.pack 