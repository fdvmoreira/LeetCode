module Challenges (largestDivisibleBy3829) where

import Common qualified

--
-- Find the largest number under 100,000 that's divisible by 3829.
--
largestDivisibleBy3829 :: (Integral a) => a
largestDivisibleBy3829 = head (Common.filter' f [100000, 99999 ..])
  where
    f x = x `mod` 3829 == 0
