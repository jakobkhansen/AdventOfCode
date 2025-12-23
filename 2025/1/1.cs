string? lstore;
List<string> lines = new List<string>();

// Read lines from stdin until Console.ReadLine() returns null (EOF)
while ((lstore = Console.ReadLine()) != null)
{
    lines.Add(lstore);
}

int curr = 50;
int output = 0;

foreach (string l in lines)
{
    Console.WriteLine(l);
    int num = int.Parse(l[1..]);
    if (l[0] == 'L')
    {
        num *= -1;
    }
    Console.WriteLine(num);
    int prev = curr;
    int rotated = curr + num;
    Console.WriteLine($"curr before {curr}");
    curr = (((curr + num) % 100) + 100) % 100;

    output += Math.Abs(rotated) / 100;
    output += prev > 0 && rotated <= 0 ? 1 : 0;
    // if (num > 0)
    // {
    //     output += (prev + num) / 100;
    // }
    // if (num < 0)
    // {
    //     output += ((prev + num) / 100) * -1;
    // }
    Console.WriteLine($"output {output}");
    Console.WriteLine($"curr {curr}");
}

Console.WriteLine(output);
