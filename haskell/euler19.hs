{-# LANGUAGE ExplicitForAll #-}
-- https://en.wikipedia.org/wiki/Zeller%27s_congruence
-- NOTE: In this algorithm January and February are counted as months 13 and 14 of the previous year.
-- E.g. if it is February 2, 2010, the algorithm counts the date as the second day of the fourteenth month of 2009
-- (02/14/2009 in DD/MM/YYYY format)
import Control.Applicative ((<$>))

data Month = January | February | March | April | May | June | July | August | September | October | November | December deriving (Show, Ord, Eq, Enum)
data Day = Saturday | Sunday | Monday | Tuesday | Wednesday | Thursday |  Friday deriving (Show, Eq, Enum)

main :: IO ()
main = print euler19

euler19 :: Int
euler19 = length $ filter (== Sunday) $ uncurry (dayOfWeek 1) <$> list
	where
		list = [(month, year)| month <- [January .. December], year <- [1901..2000]]

adjustDay :: forall t t1. (Num t, Num t1, Ord t) => t -> t1 -> (t, t1)
adjustDay month year 
			| month <= 2 = (month + 12, year -1)
			| otherwise = (month, year)

dayOfWeek :: forall a a1. (Enum a, Enum a1) => Int -> a1 -> Int -> a
dayOfWeek day month year = toEnum $ fromIntegral $ uncurry (zellersCongruence day) $ adjustDay (fromEnum month + 1) year

zellersCongruence :: forall a. Integral a => a -> a -> a -> a
zellersCongruence day month year = (day + (13*(month+1) `quot `5) + yearOfTheCentury + yearOfTheCentury `quot` 4 + century `quot` 4 + 5*century) `rem` 7
	where
		yearOfTheCentury = year `rem` 100
		century = year `quot` 100


zipMap :: forall a b. (b -> a) -> [b] -> [(a,b)]
zipMap f xs = zip (f <$> xs) xs