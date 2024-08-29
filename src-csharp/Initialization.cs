using TauriSharp.Utils;

namespace TauriSharp;

internal class Initialization {
    public const string HOST = "[::1]";
    public const string PORT = "5030";

    public static ProgramArgs GetArgs(string[] args) {
        string portArgs = args.ElementAtOrDefault(0) ?? PORT;
        string host = args.ElementAtOrDefault(1) ?? HOST;
        
        bool parsePort = int.TryParse(portArgs, out int port);
        if(!parsePort) {
            throw new Exception("Invalid port number");
        }
        return new ProgramArgs(port, host);
    }
}