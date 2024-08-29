namespace TauriSharp.Utils;

public record ProgramArgs(int port, string host) {
    public override string ToString()
    {
        return $"http://{host}:{port}";
    }
}