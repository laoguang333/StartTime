namespace WinFormDemo;

public partial class Form1 : Form
{
    private readonly long _startTimeMs;

    public Form1(long startTimeMs)
    {
        _startTimeMs = startTimeMs > 0 ? startTimeMs : DateTimeOffset.UtcNow.ToUnixTimeMilliseconds();
        InitializeComponent();
    }

    protected override void OnShown(EventArgs e)
    {
        base.OnShown(e);
        var elapsed = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds() - _startTimeMs;
        Text = $"WinForm Demo - Startup: {elapsed} ms";
        labelStartupTime.Text = $"Startup Time: {elapsed} ms";
    }
}
