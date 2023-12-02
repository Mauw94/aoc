using System.Text.RegularExpressions;
using aoc.Lib;

namespace aoc._2023;

public class Day1(int day, int year, bool isTest) : Solution(day, year, isTest)
{
    public override object SolvePart1()
    {
        var result = new List<int>();

        const string rx = @"\d";

        foreach (var line in Input)
        {
            var first = ParseMatch(Regex.Match(line, rx).Value) * 10;
            var last = ParseMatch(Regex.Match(line, rx, RegexOptions.RightToLeft).Value);

            result.Add(first + last);
        }

        return result.Sum();
    }

    public override object SolvePart2()
    {
        var result = new List<int>();
        const string rx = @"\d|one|two|three|four|five|six|seven|eight|nine";

        foreach (var line in Input)
        {
            var first = ParseMatch(Regex.Match(line, rx).Value) * 10;
            var last = ParseMatch(Regex.Match(line, rx, RegexOptions.RightToLeft).Value);

            result.Add(first + last);
        }

        return result.Sum();
    }

    private static int ParseMatch(string value) => value switch
    {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => int.Parse(value)
    };
}