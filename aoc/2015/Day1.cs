using aoc.Lib;

namespace aoc._2015;

public class Day1(int day, int year, bool isTest) : Solution(day, year, isTest)
{
    public override object SolvePart1()
    {
        return TraverseFloors();
    }

    public override object SolvePart2()
    {
        return ReachingBasement();
    }

    private int TraverseFloors()
    {
        var floor = 0;
        var steps = Input.First().ToCharArray();

        foreach (var step in steps)
        {
            if (step == '(')
                floor++;
            if (step == ')')
                floor--;
        }

        return floor;
    }

    private int ReachingBasement()
    {
        var floor = 0;
        var index = 0;
        var steps = Input.First().ToCharArray();

        for (var i = 0; i < steps.Length; i++)
        {
            var step = steps[i];
            if (step == '(')
                floor++;
            if (step == ')')
                floor--;

            if (floor == -1)
            {
                index = i + 1;
                break;
            }
        }

        return index;
    }
}