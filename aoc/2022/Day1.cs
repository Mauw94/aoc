using aoc.Lib;

namespace aoc._2022;

public class Day1 : Solution
{
    public Day1(int day, int year, bool isTest) : base(day, year, isTest)
    {
    }

    public override object SolvePart1()
    {
        return CalculateTotalCalories().Max();
    }

    public override object SolvePart2()
    {
        return CalculateTotalCalories().OrderByDescending(x => x).Take(3).Sum();
    }

    private List<int> CalculateTotalCalories()
    {
        var totals = new List<int>();
        var temp = 0;

        for (var i = 0; i < Input.Count; i++)
        {
            var line = Input[i];

            if (line == "")
            {
                totals.Add(temp);
                temp = 0;
            }
            else
            {
                temp += int.Parse(line);
            }

            if (i == Input.Count - 1)
                totals.Add(temp);
        }

        return totals;
    }
}