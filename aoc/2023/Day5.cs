using aoc.Lib;

namespace aoc._2023;

public class Day5(int day, int year, bool isTest) : Solution(day, year, isTest)
{
    public override object SolvePart1()
    {
        var almanac = ParseInput();
        return almanac.FindLocations().Min();
    }

    public override object SolvePart2()
    {
        var almanac = ParseInput();

        var task = almanac.GetLowestDestination();
        var result = Task.Run<long>(async () => await almanac.GetLowestDestination());

        return result.Result;
    }

    Almanac ParseInput()
    {
        var seeds = Input[0].Split(":")[1].Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(long.Parse).ToList();
        var alamac = new Almanac(seeds);

        var mappings = new List<List<Mapping>>();
        var mapping = new List<Mapping>();

        for (var i = 0; i < Input.Count; i++)
        {
            var line = Input[i];
            if (line.Contains("seeds")) continue;
            if (line == string.Empty)
            {
                if (mapping.Any())
                {
                    mappings.Add(mapping);
                    mapping = [];
                }
                continue;
            }
            if (!char.IsDigit(line[0])) continue;
            var ranges = line.Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(long.Parse).ToList();
            var map = new Mapping(ranges[0], ranges[1], ranges[2]);
            mapping.Add(map);

            if (i == Input.Count - 1)
                mappings.Add(mapping);
        }

        alamac.SeedToSoil = mappings[0];
        alamac.SoilToFert = mappings[1];
        alamac.FertToWater = mappings[2];
        alamac.WaterToLight = mappings[3];
        alamac.LightToTemp = mappings[4];
        alamac.TempToHumid = mappings[5];
        alamac.HumidToLoc = mappings[6];

        return alamac;
    }

    private class Almanac
    {
        public List<long> Seeds { get; set; }

        public List<Mapping> SeedToSoil { get; set; } = [];
        public List<Mapping> SoilToFert { get; set; } = [];
        public List<Mapping> FertToWater { get; set; } = [];
        public List<Mapping> WaterToLight { get; set; } = [];
        public List<Mapping> LightToTemp { get; set; } = [];
        public List<Mapping> TempToHumid { get; set; } = [];
        public List<Mapping> HumidToLoc { get; set; } = [];

        public Almanac(List<long> initialSeeds)
        {
            Seeds = initialSeeds;
        }

        public List<long> FindLocations()
        {
            var locations = new List<long>();

            foreach (var seed in Seeds)
            {
                var soilNumber = FindLocation(SeedToSoil, seed);
                var fetilizer = FindLocation(SoilToFert, soilNumber);
                var water = FindLocation(FertToWater, fetilizer);
                var light = FindLocation(WaterToLight, water);
                var temperature = FindLocation(LightToTemp, light);
                var humidity = FindLocation(TempToHumid, temperature);
                var location = FindLocation(HumidToLoc, humidity);
                locations.Add(location);
            }

            return locations;
        }

        public async Task<long> GetLowestDestination()
        {
            var lowest = long.MaxValue;
            var tasks = new List<Task>();

            for (int i = 0; i < Seeds.Count; i += 2)
            {
                var seedIndex = i;
                var range = Seeds[i + 1];
                tasks.Add(Task.Run(() =>
                {
                    for (int j = 0; j < range; j++)
                    {
                        var seed = Seeds[seedIndex] + j;
                        var location = FindLowestLocation(seed);
                        lowest = Math.Min(location, lowest);
                    }
                }));
            }

            await Task.WhenAll(tasks);

            return lowest;
        }

        public long FindLowestLocation(long seedNr)
        {
            var soilNumber = FindLocation(SeedToSoil, seedNr);
            var fetilizer = FindLocation(SoilToFert, soilNumber);
            var water = FindLocation(FertToWater, fetilizer);
            var light = FindLocation(WaterToLight, water);
            var temperature = FindLocation(LightToTemp, light);
            var humidity = FindLocation(TempToHumid, temperature);

            return FindLocation(HumidToLoc, humidity);
        }

        static long FindLocation(List<Mapping> mappings, long seedNr)
        {
            var location = seedNr;
            foreach (var map in mappings)
            {
                var maxRange = map.Source + map.Range - 1;

                if (seedNr >= map.Source && seedNr <= maxRange)
                {
                    var diff = maxRange - seedNr;
                    location = map.Dest + map.Range - 1 - diff;
                }
            }

            return location;
        }
    }

    record Mapping(long Dest, long Source, long Range);
}
