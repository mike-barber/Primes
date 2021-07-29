using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace Solution4
{
    public class PrimeSieve : IDisposable
    {
        const int _divide = 5; // 2^5 == 32 
        const int _wordBits = sizeof(uint) * 8;

        readonly int _sieveSize;
        readonly int _numBits;
        readonly int _numWords;
        readonly IntPtr _words;

        [MethodImpl(MethodImplOptions.NoInlining)]
        public PrimeSieve(int size)
        {
            _sieveSize = size;
            _numBits = (size + 1) / 2;
            _numWords = _numBits / _wordBits + 1;

            // allocate unmanaged heap memory, and zero it
            _words = Marshal.AllocHGlobal(_numWords * sizeof(uint));
            unsafe
            {
                var span = new Span<byte>(_words.ToPointer(), _numWords);
                Unsafe.InitBlock(ref MemoryMarshal.GetReference(span), 0, (uint)_numWords * sizeof(uint));
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        static bool GetBit(Span<uint> words, int index) => (words[index >> _divide] & (1u << index)) == 0;

        [MethodImpl(MethodImplOptions.NoInlining)]
        public void RunSieve()
        {
            var q = (int)Math.Sqrt(_sieveSize);
            var factor = 3;

            unsafe
            {
                uint* ptr = (uint*)_words.ToPointer();
                var span = new Span<uint>(_words.ToPointer(), _numWords);

                while (true)
                {
                    // find next factor - next still-flagged number
                    var index = factor / 2;
                    while (index < _numBits)
                    {
                        if (GetBit(span, index))
                            break;

                        ++index;
                    }
                    factor = index * 2 + 1;

                    // check completion before resetting bits
                    if (factor > q)
                        break;

                    // set bits using unsafe pointer and unrolled loop
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

                    factor += 2;
                }
            }
        }

        public int CountPrimes()
        {
            unsafe
            {
                var span = new Span<uint>(_words.ToPointer(), _numWords);
                int count = 0;
                for (int index = 1; index <= _sieveSize / 2; index++)
                {
                    if (GetBit(span, index))
                        count++;
                }
                return count;
            }
        }

        public bool IsValid => KnownPrimes.IsValid(_sieveSize, CountPrimes());


        [MethodImpl(MethodImplOptions.NoInlining)]
        public void Dispose()
        {
            Marshal.FreeHGlobal(_words);
        }
    }
}
