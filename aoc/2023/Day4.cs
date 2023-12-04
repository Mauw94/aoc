using aoc.Lib;

namespace aoc._2023;

public class Day4(int day, int year, bool isTest) : Solution(day, year, isTest)
{
    public override object SolvePart1()
    {
        var cards = ParseInput();

        foreach (var card in cards)
            card.ProductWinningNumbers();

        return cards.Select(x => x.ProductWinningNumbers()).Sum();
    }

    public override object SolvePart2()
    {
        var cards = ParseInput();
        var scratchCards = cards.ToDictionary(card => card.CardId, card => 1);

        foreach (var card in cards)
        {
            var winningNumbers = card.CountWinningNumbers();
            for (var i = 0; i < winningNumbers; i++)
            {
                var nextKey = card.CardId + 1 + i;
                var copiesOfCard = scratchCards[card.CardId];
                scratchCards[nextKey] += copiesOfCard > 1 ? 1 * copiesOfCard : 1;
            }
        }

        return scratchCards.Sum(x => x.Value);
    }

    private List<Card> ParseInput()
    {
        var cards = new List<Card>();
        var i = 0;
        foreach (var line in Input)
        {
            i++;
            var split = line.Split(":");
            var sets = split[1].Split("|");
            var set1 = sets[0].Split(" ").Where(x => x != string.Empty).Select(int.Parse).ToList();
            var set2 = sets[1].Split(" ").Where(x => x != string.Empty).Select(int.Parse).ToList();
            cards.Add(new Card(i, set1, set2));
        }

        return cards;
    }

    private class Card(int cardId, List<int> set1, List<int> set2)
    {
        public int CardId { get; set; } = cardId;
        private List<int> Set1 { get; set; } = set1;
        private List<int> Set2 { get; set; } = set2;

        public int CountWinningNumbers() => Set2.Count(nr => Set1.Contains(nr));

        public int ProductWinningNumbers()
        {
            var product = 1;
            var winningNumbers = CountWinningNumbers();

            if (winningNumbers == 0)
                return 0;

            for (var i = 1; i < winningNumbers; i++)
                product *= 2;

            return product;
        }
    }
}