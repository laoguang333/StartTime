namespace WinFormDemo;

static class Program
{
    [STAThread]
    static void Main(string[] args)
    {
        long startTimeMs = 0;
        for (int i = 0; i < args.Length - 1; i++)
        {
            if (args[i] == "--start-time" && long.TryParse(args[i + 1], out var val))
            {
                startTimeMs = val;
                break;
            }
        }

        ApplicationConfiguration.Initialize();
        Application.Run(new Form1(startTimeMs));
    }
}
