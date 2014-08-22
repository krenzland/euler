main :: IO ()
main = print $ collatzSequence 3 [0]

euler14 = undefined

collatzSequence :: (Num a) => [a] -> a -> [a]
collatzSequence xs n
				| even n = xs : (n `quot` 2)
				| otherwise = xs : (3 * n + 1)
