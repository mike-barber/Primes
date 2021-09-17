-- Template haskell Module for Sieve of Eratosthenes algorithm...
--   implements extree loop unrolling and dense culling...
--   Note:  `case` is often used to force strictness rather than as a branch!

{-# OPTIONS_GHC -O2 -fobject-code -fllvm #-}
{-
{-# OPTIONS_GHC -optlo --O3 -optlo --disable-lsr -optlc -O3 -optlc --disable-lsr #-}
--}
{-# LANGUAGE TemplateHaskell, MagicHash, UnboxedTuples #-} -- , FlexibleContexts

module PrimesTH ( makeLoopFuncArr, cDENSETHRESHOLD, makeDenseFuncArr ) where

import Language.Haskell.TH ( litE, intPrimL, wordPrimL, ExpQ )

import Data.Bits ( Bits( shiftR, shiftL, (.&.), (.|.) ) )
import GHC.Exts ( Int(I#), Addr#, Int#, State#(..) )

cNUMUNROLLEDCASES :: Int
cNUMUNROLLEDCASES = 32

cDENSETHRESHOLD :: Int
cDENSETHRESHOLD = 129

-- following code is for extreme unrolling function array builder macro...

cullBit :: Int -> Int -> ExpQ -- cull bit modadically by primitives
cullBit bi n = -- cull (set one) bit index `bi` for case selection `n`...
  let rq = case bi of 0 -> [| 0# |] -- name the register `r`, 0th none...
                      1 -> [| r1# |]
                      2 -> [| r2# |]
                      3 -> [| r3# |]
                      4 -> [| r4# |]
                      5 -> [| r5# |]
                      6 -> [| r6# |]
                      7 -> [| r7# |]
      bp8 = ((n `shiftR` 2) .|. 1) .&. 7 -- extract the step value
      msk = (n + bi * bp8) .&. 7 -- extract the mask index
      mq = litE $ wordPrimL $ 1 `shiftL` msk in
  [|  \s0# -> case readWord8OffAddr# ai# $rq s0# of -- read byte value
      (# s1#, v# #) ->
        let nv# = v# `or#` $mq in -- modify value
        case writeWord8OffAddr# ai# $rq nv# s1# of
        s2# -> s2# |] -- after writing new value

closeCulls :: Int -> ExpQ -- cull remaining < one loop span monadically...
closeCulls n = -- customized by case number `n`...
  let loop i = -- a nested monadic conditional loop...
        if i >= 7 then [| \ s# -> (# s#, () #) |] else -- maximum 7 culls!
        let rq = case i of 0 -> [| 0# |] -- name the register `r`, 0th none...
                           1 -> [| r1# |]
                           2 -> [| r2# |]
                           3 -> [| r3# |]
                           4 -> [| r4# |]
                           5 -> [| r5# |]
                           6 -> [| r6# |]
                           7 -> [| r7# |]
            rlq = [| rlmt# |] in -- `rlmt#` is definied in outer calling macro
        [| \ s# -> case $rq <# $rlq of -- tests extreme limit
              0# -> (# s#, () #) -- finished
              _ -> case $(cullBit i n) s# of -- otherwise loop
                so# -> $(loop (i + 1)) so# |] in loop 0

unrollCase :: Int -> ExpQ -- unroll the eight culling operations monadically...
unrollCase n =
  let loop i = if i > 7 then [| \s# -> (# s#, () #) |] else
               [| \ s# -> case $(cullBit i n) s# of -- nested monadic operations
                    so# -> $(loop (i + 1)) so# |] in loop 0

makeUnrolledFunc :: Int -> ExpQ -- make a function taking buffer address/size
makeUnrolledFunc n = -- with starting bit index and step size...
  [| \ ba# sz# bi0# stpi# -> -- first initialize all the register offsets...
    let r0# = bi0# `uncheckedIShiftRL#` 3#
        r1# = ((bi0# +# stpi#) `uncheckedIShiftRL#` 3#) -# r0#
        r2# = ((bi0# +# (stpi# *# 2#)) `uncheckedIShiftRL#` 3#) -# r0#
        r3# = ((bi0# +# (stpi# *# 3#)) `uncheckedIShiftRL#` 3#) -# r0#
        r4# = ((bi0# +# (stpi# *# 4#)) `uncheckedIShiftRL#` 3#) -# r0#
        r5# = ((bi0# +# (stpi# *# 5#)) `uncheckedIShiftRL#` 3#) -# r0#
        r6# = ((bi0# +# (stpi# *# 6#)) `uncheckedIShiftRL#` 3#) -# r0#
        r7# = ((bi0# +# (stpi# *# 7#)) `uncheckedIShiftRL#` 3#) -# r0#
        a0# = ba# `plusAddr#` r0# -- first byte address
        lmt# = ba# `plusAddr#` sz#
        almt# = lmt# `plusAddr#` (0# -# r7#)
        loopa# ai# si# = -- looping monadically up to limit
          case ai# `ltAddr#` almt# of
          0# ->
            let rlmt# = lmt# `minusAddr#` ai#
            in $(closeCulls n) si# -- limit reached, temination code
          _ ->
            case $(unrollCase n) si# of
            (# so#, _ #) -> loopa# (ai# `plusAddr#` stpi#) so#
    in ST $ \ s# -> loopa# a0# s# |] -- do it monadically from first address

makeUnrolledFuncList :: () -> ExpQ
makeUnrolledFuncList() =
  let loop i = if i >= cNUMUNROLLEDCASES then [| [] |] else
               [| $(makeUnrolledFunc i) : $(loop (i + 1)) |] in loop 0

makeLoopFuncArr :: () -> ExpQ
makeLoopFuncArr() =
  [| listArray (0 :: Int, cNUMUNROLLEDCASES - 1) $(makeUnrolledFuncList()) |]


-- following code is for dense culling function array builder macro...

unrollDensePattern :: Integer -> ExpQ
unrollDensePattern n =
  let bp = n + n + 3
      bi0 = (bp * bp - 3) `shiftR` 1
      lmt = bp * 64
      loop bi wi =
        if bi >= lmt then [| \ s# -> (# s#, () #) |] else
        let nbi = bi + bp
            nwi = bi `shiftR` 6              
            mskq = litE $ wordPrimL $ 1 `shiftL` (fromIntegral bi .&. 63) in
        if nwi > wi && (nbi `shiftR` 6) - nwi > 0 then
          let wiq = litE $ intPrimL nwi in
          [| \ s# -> case readWord64OffAddr# ai# $wiq s# of
                (# s1#, v# #) -> case v# `or#` $mskq of
                  nv# -> case writeWord64OffAddr# ai# $wiq nv# s1# of
                    so# -> $(loop nbi nwi) so# |] else
        if nwi > wi then
          let wiq = litE $ intPrimL nwi in
          [| \ s# -> case readWord64OffAddr# ai# $wiq s# of
                (# s1#, v# #) -> case v# `or#` $mskq of
                    nv# -> $(loop nbi nwi) (# s1#, nv# #) |] else
        if nbi `shiftR` 6 > wi then
          let wiq = litE $ intPrimL wi in
          [| \ (# s#, v# #) -> case v# `or#` $mskq of
                nv# -> case writeWord64OffAddr# ai# $wiq nv# s# of
                  so# -> $(loop nbi wi) so# |]
        else [| \ (# s#, v# #) -> case v# `or#` $mskq of
                                    nv# -> $(loop nbi wi) (# s#, nv# #) |]
  in loop 0 (-1) -- pre-conditioning guarantees first bit index is zero!

makeUDenseFunc :: Int -> ExpQ -- make a function taking buffer address
makeUDenseFunc n = -- and byte size with starting bit index and step size...
  [| \ ba# sz# bi0# stpi# -> -- first initialize all the register offsets...
    let a0# = ba# `plusAddr#` ((bi0# `uncheckedIShiftRL#` 3#) `andI#` (-8#))
        advi# = stpi# *# 8# -- advance by step size words at a time
        almt# = ba# `plusAddr#` (sz# -# (advi# -# 8#))
        loopa# :: Addr# -> State# s -> (# State# s, Int #)
        loopa# ai# si# = -- looping monadically up to limit
          case ai# `ltAddr#` almt# of -- if limit reached, return next index...
            0# -> case (ai# `minusAddr#` ba#) *# 8# of -- six LSB's always 0's!
                    newndx# -> (# si#, I# newndx# #)
            _ -> case $(unrollDensePattern $ fromIntegral n) si# of
                   (# so#, _ #) ->
                     loopa# (ai# `plusAddr#` advi#) si#
    in ST $ \ s# -> case loopa# a0# s# of { rslt# -> rslt# } |] -- monadically!

makeDenseFuncList :: () -> ExpQ
makeDenseFuncList() =
  let loop i = if i > (cDENSETHRESHOLD - 3) `shiftR` 1 then [| [] |] else
               [| $(makeUDenseFunc i) : $(loop (i + 1)) |]
  in loop 0

makeDenseFuncArr :: () -> ExpQ
makeDenseFuncArr() =
  [| listArray (0 :: Int, (cDENSETHRESHOLD - 3) `shiftR` 1)
               $(makeDenseFuncList()) |]
