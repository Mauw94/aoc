namespace aoc.Lib;

public record Coord(int X, int Y, string Symbol);

public class Grid2D
{
    (int x, int y) Dimensions { get; set; }
    List<Coord> Coords { get; set; }

    readonly List<string> _input;

    public Grid2D(List<string> input, Coord coord)
    {
        Dimensions = (input.Count, input[0].Length);
        Coords = [];

        _input = input;

        ParseGrid();
    }

    void ParseGrid()
    {
        for (var x = 0; x < Dimensions.x; x++)
            for (var y = 0; y < Dimensions.y; y++)
            {
                Coords.Add(new Coord(X: x, Y: y, Symbol: _input[y][x].ToString()));
            }
    }
}