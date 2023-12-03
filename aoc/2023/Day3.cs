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
        throw new NotImplementedException();
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