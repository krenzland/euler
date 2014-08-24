import           Control.Applicative ((<$>))
import           Data.Map            (Map)
import qualified Data.Map            as M
import           Data.Maybe          (fromJust)
import Control.Monad.RWS
import qualified Data.Vector.Mutable as VM
import qualified Data.Vector as V

main :: IO ()
main = print example

euler14 :: Int
euler14 = snd . maximum . zipMap length $  getList 999999
	where
		zipMap f xs = zip (f <$> xs) (head <$> xs)

getList :: Int -> [[Int]]
getList x = getList' x M.empty
	where
		getList' 0 _  = []
		getList' n m  = fst (res n m) : getList' (n-1) (snd $ res n m)
		res = collatz

type Cmap = Map Int [Int]

collatz :: Int -> Cmap -> ([Int], Cmap)
collatz 1 m = ([1], m)
collatz n m = if n `M.member` m
				then (fromJust $ n `M.lookup` m, m)
				else (list, mm)
			where
				mm = M.insert n list $ snd res
				list = n : fst res
				res = collatz (nextCollatz n) m

nextCollatz :: Int -> Int
nextCollatz n
			| even n = n `quot` 2
			| otherwise = 3 * n + 1


type R = Int
type W = [Int]
type S = Map Int Int

computation :: RWS R W S ()
computation = do
  value <- ask
  stat <- get
  let b = value + value
  tell [b]
  tell [b]


example = execRWS computation reader_ (state_)
	where
		reader_ = 10
		state__ = VM.new 10
		state_ = M.empty
