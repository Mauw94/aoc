using System.Diagnostics;

namespace aoc.Lib;

public class Common
{
    private static Stopwatch _stopWatch;

    public static IEnumerable<string> GetInput(int day, int year, bool isTest)
    {
        var fileName = isTest ? $"day{day}-test.txt" : $"day{day}.txt";
        var inputPath = $@"D:\projects\aoc\aoc\{year}\Input\";
        var file = inputPath + fileName;

        if (File.Exists(file))
            return File.ReadAllLines(file);

        throw new Exception($"Couldn't find file for year: {year} and  day: {day} \tPath: {inputPath}");
    }

    public static void StartStopwatch() => _stopWatch = Stopwatch.StartNew();
    public static void StopStopwatch() => _stopWatch.Stop();
    public static long TimeElapsed() => _stopWatch.ElapsedMilliseconds;
}