﻿using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Text;
using System.Threading.Tasks;

namespace Solution4
{
    public class PrimeSieve
    {
        const int _divide = 5; // 2^5 == 32 

        readonly int _sieveSize;
        readonly int _numBits;
        readonly uint[] _words;

        public PrimeSieve(int size)
        {
            _sieveSize = size;
            _numBits = (size + 1) / 2;

            var numWords = _numBits / sizeof(uint) + 1;
            _words = new uint[numWords];
            // TODO: IntPtr ptr = Marshal.AllocHGlobal(..); .. Marshal.FreeHGlobal(hglobal)
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        bool GetBit(int index) => (_words[index >> _divide] & (1u << index)) == 0;

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void RunSieve()
        {
            var q = (int)Math.Sqrt(_sieveSize);

            var factor = 3;
            while (true)
            {
                // find next factor - next still-flagged number
                var index = factor >> 1;
                while (index < _numBits)
                {
                    if (GetBit(index))
                        break;

                    ++index;
                }
                factor = index * 2 + 1;

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
                        var i0 = (factor * factor) >> 1;
                        var i1 = i0 + factor;
                        var i2 = i0 + factor * 2;
                        var i3 = i0 + factor * 3;

                        // safety: we've ensured that (i3 >> _divide) < length
                        var factor4 = factor * 4;
                        while (i3 < _numBits)
                        {
                            // shifts in C# are already wrapping (low 5 bits)
                            ptr[i0 >> _divide] |= 1u << i0;
                            ptr[i1 >> _divide] |= 1u << i1;
                            ptr[i2 >> _divide] |= 1u << i2;
                            ptr[i3 >> _divide] |= 1u << i3;

                            i0 += factor4;
                            i1 += factor4;
                            i2 += factor4;
                            i3 += factor4;
                        }

                        // safety: we've ensured that (i0 >> _divide) < length
                        while (i0 < _numBits)
                        {
                            // shifts in C# are already wrapping (low 5 bits)
                            ptr[i0 >> _divide] |= 1u << i0;
                            i0 += factor;
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
