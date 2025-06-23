using Dapper;
using Npgsql;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.

var app = builder.Build();

// Configure the HTTP request pipeline.
bool IsPrimeNaive(int number)
{
    if (number <= 1) return false;
    if (number == 2) return true;
    if (number % 2 == 0) return false;

    for (int i = 3; i < number; i+=2)
    {
        if (number % i == 0)
        {
            return false;
        }
    }
    return true;
}


app.MapGet("/api/cars/{count}", async (int count) =>
{
    using var connection = new NpgsqlConnection("Host=<ip-database-container>;Port=5432;Database=cars;Username=cars;Password=hellopwd;SSL Mode=Require;Trust Server Certificate=true");
    return await connection.QueryAsync<Car>("select * from car order by RANDOM() limit @Limit", new { Limit = count });
});

app.MapGet("/api/prime/{n}", async (int n) =>
{
    return await Task.Run(() =>
    {
        int count = 1;
        int currentPrime = 2;
        int currentNum = 2;
        while (count < n)
        {
            ++currentNum;
            if (IsPrimeNaive(currentNum))
            {
                currentPrime = currentNum;
                ++count;
            }
        }
        return currentPrime;
    });
});

app.Run();


public class Car
{
    public Int64 id { get; set; }
    public int year { get; set; }
    public string make { get; set; }
    public string model { get; set; }
}
