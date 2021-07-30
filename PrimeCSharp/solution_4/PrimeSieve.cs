using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Runtime.Intrinsics;
using System.Runtime.Intrinsics.X86;
using System.Text;
using System.Threading.Tasks;

namespace Solution4
{
    public class PrimeSieve
    {
        const int _divide = 5; // 2^5 == 32 
        const int _wordBits = sizeof(uint) * 8;

        readonly int _sieveSize;
        readonly int _numBits;
        readonly uint[] _words;

        public PrimeSieve(int size)
        {
            _sieveSize = size;
            _numBits = (size + 1) / 2;

            var numWords = _numBits / _wordBits + 1;
            _words = new uint[numWords];
            // TODO: IntPtr ptr = Marshal.AllocHGlobal(..); .. Marshal.FreeHGlobal(hglobal)
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        bool GetBit(int index) => (_words[index >> _divide] & (1u << index)) == 0;

        [MethodImpl(MethodImplOptions.NoInlining)]
        public void RunSieve()
        {
            var q = (int)Math.Sqrt(_sieveSize);

            var maskOnes = Vector256.Create(1u);
            var maskMod32 = Vector256.Create(31u);
            var factorMult = Vector256.Create(0u, 1u, 2u, 3u, 4u, 5u, 6u, 7u);

            uint factor = 3;
            while (true)
            {
                // find next factor - next still-flagged number
                var index = (int)factor >> 1;
                while (index < _numBits)
                {
                    if (GetBit(index))
                        break;

                    ++index;
                }
                factor = (uint)index * 2 + 1;

                // check for termination _before_ resetting flags;
                // note: need to check up to and including q, otherwise we
                // fail to catch cases like sieve_size = 1000
                if (factor > q) 
                {
                    break;
                }

                // set bits using unsafe pointer and unrolled loop
                unsafe
                {
                    fixed (uint* ptr = _words)
                    {
                        var i0 = (uint)(factor * factor) >> 1;
                        var indexVector = Avx2.Add(
                            Vector256.Create(i0),
                            Vector256.Create(
                                0,
                                factor,
                                factor * 2,
                                factor * 3,
                                factor * 4,
                                factor * 5,
                                factor * 6,
                                factor * 7
                                )
                            );

                        // compare
                        //var i0 = (factor * factor) >> 1;
                        //var i1 = i0 + factor;
                        //var i2 = i0 + factor * 2;
                        //var i3 = i0 + factor * 3;

                        // safety: we've ensured that (i3 >> _divide) < length
                        var factor8 = (uint)(factor * 8);
                        while (indexVector.GetElement(7) < _numBits)
                        {
                            var indexWord = Avx2.ShiftRightLogical(indexVector, _divide);
                            var ivMod32 = Avx2.And(indexVector, maskMod32);
                            var mask = Avx2.ShiftLeftLogicalVariable(maskOnes, ivMod32);

                            // shifts in C# are already wrapping (low 5 bits)
                            //ptr[i0 >> _divide] |= 1u << i0;
                            //ptr[i1 >> _divide] |= 1u << i1;
                            //ptr[i2 >> _divide] |= 1u << i2;
                            //ptr[i3 >> _divide] |= 1u << i3;
                            ptr[indexWord.GetElement(0)] |= mask.GetElement(0);
                            ptr[indexWord.GetElement(1)] |= mask.GetElement(1);
                            ptr[indexWord.GetElement(2)] |= mask.GetElement(2);
                            ptr[indexWord.GetElement(3)] |= mask.GetElement(3);
                            ptr[indexWord.GetElement(4)] |= mask.GetElement(4);
                            ptr[indexWord.GetElement(5)] |= mask.GetElement(5);
                            ptr[indexWord.GetElement(6)] |= mask.GetElement(6);
                            ptr[indexWord.GetElement(7)] |= mask.GetElement(7);

                            indexVector = Avx2.Add(indexVector, Vector256.Create(factor8));
                            //iv = Sse2.Add(iv, Vector128.Create(factor4));
                            //i0 += factor4;
                            //i1 += factor4;
                            //i2 += factor4;
                            //i3 += factor4;
                        }

                        // safety: we've ensured that (i0 >> _divide) < length
                        var i = (int)indexVector.GetElement(0);
                        while (i < _numBits)
                        {
                            // shifts in C# are already wrapping (low 5 bits)
                            ptr[i >> _divide] |= 1u << i;
                            i += (int)factor;
                        }
                    }
                }

                // advance factor
                factor += 2;
            }
        }

        public int CountPrimes()
        {
            int count = 0;
            for (int index = 1; index <= _sieveSize / 2; index++)
            {
                if (GetBit(index))
                    count++;
            }
            return count;
        }

        public bool IsValid => KnownPrimes.IsValid(_sieveSize, CountPrimes());

        // TODO: implement IDisposable.
    }
}
