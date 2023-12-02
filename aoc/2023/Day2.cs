using System.Text.RegularExpressions;
using aoc.Lib;

namespace aoc._2023;

public class Day2(int day, int year, bool isTest) : Solution(day, year, isTest)
{
    public override object SolvePart1()
    {
        var games = new List<Game>();

        foreach (var line in Input)
        {
            var gameId = ParseInts(line, @"Game (\d+)").First();
            var red = ParseInts(line, @"(\d+) red").Max();
            var green = ParseInts(line, @"(\d+) green").Max();
            var blue = ParseInts(line, @"(\d+) blue").Max();
            games.Add(new Game(gameId, red, green, blue));
        }

        return games
            .Where(x => x is { Red: <= 12, Green: <= 13, Blue: <= 14 })
            .Select(x => x.Id)
            .Sum();
    }

    public override object SolvePart2()
    {
        var games = new List<Game>();

        foreach (var line in Input)
        {
            var gameId = ParseInts(line, @"Game (\d+)").First();
            var red = ParseInts(line, @"(\d+) red").Max();
            var green = ParseInts(line, @"(\d+) green").Max();
            var blue = ParseInts(line, @"(\d+) blue").Max();
            games.Add(new Game(gameId, red, green, blue));
        }

        return games
            .Select(x => x.Red * x.Green * x.Blue)
            .Sum();
    }

    private static IEnumerable<int> ParseInts(string line, string rx) =>
        Regex.Matches(line, rx).Select(x => int.Parse(x.Groups[1].Value));

    private record Game(int Id, int Red, int Green, int Blue);
}