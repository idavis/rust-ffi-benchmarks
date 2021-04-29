
long __attribute__((noinline)) add_one(long x)
{
    return x + 1;
}

long __attribute__((noinline)) get_one()
{
    return 1;
}

long __attribute__((noinline)) calc_sum(long iterations)
{
    long sum = 0;
    for (long i = 0; i < iterations; i++)
    {
        sum = add_one(sum);
    }
    return sum;
}

long __attribute__((noinline)) calc_sum_opt(long iterations)
{
    long sum = 0;
    for (long i = 0; i < iterations; i++)
    {
        sum++;
    }
    return sum;
}