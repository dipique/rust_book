using System;
using System.Diagnostics;

namespace cs_3np1
{
    class Program
    {
        static void Main(string[] args)
        {
            const uint MAX = 10_000_000;
            uint ctr = 1;
            bool log = false;

            var sw = new Stopwatch();
            sw.Start();
            ulong top = 1;
            while (++ctr <= MAX)
            {
                ulong val = ctr;
                ulong iters = 1;
                while ((val = tnpo(val)) != 4)
                    iters++;
                if (iters > top)
                {
                    top = iters;
                    Console.WriteLine($"\r{ctr}: {iters} iterations");
                } else if (log)
                    Console.WriteLine($"{ctr}: {iters} iterations");
            }
            sw.Stop();
            var tms = sw.ElapsedMilliseconds;
            var ms = tms % 1000;
            var s = (tms - ms) / 1000;
            Console.WriteLine($"Time elapsed: {s}_{ms}");
        }

        static ulong tnpo(ulong n) => n % 2 == 0 ? n / 2 : n * 3 + 1;
    }
}
