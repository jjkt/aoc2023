using System;
using System.Text;


class Program
{
    static string[] readFile(string fileName)
    {
        List<string> lines = new List<string>();

        const Int32 BufferSize = 128;
        using (var fileStream = File.OpenRead(fileName))
        using (var streamReader = new StreamReader(fileStream, Encoding.UTF8, true, BufferSize))
        {
            String line;
            while ((line = streamReader.ReadLine()) != null)
            {
                lines.Add(line);
            }
        }

        return lines.ToArray();
    }


    static int Main(string[] args)
    {
        if (args.Length == 0)
        {
            Console.WriteLine("please give a filename as parameter");

            return 1;
        }
        else
        {
            string fileName = args[0];
            string[] lines = readFile(fileName);


            int points_sum = 0;
            int card_count = 0;
            List<int> cards = new List<int>(card_count);
            for (int i = 0; i < lines.Count(); i++)
            {
                cards.Add(1);
            }

            int card_index = 0;
            foreach (var line in lines)
            {
                // parsing line
                var parts = line.Split('|');
                var left_side_parts = parts[0].Split(':');

                var winning_numbers_str = left_side_parts[1].Trim().Split(' ', StringSplitOptions.RemoveEmptyEntries);
                var my_numbers_str = parts[1].Trim().Split(' ', StringSplitOptions.RemoveEmptyEntries);

                int[] winning_numbers = Array.ConvertAll(winning_numbers_str, int.Parse);
                int[] my_numbers = Array.ConvertAll(my_numbers_str, int.Parse);

                IEnumerable<int> both = winning_numbers.Intersect(my_numbers);

                // calculate the number of winning numbers:
                int count = both.Count();

                int points = 0;
                if (count > 0)
                {
                    points = 1 << (count - 1);
                    int add = cards[card_index];
                    for (int i = 0; i < count; i++)
                    {
                        if (card_index + 1 + i < cards.Count)
                        {
                            cards[card_index + 1 + i] += add;
                        }
                    }
                }

                card_count += cards[card_index];

                points_sum += points;
                card_index++;
            }
            Console.WriteLine("total = " + points_sum + " points");
            Console.WriteLine("total cards= " + card_count + "");
            return 0;
        }
    }
}