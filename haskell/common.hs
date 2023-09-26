module Common
  ( map',
    filter',
    quicksort',
  )
where

import Distribution.Simple.Utils (xargs)

map' :: (a -> b) -> [a] -> [b]
map' _ [] = []
map' fn (x : xs) = fn x : map' fn xs

-- usage: map' (**2) [43,2,34,55]
-- output [1849.0,4.0,1156.0,3025.0]

filter' :: (a -> Bool) -> [a] -> [a]
filter' _ [] = []
filter' f (x : xs)
  | f x = x : filter' f xs
  | otherwise = filter' f xs

-- usage:
-- filter' (>10) [5..15]
-- output> [11,12,13,14,15]

quicksort' :: (Ord a) => [a] -> [a]
quicksort' [] = []
quicksort' (x : xs) =
  let smallerSorted = quicksort' (filter' (<= x) xs)
      biggerSorted = quicksort' (filter' (> x) xs)
   in smallerSorted ++ [x] ++ biggerSorted

-- usage
-- quicksort' [100,90..0]
-- output: [0,10,20,30,40,50,60,70,80,90,100]
--
--
