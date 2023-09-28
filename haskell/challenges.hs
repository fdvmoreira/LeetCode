module Challenges (largestDivisibleBy3829, oddSquareUnder10k) where

import Common qualified

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
