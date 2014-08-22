{-# LANGUAGE ExplicitForAll #-}

import           Control.Applicative ((<$>))
import           Data.List           (sort)
import           Data.Maybe          (fromMaybe)
import qualified Data.Vector         as V

main :: IO ()
main = print euler21

euler21 :: Int
euler21 = sum amicableNumbers

amicableNumbers :: [Int]
amicableNumbers = [x | x <- numbers, isAmicableNumber x]
        where
            isAmicableNumber n = doubleLookup n /= [] && check n  && checkself n
            check y = or (elem y <$> doubleLookup y)
            checkself y = y `notElem` fromMaybe [] (numberVector `vectorLookup` y)

doubleLookup :: Int -> [[Int]]
doubleLookup n = (fromMaybe [] . (numberVector `vectorLookup`)) <$> fromMaybe [] (numberVector `vectorLookup` n)

vectorLookup :: forall a. V.Vector (Maybe a) -> Int -> Maybe a
vectorLookup v i
                | i > V.length v = Nothing
                | otherwise = v V.! i

numberVector :: V.Vector (Maybe [Int])
numberVector = V.fromList $ maybefy $ constructList divisorList 0

constructList :: forall b. (Eq b, Num b) => [(b, b)] -> b -> [[b]]
constructList [] _ = []
constructList xs acc = listyfy  (takeWhile (\x -> fst x == acc) xs) : constructList (dropWhile (\x -> fst x == acc) xs) (acc + 1)

maybefy :: forall a. [[a]] -> [Maybe [a]]
maybefy = map maybefy'
        where
            maybefy' [] = Nothing
            maybefy' xs = Just xs

listyfy :: forall a. [(a, a)] -> [a]
listyfy = map snd

divisorList :: [(Int, Int)]
divisorList =  sort $ zipMap (sum . divisors) numbers

numbers :: [Int]
numbers = [1..9999]

divisors :: Int -> [Int]
divisors n = [x | x <- [1..(n-1)], n `rem` x == 0]

zipMap :: forall a b. (b -> a) -> [b] -> [(a,b)]
zipMap f xs = zip (f <$> xs) xs
