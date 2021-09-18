-- implements the Techniques that benefit from LLVM `--disable-slr`...
--   Note:  disable Loop Strength Reduction, which mis-optimized in this case.

-- {-# OPTIONS_GHC -O2 -fllvm #-}
-- {-# OPTIONS_GHC -optlo --O3 -optlo --disable-lsr -optlc -O3 -optlc --disable-lsr #-}
{-# LANGUAGE TemplateHaskell, MagicHash, UnboxedTuples, FlexibleContexts #-}

module PrimesNoLSR ( Technique(..), primesSoENoLSR ) where

import PrimesTH ( makeLoopFuncArr, cDENSETHRESHOLD, makeDenseFuncArr )

import Data.Word ( Word8, Word64 )
import Data.Bits ( Bits((.|.), (.&.), shiftL, shiftR) )
import Data.Array ( Array )
import Data.Array.Base ( STUArray(STUArray),
                         castSTUArray, unsafeRead, unsafeWrite,
                         UArray(..), listArray, unsafeAt )
import Data.Array.ST ( runSTUArray )
import Control.Monad ( forM_ )
import GHC.ST ( ST(ST), runST )
import GHC.Base
    ( newPinnedByteArray#, byteArrayContents#, unsafeFreezeByteArray#,
      Addr#, Int#, Int(I#), (+#), (-#), (*#), (<#), or#, orI#, andI#,
      uncheckedIShiftRL#, plusAddr#, minusAddr#, ltAddr#,
      setByteArray#, readWord8OffAddr#, writeWord8OffAddr#,
      readWord64OffAddr#, writeWord64OffAddr# )

type Prime = Word64
type SieveBuffer = UArray Int Bool

data Technique = BitTwiddle | Stride8 | Stride8Block
               | Extreme | ExtremeHybrid deriving (Enum)

cBITMASK :: UArray Int Word8 -- faster than bit shifting...
cBITMASK = listArray (0, 7) [ 1, 2, 4, 8, 16, 32, 64, 128 ]

-- used for Extreme and ExtremeHybrid Techniqeus...
-- arguments are address of SieveBuffer, size of SieveBuffer in bytes,
-- start bit index in SieveBuffer, and the step span per marking to 1;
-- with all parameters as raw primitive# values...
cUNROLLFNCARR :: Array Int (Addr# -> Int# -> Int# -> Int# -> ST s ())
cUNROLLFNCARR = $(makeLoopFuncArr())

-- {-
-- used for ExtremeHybrid Techniqeu...
-- arguments are address of SieveBuffer, size of SieveBuffer in bytes,
-- start bit index in SieveBuffer, and the step span per marking to 1;
-- with all parameters as raw primitive# values...
cDENSEFNCARR :: Array Int (Addr# -> Int# -> Int# -> Int# -> ST s Int)
cDENSEFNCARR = $(makeDenseFuncArr())
--}

primesSoENoLSR :: Prime -> Technique -> SieveBuffer
primesSoENoLSR limit tec = runSTUArray $ do -- in ST Monad for mutation...
  let bitlmt = fromIntegral ((limit - 3) `div` 2) -- round up to 64-bit word...
      bytesz@(I# cmpstssz#) = ((bitlmt + 64) `shiftR` 3) .&. (-8)
  cs@(STUArray l h n pmba#) <- -- make pinned Word8 array
    ST $ \s0# ->
      case newPinnedByteArray# cmpstssz# s0# of
        (# s1#, marr# #) ->
          case setByteArray# marr# 0# cmpstssz# 0# s1# of
            s2# -> (# s2#, STUArray 0 bitlmt (bitlmt + 1) marr# #)
  (UArray _ _ _ fpba#) <- ST $ \s0# ->
    case unsafeFreezeByteArray# pmba# s0# of
      (# s1#, pba# #) -> (# s1#, UArray l h n pba# #)
  let cmpstsa# = byteArrayContents# fpba# -- this and cmpstsasz# for Extremes!
  csb <- (castSTUArray :: STUArray s Int Word8 -> -- boolean view of array
                            ST s (STUArray s Int Bool)) cs
  let lastByteIndex = bitlmt `shiftR` 3
      sqrtlmtndx = floor (sqrt (fromIntegral limit) - 3) `div` 2
  forM_ [ 0 .. sqrtlmtndx ] $ \ ndx -> do -- outer loop finding base primes
    b <- unsafeRead csb ndx
    if b then return () else do -- all cases must be covered; found one!...
      let basePrime = ndx + ndx + 3
          startIndex = (basePrime * basePrime - 3) `shiftR` 1

      case tec of -- select culling technique as per argument...

        BitTwiddle ->
          forM_ [startIndex, startIndex+basePrime..bitlmt] $ \ bitIndex -> do
            let byteIndex = bitIndex `shiftR` 3
            v <- unsafeRead cs byteIndex
            let nv = v .|. unsafeAt cBITMASK (bitIndex .&. 7)
            unsafeWrite cs byteIndex nv

        Extreme ->
          let bp@(I# bp#) = basePrime
              (I# bi0#) = startIndex
              n = ((bp .&. 6) `shiftL` 2) + (startIndex .&. 7)
              f = unsafeAt cUNROLLFNCARR n
          in f cmpstsa# cmpstssz# bi0# bp#

        ExtremeHybrid ->
          let bp@(I# bp#) = basePrime in
          if bp > cDENSETHRESHOLD then
            let (I# bi0#) = startIndex
                n = ((bp .&. 6) `shiftL` 2) + (startIndex .&. 7)
                f = unsafeAt cUNROLLFNCARR n
            in f cmpstsa# cmpstssz# bi0# bp#
          else do -- dense technique...
            let loop0 bitIndex = -- cull to 64-bit boundary, avg 15.5 times...
                  if (bitIndex .&. 63) <= 0 then return bitIndex else do
                  let byteIndex = bitIndex `shiftR` 3
                  v <- unsafeRead cs byteIndex
                  let nv = v .|. unsafeAt cBITMASK (bitIndex .&. 7)
                  unsafeWrite cs byteIndex nv; loop0 (bitIndex + basePrime)
                f = unsafeAt cDENSEFNCARR $ (bp - 3) `shiftR` 1
            (I# bi0#) <- loop0 startIndex -- always zero!
            rndx <- f cmpstsa# cmpstssz# bi0# bp#
            -- cull remaining that didn't fit in dense loop...
            forM_ [rndx, rndx + basePrime .. bitlmt] $ \ bitIndex -> do
              let byteIndex = bitIndex `shiftR` 3
              v <- unsafeRead cs byteIndex
              let nv = v .|. unsafeAt cBITMASK (bitIndex .&. 7)
              unsafeWrite cs byteIndex nv

        _ -> error "Unimplemented no-LSR Technique!!!"

  lstv <- unsafeRead cs lastByteIndex -- mask primes above bitlmt
  unsafeWrite cs lastByteIndex (lstv .|. (0xFE `shiftL` (bitlmt .&. 7)))
  return csb -- actual deliverable is boolean for convenience in decoding
