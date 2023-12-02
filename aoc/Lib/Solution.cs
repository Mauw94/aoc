namespace aoc.Lib;

public abstract class Solution
{
    public int Day { get; set; }
    public int Year { get; set; }
    protected List<string> Input { get; set; }

    public Solution(int day, int year, bool isTest)
    {
        Day = day;
        Year = year;
        Input = Common.GetInput(day, year, isTest).ToList();
    }

    public abstract object SolvePart1();
    public abstract object SolvePart2();
}