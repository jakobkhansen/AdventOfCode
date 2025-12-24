string? lstore;
List<string> lines = new List<string>();

// Read lines from stdin until Console.ReadLine() returns null (EOF)
while ((lstore = Console.ReadLine()) != null)
{
    lines.Add(lstore);
}

string[] ranges = lines[0].Split(",");
long sum = 0;

foreach (string range in ranges)
{
    string[] nums = range.Split("-");
    long first = long.Parse(nums[0]);
    long second = long.Parse(nums[1]);

    for (long i = first; i <= second; i++)
    {
        if (isInvalid(i))
        {
            Console.WriteLine($"invalid: {i}");
            sum += i;
        }
    }
}
Console.WriteLine(sum);

static bool isInvalid(long n)
{
    string num = n.ToString();
    for (int i = 1; i < ((num.Length / 2) + 1); i++)
    {
        if (num.Length % i != 0)
        {
            continue;
        }

        string[] substrings = Enumerable
            .Range(0, num.Length / i)
            .Select(j => num.Substring(j * i, i))
            .ToArray();
        if (substrings.All(sub => sub == substrings[0]))
        {
            return true;
        }
    }
    return false;
}
