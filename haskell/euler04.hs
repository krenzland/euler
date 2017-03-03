main :: IO ()
main = print euler4

euler4 = maximum $ filter isPalindrome products
			where 
				products =  [x*y | x <- [100..999], y <- [100..999]]  
				isPalindrome x = show x == (reverse . show) x 
