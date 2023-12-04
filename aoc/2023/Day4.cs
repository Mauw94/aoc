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
        var scratchCards = new Dictionary<int, int>();

        foreach (var card in cards)
            scratchCards.Add(card.CardId, 1);

        foreach (var card in cards)
        {
            var winningNumbers = card.CountWinningNumbers();
            for (var i = 0; i < winningNumbers; i++)
            {
                var nextKey = card.CardId + 1 + i;
                var copiesOfCard = scratchCards[card.CardId];
                if (copiesOfCard > 1)
                {
                    for (var j = 0; j < copiesOfCard; j++)
                        scratchCards[nextKey]++;
                }
                else
                {
                    scratchCards[nextKey]++;
                }
            }
        }

        var total = scratchCards.Sum(x => x.Value);
        return total;
    }

    List<Card> ParseInput()
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

    class Card(int cardId, List<int> set1, List<int> set2)
    {
        public int CardId { get; set; } = cardId;
        public List<int> Set1 { get; set; } = set1;
        public List<int> Set2 { get; set; } = set2;

        public int CountWinningNumbers()
        {
            var count = 0;
            foreach (var nr in Set2)
                if (Set1.Contains(nr))
                    count++;

            return count;
        }

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