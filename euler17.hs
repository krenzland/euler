main :: IO ()
main = print $ euler17

euler17 = toString 0

toString n
	|  n < 10  = case n of
		1 -> "one"
		2 -> "two"
		3 -> "three"
		4 -> "four"
		5 -> "five"
		6 -> "six"
		7 -> "seven"
		8 -> "eigth"
		9 -> "nine"
		_ -> "THE FUCK?"
	| n == 10 = "ten"	
	| otherwise	= "What the fucking fuck?"

