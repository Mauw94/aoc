using System.Runtime.CompilerServices;
using aoc.Lib;

namespace aoc._2023;

public class Day3(int day, int year, bool isTest) : Solution(day, year, isTest)
{
    public override object SolvePart1()
    {
        ParseSchematic();
        var numberParts = FindAllNumberParts();

        return numberParts.Sum();
    }

    public override object SolvePart2()
    {
        ParseSchematic();
        var gearRatios = FindAllGearRatios();

        return gearRatios.Sum();
    }

    void ParseSchematic()
    {
        Dimensions = (Input.Count, Input[0].Length);

        for (var x = 0; x < Dimensions.x; x++)
            for (var y = 0; y < Dimensions.y; y++)
            {
                Schematic.Add(new EnginePart(X: x, Y: y, C: Input[y][x]));
            }
    }

    List<long> FindAllGearRatios()
    {
        var gearRatios = new List<long>();
        for (var x = 0; x < Dimensions.x; x++)
            for (var y = 0; y < Dimensions.y; y++)
            {
                var symbol = GetSymbolAtPos(x, y);
                if (symbol != '*') continue;

                var numbers = FindAdjacentNumberParts(x, y);
                if (numbers.Count >= 2)
                    gearRatios.Add(numbers[0] * numbers[1]);
            }

        return gearRatios;
    }

    List<int> FindAdjacentNumberParts(int x, int y)
    {
        var adjacents = GetAdjacent(x, y);
        var numberPartCoords = new List<(int x, int y)>();
        var numbers = new List<int>();
        var seen = new List<(int x, int y)>();

        foreach (var adjacent in adjacents)
        {
            if (SymbolIsNumberPart(adjacent.C))
            {
                if (!numberPartCoords.Contains((adjacent.X, adjacent.Y)))
                    numberPartCoords.Add((adjacent.X, adjacent.Y));
            }
        }

        if (numberPartCoords.Count < 2) return numbers;

        foreach (var numberPart in numberPartCoords)
        {
            if (seen.Contains((numberPart.x, numberPart.y))) continue;

            seen.Add((numberPart.x, numberPart.y));

            var numberParts = new List<string>
            {
                GetSymbolAtPos(numberPart.x, numberPart.y).ToString()
            };

            var xP = numberPart.x;
            var yP = numberPart.y;

            TraverseLeft(--xP, yP, numberParts, seen);

            xP = numberPart.x + 1;
            yP = numberPart.y;

            TraverseRight(xP, yP, numberParts, seen);

            var realNumber = int.Parse(string.Join("", numberParts));
            numbers.Add(realNumber);
        }

        return numbers;
    }

    List<string> TraverseLeft(int x, int y, List<string> numberParts, List<(int, int)> seen)
    {
        seen.Add((x, y));
        var symbol = GetSymbolAtPos(x, y);
        if (!SymbolIsNumberPart(symbol)) return numberParts;

        numberParts.Insert(0, symbol.ToString());

        if (x - 1 >= 0)
        {
            return TraverseLeft(--x, y, numberParts, seen);
        }

        return numberParts;
    }

    List<string> TraverseRight(int x, int y, List<string> numberParts, List<(int, int)> seen)
    {
        seen.Add((x, y));
        var symbol = GetSymbolAtPos(x, y);
        if (!SymbolIsNumberPart(symbol)) return numberParts;

        numberParts.Add(symbol.ToString());

        if (x + 1 < Dimensions.x)
        {
            return TraverseRight(++x, y, numberParts, seen);
        }
        return numberParts;
    }

    List<int> FindAllNumberParts()
    {
        var numbers = new List<int>();

        for (var x = 0; x < Dimensions.x; x++)
            for (var y = 0; y < Dimensions.y; y++)
            {
                if (IsInSeen(x, y)) continue;
                if (!SymbolIsNumberPart(x, y)) continue;

                var numberParts = FindWholeNumber(x, y);

                if (AnyAdjacentIsASymbol(numberParts))
                {
                    var number = new List<string>();
                    foreach (var part in numberParts)
                    {
                        number.Add(part.Item1.ToString());
                    }
                    var realNumber = int.Parse(string.Join("", number));
                    numbers.Add(realNumber);
                }
            }

        return numbers;
    }

    bool AnyAdjacentIsASymbol(List<(char c, (int x, int y) coord)> numberParts)
    {
        var allAdjacents = new List<EnginePart>();
        foreach (var (c, coord) in numberParts)
        {
            var adjacents = GetAdjacent(coord.x, coord.y);
            allAdjacents.AddRange(adjacents);
        }

        return allAdjacents.Any(x => Symbols.Contains(x.C));
    }

    char GetSymbolAtPos(int x, int y) => Schematic.Where(s => s.X == x && s.Y == y).First().C;

    bool SymbolIsNumberPart(int x, int y)
        => char.IsDigit(Schematic.Where(s => s.X == x && s.Y == y).First().C);

    static bool SymbolIsNumberPart(char c) => char.IsDigit(c);

    List<(char, (int, int))> FindWholeNumber(int x, int y)
    {
        var numberParts = new List<(char, (int, int))>
        {
            (GetSymbolAtPos(x, y), (x, y))
        };
        AddCoordsToSeen(x, y);
        return FindNextNumberPart(x, y, numberParts);
    }

    List<(char, (int, int))> FindNextNumberPart(int x, int y, List<(char, (int, int))> numberParts)
    {
        var nextPos = ++x;
        if (nextPos < Dimensions.x)
        {
            var symbol = GetSymbolAtPos(nextPos, y);
            if (SymbolIsNumberPart(symbol))
            {
                numberParts.Add((symbol, (nextPos, y)));
                AddCoordsToSeen(nextPos, y);
                return FindNextNumberPart(nextPos, y, numberParts);
            }
        }

        return numberParts;
    }

    void AddCoordsToSeen(int x, int y) => Seen.Add((x, y));

    bool IsInSeen(int x, int y) => Seen.Contains((x, y));

    List<EnginePart> GetAdjacent(int x, int y)
    {
        List<(int x, int y)> coords =
        [
            new (x + 1, y),
            new (x - 1, y),
            new (x, y + 1),
            new (x, y - 1),
            new (x + 1, y + 1),
            new (x + 1, y - 1),
            new (x - 1, y + 1),
            new (x - 1, y - 1),
        ];

        var neighbours = coords
            .Where(p => p.x >= 0 && p.x < Dimensions.x
                && p.y >= 0 && p.y <= Dimensions.y)
            .Select(p => p)
            .ToList();

        return Schematic
            .Where(x => neighbours.Contains((x.X, x.Y)))
            .Select(x => x)
            .ToList();
    }

    readonly List<char> Symbols = ['+', '*', '$', '#', '/', '%', '=', '@', '&', '-'];
    List<(int x, int y)> Seen = [];
    readonly List<EnginePart> Schematic = [];
    (int x, int y) Dimensions;
    record EnginePart(int X, int Y, char C);
}