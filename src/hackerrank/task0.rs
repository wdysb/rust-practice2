using System;
using System.Linq;

class Solution
{
    public static int SimpleArraySum(int[] ar)
    {
        return ar.Sum();
    }

    static void Main(string[] args)
    {
        string inputN = Console.ReadLine();
        if (string.IsNullOrEmpty(inputN)) return;

        int[] ar = Console.ReadLine()
            .Split(new[] { ' ' }, StringSplitOptions.RemoveEmptyEntries)
            .Select(int.Parse)
            .ToArray();

        Console.WriteLine(SimpleArraySum(ar));
    }
}
