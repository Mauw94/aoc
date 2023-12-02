using aoc.Lib;

namespace aoc._2015;

public class Day2(int day, int year, bool isTest) : Solution(day, year, isTest)
{
    public override object SolvePart1()
    {
        var dimensions = new List<Dimension>();
        foreach (var line in Input)
        {
            var dims = line.Split('x').Select(int.Parse).ToList();
            dimensions.Add(new Dimension(dims[0], dims[1], dims[2]));
        }

        return dimensions.Select(x => x.WrappingPaperNeeded()).Sum();
    }

    public override object SolvePart2()
    {
        var dimensions = new List<Dimension>();
        foreach (var line in Input)
        {
            var dims = line.Split('x').Select(int.Parse).ToList();
            dimensions.Add(new Dimension(dims[0], dims[1], dims[2]));
        }

        return dimensions.Select(x => x.TotalRibbonNeeded()).Sum();
    }

    class Dimension(int Lenght, int Width, int Height)
    {
        int FindSmallestSide()
        {
            var sides = new List<int>() {
                Lenght * Width,
                Width * Height,
                Height * Lenght
            };

            return sides.Min();
        }

        int CalculateSquareFeet() => 2 * Lenght * Width + 2 * Width * Height + 2 * Height * Lenght;
        int RibbonWrap()
        {
            var dims = new List<int>() { Lenght, Width, Height };
            var smallest = dims.OrderBy(x => x).Take(2).ToList();

            return smallest[0] * 2 + smallest[1] * 2;
        }

        int RibbonBow() => Lenght * Width * Height;

        public long TotalRibbonNeeded() => RibbonWrap() + RibbonBow();
        public long WrappingPaperNeeded() => CalculateSquareFeet() + FindSmallestSide();
    }
}