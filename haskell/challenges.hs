module Challenges (elem', oddSquareUnder10k, chain', sum', mapWithFoldr, sqrtSum, numOfUniques, firstTimeOver1k, countChar) where

import Common
import Data.List (group, nub, sort)
import Distribution.Compat.CharParsing (CharParsing (string))

--
-- Find the largest number under 100,000 that's divisible by 3829.
--
largestDivisibleBy3829 :: (Integral a) => a
largestDivisibleBy3829 = head (Common.filter' f [100000, 99999 ..])
  where
    f x = x `mod` 3829 == 0

--
-- Sum all odd squares less than 10000
--
oddSquareUnder10k :: (Integral a) => a
oddSquareUnder10k = sum (takeWhile (< 10000) (Common.filter' odd (Common.map' (^ 2) [1 ..])))

--
-- Collatz Sequences
-- We take a natural number. If that number is even, we divide it by two. If it's odd, we multiply it by 3 and then add 1 to that. We take the resulting number and apply the same thing to it, which produces a new number and so on. In essence, we get a chain of numbers. It is thought that for all starting numbers, the chains finish at the number 1. So if we take the starting number 13, we get this sequence: 13, 40, 20, 10, 5, 16, 8, 4, 2, 1. 13*3 + 1 equals 40. 40 divided by 2 is 20, etc. We see that the chain has 10 terms.
--
--
chain' :: (Integral a) => a -> [a]
chain' 1 = [1]
chain' x
  | even x = x : chain' (x `div` 2)
  | odd x = x : chain' (x * 3 + 1)

--
-- For all starting numbers between 1 and 100, how many chains have a length greater than 15?
--
numLongChains :: Int
numLongChains = length $ Common.filter' (> 15) [length (chain' x) | x <- [1 .. 100]]

--
-- Alternative implementation
-- numLongChains = length $ Common.filter' (\x -> length x > 15) (Common.map' chain' [1 .. 100])
--
--
--
sum' :: (Num a) => [a] -> a
-- sum' xs = foldl (\acc x -> acc + x) 0 xs
-- sum' xs = foldl (+) 0 xs
sum' = foldl (+) 0

--
--
elem' :: (Eq a) => a -> [a] -> Bool
-- elem' y ys = foldl (\acc x -> if x == y then True else acc) False ys
elem' y ys = foldl (\acc x -> (x == y) || acc) False ys

--
--
--
mapWithFoldr :: (a -> b) -> [a] -> [b]
mapWithFoldr f xs = foldr (\x acc -> f x : acc) [] xs

--
--
-- How many elements does it take for the sum of the roots of all natural numbers to exceed 1000?
sqrtSum :: Int
sqrtSum = length (takeWhile (< 1000) $ scanl1 (\acc x -> acc + sqrt x) [1 ..]) + 1

-- Usinf function composition to solve previous problem
--
oddSquareSum :: Integer
oddSquareSum = sum . takeWhile (< 10000) . filter odd . map (^ 2) $ [1 ..]

--
-- return number of unique numbers in a list
--
numOfUniques :: (Eq a) => [a] -> Int
numOfUniques = length . nub

--
--
-- We're given a list that represents the value of a stock by date. The list is made of tuples whose first component is the stock value, the second is the year, the third is the month and the fourth is the date. We want to know when the stock value first exceeded one thousand dollars!
--
firstTimeOver1k :: [(Float, Int, Int, Int)] -> (Float, Int, Int, Int)
firstTimeOver1k xs = head $ dropWhile (\(stock, _, _, _) -> stock < 1000) xs

--
--
-- Count how many time each chareacter appears in a string
countChar :: (Ord a) => [a] -> [(a, Int)]
countChar [] = []
countChar (x : xs) = map (\lst@(y : ys) -> (y, length lst)) . group . sort $ xs

--
--
--
