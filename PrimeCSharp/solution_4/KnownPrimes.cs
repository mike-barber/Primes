using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Solution4
{
    public static class KnownPrimes
    {
        static readonly Dictionary<int, int> _known = new()
        {
            { 10, 4 },                // Historical data for validating our results - the number of primes
            { 100, 25 },               // to be found under some limit, such as 168 primes under 1000
            { 1_000, 168 },
            { 10_000, 1229 },
            { 100_000, 9592 },
            { 1_000_000, 78498 },
            { 10_000_000, 664579 },
            { 100_000_000, 5761455 }
        };

        public static bool IsValid(int sieveSize, int primeCount) =>
            _known.ContainsKey(sieveSize) && _known[sieveSize] == primeCount;

        public static IEnumerable<int> KnownSizes => _known.Keys;
    }
}
