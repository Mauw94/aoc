using aoc.Lib;
List<Solution> solutions2015 =
[
    new aoc._2015.Day1(1, 2015, false),
    new aoc._2015.Day2(2, 2015, false)
];

List<Solution> solutions2022 =
[
    new aoc._2022.Day1(1, 2022, false)
];

List<Solution> solutions2023 =
[
    new aoc._2023.Day1(1, 2023, false),
    new aoc._2023.Day2(2, 2023, false),
    new aoc._2023.Day3(3, 2023, false)
];

// Run(solutions2015);
// Run(solutions2022);
Run(solutions2023);
return;

static void Run(List<Solution> solutions)
{
    Console.WriteLine("\nWelcome to Advent of Code!\n\n");

    foreach (var solution in solutions)
    {
        Console.WriteLine("Year{0} ", solution.Year);
        Console.WriteLine("Day{0} ", solution.Day);

        Common.StartStopwatch();
        Console.WriteLine("\tSolution part1: {0}", solution.SolvePart1());
        Common.StopStopwatch();
        Console.Write($"\tExecuted in {Common.TimeElapsed()} ms");

        Console.WriteLine("\n");

        Common.StartStopwatch();
        Console.WriteLine("\tSolution part2: {0}", solution.SolvePart2());
        Common.StopStopwatch();
        Console.Write($"\tExecuted in {Common.TimeElapsed()} ms");

        Console.WriteLine("\n");
    }
}